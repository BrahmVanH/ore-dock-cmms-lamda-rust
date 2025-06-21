//! Miscellaneous system tables.
//!
//! This module contains table definitions for user preferences and other
//! miscellaneous system functionality.

use aws_sdk_dynamodb::{
    Client,
    operation::list_tables::ListTablesOutput,
    types::{ AttributeDefinition, BillingMode, KeySchemaElement, KeyType, ScalarAttributeType },
};

use crate::error::AppError;
use super::common::build;

/// Creates the UserPreferences table.
pub async fn create_user_preferences_table(
    tables: &ListTablesOutput,
    client: &Client
) -> Result<(), AppError> {
    let table_name = "UserPreferences";

    if tables.table_names().contains(&table_name.to_string()) {
        println!("Table '{}' already exists", table_name);
        return Ok(());
    }

    // Define attribute definitions
    let ad_user_id = build(
        AttributeDefinition::builder()
            .attribute_name("user_id")
            .attribute_type(ScalarAttributeType::S)
            .build(),
        "Failed to build user_id attribute definition"
    )?;

    // Define key schema
    let ks_user_id = build(
        KeySchemaElement::builder().attribute_name("user_id").key_type(KeyType::Hash).build(),
        "Failed to build user_id key schema"
    )?;

    // Create the table
    let response = client
        .create_table()
        .table_name("UserPreferences")
        .billing_mode(BillingMode::PayPerRequest)
        .attribute_definitions(ad_user_id)
        .key_schema(ks_user_id)
        .send().await
        .map_err(|e|
            AppError::DatabaseError(
                format!("Failed to create {} table: {:?}", table_name, e.to_string())
            )
        )?;

    println!("UserPreferences table created: {:?}", response);
    Ok(())
}
