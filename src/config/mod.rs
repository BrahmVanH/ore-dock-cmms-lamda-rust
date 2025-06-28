use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub database: DatabaseConfig,
    pub graphql: GraphQLConfig,
    pub auth: AuthConfig,
    pub aws: AwsConfig,
    pub environment: String,
    pub allow_origins: String,
    pub log_level: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    pub region: String,
    pub endpoint: Option<String>, // For local DynamoDB
}

#[derive(Debug, Clone, Deserialize)]
pub struct GraphQLConfig {
    pub playground: bool,
    pub introspection: bool,
    pub complexity_limit: Option<usize>,
    pub depth_limit: Option<usize>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AuthConfig {
    pub jwt_secret: String,
    pub token_expiry: u64, // seconds
}

#[derive(Debug, Clone, Deserialize)]
pub struct AwsConfig {
    pub region: String,
    pub access_key_id: Option<String>,
    pub secret_access_key: Option<String>,
}

impl Config {
    pub fn from_env() -> Result<Self, crate::AppError> {
        envy::from_env().map_err(|e| {
            crate::AppError::ConfigError(format!("Failed to load config from environment: {}", e))
        })
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            database: DatabaseConfig {
                region: "us-east-2".to_string(),
                endpoint: Some("http://localhost:8000".to_string()),
            },
            graphql: GraphQLConfig {
                playground: true,
                introspection: true,
                complexity_limit: Some(1000),
                depth_limit: Some(10),
            },
            auth: AuthConfig {
                jwt_secret: "default-secret-change-in-production".to_string(),
                token_expiry: 3600, // 1 hour
            },
            aws: AwsConfig {
                region: "us-east-2".to_string(),
                access_key_id: None,
                secret_access_key: None,
            },
            environment: "dev".to_string(),
            allow_origins: "".to_string(),
            log_level: "error".to_string(),
        }
    }
}
