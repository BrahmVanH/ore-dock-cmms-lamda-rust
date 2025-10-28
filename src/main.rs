use std::env;

use aws_config::Region;
use axum::{ extract::Extension, http::{ HeaderValue, Method }, routing::get, Router };
use dotenvy::dotenv;
use ore_dock_cmms_lambda::{
    config::Config,
    context::{ AppContext, ContextExtensions },
    create_schema,
    db,
    s3::connect::setup_aws_s3_client,
    DbClient,
    GraphQLSchema,
    S3Client,
};
use tower::ServiceBuilder;
use tower_http::{ compression::CompressionLayer, cors::{ Any, CorsLayer } };
use async_graphql_axum::{ GraphQLBatchRequest, GraphQLRequest, GraphQLResponse };
use serde::Serialize;
use tracing::{ info, error };

mod auth;

// Success/Failure response structs (if still needed)
#[derive(Debug, Serialize)]
struct SuccessResponse {
    pub body: String,
}

#[derive(Debug, Serialize)]
struct FailureResponse {
    pub body: String,
}

impl std::fmt::Display for FailureResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.body)
    }
}

impl std::error::Error for FailureResponse {}

// Handler for GraphQL requests
async fn graphql_handler(
    Extension(schema): Extension<GraphQLSchema>,
    req: GraphQLBatchRequest
) -> GraphQLResponse {
    schema.execute_batch(req.into_inner()).await.into()
}

// Handler for GraphQL playground
async fn graphql_playground() -> impl axum::response::IntoResponse {
    axum::response::Html(async_graphql::http::GraphiQLSource::build().endpoint("/graphql").finish())
}

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber
        ::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .with_thread_ids(true)
        .with_line_number(true)
        .with_file(true)
        .init();

    info!("Starting up Ore Dock CMMS Lambda service");

    // Load configuration
    let db_config = Config::from_env().unwrap_or_else(|e| {
        error!("Failed to load configuration, using defaults: {}", e);
        Config::default()
    });

    info!("Configuration loaded: {:?}", db_config);

    // Create database client
    let db_client = match setup_database_client(&db_config).await {
        Ok(client) => client,
        Err(e) => {
            error!("Fatal error creating database client: {}", e);
            std::process::exit(1);
        }
    };

    let s3_client: S3Client = match setup_aws_s3_client().await {
        Ok(client) => client,
        Err(e) => {
            error!("Fatal error creating aws s3 client: {}", e);
            std::process::exit(1);
        }
    };

    // Ensure all tables exist
    if let Err(e) = db::init::ensure_tables_exist(&db_client).await {
        error!("Fatal error ensuring tables exist: {}", e);
        std::process::exit(1);
    }

    info!("Database tables verified/created successfully");

    // Create application context
    let app_context = AppContext::new(db_client.clone(), db_config.clone(), s3_client.clone());

    // Create GraphQL schema with all necessary data
    let schema = create_schema()
        .data(db_client.clone()) // For backward compatibility with existing resolvers
        .data(db_config.clone())
        .data(s3_client.clone())
        .data(app_context)
        .finish();

    info!("GraphQL schema created successfully");

    // Configure CORS based on environment
    let cors = if db_config.graphql.playground {
        // Development mode - allow all origins
        CorsLayer::new()
            .allow_origin(Any)
            .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
            .allow_headers(Any)
    } else {
        // Production mode - restrict origins (you'd configure this based on your needs)
        let allowed_origins: Vec<String> = db_config
            .clone()
            .allow_origins.split(",")
            .map(|s| s.to_string())
            .collect();

        let mut cors_layer = CorsLayer::new()
            .allow_methods([Method::GET, Method::POST])
            .allow_headers(Any);

        for origin in &allowed_origins {
            let header_value = origin
                .parse::<HeaderValue>()
                .map_err(|e| {
                    error!("Could not configure cors layer: {:?}", e);
                    std::process::exit(1);
                })
                .unwrap();
            cors_layer = cors_layer.allow_origin(header_value);
        }

        cors_layer
    };

    // Build router
    let mut router = Router::new();

    // Add GraphQL endpoint
    router = router
        .route("/graphql", get(graphql_playground))
        .route("/graphql", axum::routing::post(graphql_handler));

    // Add health check endpoint
    router = router.route("/health", get(health_check));

    // Add middleware layers
    let app = router.layer(
        ServiceBuilder::new()
            .layer(CompressionLayer::new().gzip(true).deflate(true).br(true))
            .layer(Extension(db_client))
            .layer(Extension(schema))
            .layer(Extension(db_config.clone()))
            .layer(cors)
    );

    // Determine port from environment or use default
    let port = std::env
        ::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()
        .unwrap_or(3000);

    let bind_address = format!("0.0.0.0:{}", port);

    // Start server
    let listener = match tokio::net::TcpListener::bind(&bind_address).await {
        Ok(l) => l,
        Err(e) => {
            error!("Fatal error binding to {}: {}", bind_address, e);
            std::process::exit(1);
        }
    };

    info!("Server running on http://localhost:{}", port);
    info!("GraphQL Playground available at http://localhost:{}/graphql", port);

    if let Err(e) = axum::serve(listener, app).await {
        error!("Fatal error running server: {}", e);
        std::process::exit(1);
    }
}

