use aws_config::{ meta::region::RegionProviderChain, BehaviorVersion };
use dotenvy::dotenv;
use tracing::info;

use crate::S3Client;
use crate::error::AppError;

pub async fn setup_aws_s3_client() -> Result<S3Client, AppError> {
    dotenv().ok();

    let region_provider = RegionProviderChain::default_provider().or_else("us-east-2");
    // info!("db region provider value: {:?}", &region_provider);

    let config = aws_config
        ::from_env()
        .behavior_version(BehaviorVersion::v2025_08_07())
        .region(region_provider)
        .load().await;

    // info!("s3 config value: {:?}", &config);

    Ok(S3Client::new(&config))
}
