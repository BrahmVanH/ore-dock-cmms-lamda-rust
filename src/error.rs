use async_graphql::{Error as GraphQLError, ErrorExtensions};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    DatabaseError(String),
    
    #[error("Validation error: {0}")]
    ValidationError(String),
    
    #[error("Not found: {0}")]
    NotFound(String),
    
    #[error("Unauthorized: {0}")]
    Unauthorized(String),
    
    #[error("Forbidden: {0}")]
    Forbidden(String),
    
    #[error("Internal server error: {0}")]
    InternalServerError(String),
    
    #[error("Configuration error: {0}")]
    ConfigError(String),
    
    #[error("Authentication error: {0}")]
    AuthError(String),
}

impl ErrorExtensions for AppError {
    fn extend(&self) -> GraphQLError {
        GraphQLError::new(format!("{}", self)).extend_with(|_err, e| {
            match self {
                AppError::DatabaseError(_) => e.set("code", "DATABASE_ERROR"),
                AppError::ValidationError(_) => e.set("code", "VALIDATION_ERROR"),
                AppError::NotFound(_) => e.set("code", "NOT_FOUND"),
                AppError::Unauthorized(_) => e.set("code", "UNAUTHORIZED"),
                AppError::Forbidden(_) => e.set("code", "FORBIDDEN"),
                AppError::InternalServerError(_) => e.set("code", "INTERNAL_SERVER_ERROR"),
                AppError::ConfigError(_) => e.set("code", "CONFIG_ERROR"),
                AppError::AuthError(_) => e.set("code", "AUTH_ERROR"),
            }
        })
    }
}

impl AppError {
    pub fn to_graphql_error(self) -> GraphQLError {
        self.extend()
    }
}

pub type AppResult<T> = Result<T, AppError>;