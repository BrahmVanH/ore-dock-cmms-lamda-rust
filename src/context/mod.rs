use async_graphql::Context;
use aws_sdk_dynamodb::Client;
use std::sync::Arc;

use crate::{config::Config, AppError};

#[derive(Clone)]
pub struct AppContext {
    pub db_client: Arc<Client>,
    pub config: Arc<Config>,
}

impl AppContext {
    pub fn new(db_client: Client, config: Config) -> Self {
        Self {
            db_client: Arc::new(db_client),
            config: Arc::new(config),
        }
    }
}

// Extension trait for GraphQL Context
pub trait ContextExtensions {
    fn db_client(&self) -> Result<&Client, AppError>;
    fn config(&self) -> Result<&Config, AppError>;
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
}