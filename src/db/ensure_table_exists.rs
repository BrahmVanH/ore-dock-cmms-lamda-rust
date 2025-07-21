//! Main table creation orchestration module.
//!
//! This module provides the main entry point for ensuring all database tables exist.
//! It imports and calls the modular table creation functions from the specialized modules.

use aws_sdk_dynamodb::Client;
use crate::error::AppError;

use super::{
    user_tables,
    asset_tables,
    notification_tables,
    security_tables,
    vendor_tables,
    misc_tables,
};

/// Main function to ensure all required DynamoDB tables exist.
///
/// This function orchestrates the creation of all tables across different functional areas
/// by calling the specialized table creation functions from their respective modules.
///
/// # Arguments
///
/// * `client` - DynamoDB client for AWS API operations
///
/// # Returns
///
/// * `Result<(), AppError>` - Success or a database error with context
pub async fn ensure_all_tables_exist(client: &Client) -> Result<(), AppError> {
    // Get all existing tables once to avoid multiple API calls
    let tables = client
        .list_tables()
        .send().await
        .map_err(|e|
            AppError::DatabaseError(
                format!("Failed to retrieve tables list from db client: {:?}", e.to_string())
            )
        )?;

    println!("Starting table creation process...");

    // Create user and access management tables
    println!("Creating user and access management tables...");
    user_tables::create_pantry_system_table(&tables, client).await?;
    user_tables::create_users_table(&tables, client).await?;
    user_tables::create_pantry_access_table(&tables, client).await?;

    // Create asset management tables
    println!("Creating asset management tables...");
    asset_tables::create_assets_table(&tables, client).await?;
    asset_tables::create_asset_types_table(&tables, client).await?;
    asset_tables::create_locations_table(&tables, client).await?;
    asset_tables::create_location_types_table(&tables, client).await?;
    asset_tables::create_manufacturers_table(&tables, client).await?;
    asset_tables::create_maintenance_schedules_table(&tables, client).await?;
    asset_tables::create_work_orders_table(&tables, client).await?;
    asset_tables::create_maintenance_requests_table(&tables, client).await?;

    // Create notification system tables
    println!("Creating notification system tables...");
    notification_tables::create_notifications_table(&tables, client).await?;
    notification_tables::create_notification_templates_table(&tables, client).await?;
    notification_tables::create_notification_delivery_logs_table(&tables, client).await?;
    notification_tables::create_user_notification_preferences_table(&tables, client).await?;

    // Create security and permissions tables
    println!("Creating security and permissions tables...");
    security_tables::create_roles_table(&tables, client).await?;
    security_tables::create_user_roles_table(&tables, client).await?;
    security_tables::create_permissions_table(&tables, client).await?;
    security_tables::create_permission_logs_table(&tables, client).await?;
    security_tables::create_role_hierarchy_table(&tables, client).await?;
    security_tables::create_temp_role_elevation_table(&tables, client).await?;

    // Create vendor management tables
    println!("Creating vendor management tables...");
    vendor_tables::create_vendors_table(&tables, client).await?;
    vendor_tables::create_vendor_categories_table(&tables, client).await?;

    // Create miscellaneous tables
    println!("Creating miscellaneous system tables...");
    misc_tables::create_user_preferences_table(&tables, client).await?;

    println!("All tables created successfully!");
    Ok(())
}
