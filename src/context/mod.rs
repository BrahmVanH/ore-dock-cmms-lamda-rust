use async_graphql::Context;
use aws_sdk_dynamodb::Client;
use aws_sdk_s3::Client as S3Client;
use std::sync::Arc;

use crate::{ config::Config, AppError };

#[derive(Clone)]
pub struct AppContext {
    pub db_client: Arc<Client>,
    pub s3_client: Arc<S3Client>,
    pub config: Arc<Config>,
}

impl AppContext {
    pub fn new(db_client: Client, config: Config, s3_client: S3Client) -> Self {
        Self {
            db_client: Arc::new(db_client),
            config: Arc::new(config),
            s3_client: Arc::new(s3_client),
        }
    }
}

// Extension trait for GraphQL Context
pub trait ContextExtensions {
    fn db_client(&self) -> Result<&Client, AppError>;
    fn config(&self) -> Result<&Config, AppError>;
    fn s3_client(&self) -> Result<&S3Client, AppError>;
}

impl<'a> ContextExtensions for Context<'a> {
    fn db_client(&self) -> Result<&Client, AppError> {
        self.data::<Client>().map_err(|_| {
            AppError::InternalServerError("Database client not available in context".to_string())
        })
    }

    fn config(&self) -> Result<&Config, AppError> {
        self.data::<Config>().map_err(|_| {
            AppError::InternalServerError("Config not available in context".to_string())
        })
    }

    fn s3_client(&self) -> Result<&S3Client, AppError> {
        self.data::<S3Client>().map_err(|_| {
            AppError::InternalServerError("AWS S3 client not available in context".to_string())
        })
    }
}
