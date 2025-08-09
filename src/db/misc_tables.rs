/// Creates the Tasks table for managing tasks associated with work orders, cleaning, and maintenance requests.
pub async fn create_task_tables(
    tables: &ListTablesOutput,
    client: &Client
) -> Result<(), AppError> {
    let table_name = "Tasks";

    if tables.table_names().contains(&table_name.to_string()) {
        println!("Table '{}' already exists", table_name);
        return Ok(());
    }

    // Define attribute definitions
    let ad_id = build(
        AttributeDefinition::builder()
            .attribute_name("id")
            .attribute_type(ScalarAttributeType::S)
            .build(),
        "Failed to build id attribute definition"
    )?;

    let ad_task_number = build(
        AttributeDefinition::builder()
            .attribute_name("task_number")
            .attribute_type(ScalarAttributeType::S)
            .build(),
        "Failed to build task_number attribute definition"
    )?;

    let ad_work_order_id = build(
        AttributeDefinition::builder()
            .attribute_name("work_order_id")
            .attribute_type(ScalarAttributeType::S)
            .build(),
        "Failed to build work_order_id attribute definition"
    )?;

    let ad_task_type = build(
        AttributeDefinition::builder()
            .attribute_name("task_type")
            .attribute_type(ScalarAttributeType::S)
            .build(),
        "Failed to build task_type attribute definition"
    )?;

    let ad_assigned_to = build(
        AttributeDefinition::builder()
            .attribute_name("assigned_to")
            .attribute_type(ScalarAttributeType::S)
            .build(),
        "Failed to build assigned_to attribute definition"
    )?;

    let ad_completed = build(
        AttributeDefinition::builder()
            .attribute_name("completed")
            .attribute_type(ScalarAttributeType::B)
            .build(),
        "Failed to build completed attribute definition"
    )?;

    let ad_private = build(
        AttributeDefinition::builder()
            .attribute_name("private")
            .attribute_type(ScalarAttributeType::B)
            .build(),
        "Failed to build private attribute definition"
    )?;

    let ad_created_at = build(
        AttributeDefinition::builder()
            .attribute_name("created_at")
            .attribute_type(ScalarAttributeType::S)
            .build(),
        "Failed to build created_at attribute definition"
    )?;

    let ad_updated_at = build(
        AttributeDefinition::builder()
            .attribute_name("updated_at")
            .attribute_type(ScalarAttributeType::S)
            .build(),
        "Failed to build updated_at attribute definition"
    )?;

    // Define key schema
    let ks_id = build(
        KeySchemaElement::builder().attribute_name("id").key_type(KeyType::Hash).build(),
        "Failed to build id key schema"
    )?;

    // Create the table
    let response = client
        .create_table()
        .table_name(table_name)
        .billing_mode(BillingMode::PayPerRequest)
        .attribute_definitions(ad_id)
        .attribute_definitions(ad_task_number)
        .attribute_definitions(ad_work_order_id)
        .attribute_definitions(ad_task_type)
        .attribute_definitions(ad_assigned_to)
        .attribute_definitions(ad_completed)
        .attribute_definitions(ad_private)
        .attribute_definitions(ad_created_at)
        .attribute_definitions(ad_updated_at)
        .key_schema(ks_id)
        .send().await
        .map_err(|e|
            AppError::DatabaseError(
                format!("Failed to create {} table: {:?}", table_name, e.to_string())
            )
        )?;

    println!("Tasks table created: {:?}", response);
    Ok(())
}
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