// Setup database client based on configuration
async fn setup_database_client(config: &Config) -> Result<DbClient, Box<dyn std::error::Error>> {
    if let Some(endpoint) = &config.database.endpoint {
        // Local DynamoDB setup
        info!("Setting up local DynamoDB client with endpoint: {}", endpoint);
        setup_local_client_with_config(&config).await
    } else {
        // AWS DynamoDB setup
        info!("Setting up AWS DynamoDB client for region: {}", config.database.region);
        setup_aws_client_with_config(&config).await
    }
}

// Setup local DynamoDB client using configuration
async fn setup_local_client_with_config(
    config: &Config
) -> Result<DbClient, Box<dyn std::error::Error>> {
    use aws_config::{ meta::region::RegionProviderChain, BehaviorVersion };
    use dotenvy::dotenv;

    dotenv().ok();
    let default_region = config.database.region.to_string();
    let region_provider = RegionProviderChain::default_provider().or_else(
        Region::new(default_region)
    );

    let aws_config = aws_config
        ::from_env()
        .behavior_version(BehaviorVersion::v2025_08_07())
        .region(region_provider)
        .load().await;

    // Use endpoint from config or fall back to DB_URL environment variable
    let endpoint = config.database.endpoint
        .clone()
        .or_else(|| std::env::var("DB_URL").ok())
        .ok_or("No local DynamoDB endpoint configured")?;

    let dynamo_config = aws_sdk_dynamodb::config::Builder
        ::from(&aws_config)
        .endpoint_url(endpoint)
        .build();

    Ok(aws_sdk_dynamodb::Client::from_conf(dynamo_config))
}

// Setup AWS DynamoDB client using configuration
async fn setup_aws_client_with_config(
    config: &Config
) -> Result<DbClient, Box<dyn std::error::Error>> {
    use aws_config::{ meta::region::RegionProviderChain, BehaviorVersion };

    let default_region = config.database.region.to_string();
    let region_provider = RegionProviderChain::default_provider().or_else(
        Region::new(default_region)
    );

    let mut aws_config_builder = aws_config
        ::from_env()
        .behavior_version(BehaviorVersion::v2025_08_07())
        .region(region_provider);

    // Override credentials if provided in config
    if
        let (Some(access_key), Some(secret_key)) = (
            &config.aws.access_key_id,
            &config.aws.secret_access_key,
        )
    {
        use aws_credential_types::Credentials;
        let credentials = Credentials::new(access_key, secret_key, None, None, "config");
        aws_config_builder = aws_config_builder.credentials_provider(credentials);
    }

    let aws_config = aws_config_builder.load().await;
    Ok(aws_sdk_dynamodb::Client::new(&aws_config))
}

// Health check endpoint
async fn health_check() -> &'static str {
    "OK"
}
