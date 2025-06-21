//! DynamoDB table initialization and management module.
//!
//! This module is responsible for ensuring all required DynamoDB tables
//! exist with the correct configuration before the application starts.
//! It calls functions to check for table existence and create tables
//! with appropriate indexes and configuration when needed.

use aws_sdk_dynamodb::{ Client, Error };

use crate::error::AppError;

use super::ensure_table_exists;

pub async fn ensure_tables_exist(client: &Client) -> Result<(), AppError> {
    // Get all existing tables
    let tables = client
        .list_tables()
        .send().await
        .map_err(|e|
            AppError::DatabaseError(
                format!("Failed to retrieve tables list from db client: {:?}", e.to_string())
            )
        )?;

    // Check and create individual tables as needed
    ensure_table_exists::pantry_system(&tables, client).await?;
    ensure_table_exists::users(&tables, client).await?;
    ensure_table_exists::pantries(&tables, client).await?;
    ensure_table_exists::pantry_access(&tables, client).await?;

    // Additional tables can be added here in the future

    Ok(())
}
