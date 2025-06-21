//! DynamoDB table initialization and management module.
//!
//! This module is responsible for ensuring all required DynamoDB tables
//! exist with the correct configuration before the application starts.
//! It uses the modular table creation approach for better organization
//! and maintainability.

use aws_sdk_dynamodb::Client;
use crate::error::AppError;
use super::ensure_table_exists;

pub async fn ensure_tables_exist(client: &Client) -> Result<(), AppError> {
    // Use the new modular table creation orchestration
    ensure_table_exists::ensure_all_tables_exist(client).await
}
