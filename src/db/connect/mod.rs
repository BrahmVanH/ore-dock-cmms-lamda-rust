use aws_config::{ meta::region::RegionProviderChain, BehaviorVersion };
use aws_sdk_dynamodb::Client;
use dotenvy::dotenv;
use tracing::info;
use std::env;

use crate::error::AppError;

pub async fn setup_local_client() -> Result<Client, AppError> {
    dotenv().ok();
    let region_provider = RegionProviderChain::default_provider().or_else("us-east-2");
    info!("db region provider value: {:?}", &region_provider);

    let config = aws_config
        ::from_env()
        .behavior_version(BehaviorVersion::v2025_01_17())
        .region(region_provider)
        .load().await;

    info!("db config value: {:?}", &config);

    // Load DB_URL from ENV
    let db_url = match env::var("DB_URL") {
        Ok(env) => env,
        Err(e) => {
            eprintln!("Failed to get DB_URL from env");
            return Err(AppError::InternalServerError(e.to_string()));
        }
    };

    // Override the endpoint URL from config envs to point to local DB instance
    let dynamo_config = aws_sdk_dynamodb::config::Builder
        ::from(&config)
        .endpoint_url(db_url)
        .build();

    Ok(Client::from_conf(dynamo_config))
}
