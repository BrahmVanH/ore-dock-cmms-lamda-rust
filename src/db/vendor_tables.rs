//! Vendor and supply chain management table definitions.
//!
//! This module contains table definitions for vendors, vendor categories,
//! and related supply chain functionality.

use aws_sdk_dynamodb::{
    Client,
    operation::list_tables::ListTablesOutput,
    types::{
        AttributeDefinition,
        BillingMode,
        KeySchemaElement,
        KeyType,
        GlobalSecondaryIndex,
        Projection,
        ProjectionType,
        ScalarAttributeType,
    },
};

use crate::error::AppError;
use super::common::build;

/// Creates the Vendors table.
pub async fn create_vendors_table(
    tables: &ListTablesOutput,
    client: &Client
) -> Result<(), AppError> {
    let table_name = "Vendors";

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

    let ad_category_id = build(
        AttributeDefinition::builder()
            .attribute_name("category_id")
            .attribute_type(ScalarAttributeType::S)
            .build(),
        "Failed to build category_id attribute definition"
    )?;

    let ad_status = build(
        AttributeDefinition::builder()
            .attribute_name("status")
            .attribute_type(ScalarAttributeType::S)
            .build(),
        "Failed to build status attribute definition"
    )?;

    let ad_rating = build(
        AttributeDefinition::builder()
            .attribute_name("rating")
            .attribute_type(ScalarAttributeType::N)
            .build(),
        "Failed to build rating attribute definition"
    )?;

    // Define key schema
    let ks_id = build(
        KeySchemaElement::builder().attribute_name("id").key_type(KeyType::Hash).build(),
        "Failed to build id key schema"
    )?;

    // Define GSI 1: Category Index
    let gsi1_pk = build(
        KeySchemaElement::builder().attribute_name("category_id").key_type(KeyType::Hash).build(),
        "Failed to build Category GSI PK"
    )?;

    let gsi1 = build(
        GlobalSecondaryIndex::builder()
            .index_name("CategoryIndex")
            .key_schema(gsi1_pk)
            .projection(Projection::builder().projection_type(ProjectionType::All).build())
            .build(),
        "Failed to build CategoryIndex GSI"
    )?;

    // Define GSI 2: Status Index
    let gsi2_pk = build(
        KeySchemaElement::builder().attribute_name("status").key_type(KeyType::Hash).build(),
        "Failed to build Status GSI PK"
    )?;

    let gsi2 = build(
        GlobalSecondaryIndex::builder()
            .index_name("StatusIndex")
            .key_schema(gsi2_pk)
            .projection(Projection::builder().projection_type(ProjectionType::All).build())
            .build(),
        "Failed to build StatusIndex GSI"
    )?;

    // Define GSI 3: Rating Index
    let gsi3_pk = build(
        KeySchemaElement::builder().attribute_name("rating").key_type(KeyType::Hash).build(),
        "Failed to build Rating GSI PK"
    )?;

    let gsi3 = build(
        GlobalSecondaryIndex::builder()
            .index_name("RatingIndex")
            .key_schema(gsi3_pk)
            .projection(Projection::builder().projection_type(ProjectionType::All).build())
            .build(),
        "Failed to build RatingIndex GSI"
    )?;

    // Create the table
    let response = client
        .create_table()
        .table_name("Vendors")
        .billing_mode(BillingMode::PayPerRequest)
        .attribute_definitions(ad_id)
        .attribute_definitions(ad_category_id)
        .attribute_definitions(ad_status)
        .attribute_definitions(ad_rating)
        .key_schema(ks_id)
        .global_secondary_indexes(gsi1)
        .global_secondary_indexes(gsi2)
        .global_secondary_indexes(gsi3)
        .send().await
        .map_err(|e|
            AppError::DatabaseError(
                format!("Failed to create {} table: {:?}", table_name, e.to_string())
            )
        )?;

    println!("Vendors table created: {:?}", response);
    Ok(())
}

/// Creates the VendorCategories table.
pub async fn create_vendor_categories_table(
    tables: &ListTablesOutput,
    client: &Client
) -> Result<(), AppError> {
    let table_name = "VendorCategories";

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

    // Define key schema
    let ks_id = build(
        KeySchemaElement::builder().attribute_name("id").key_type(KeyType::Hash).build(),
        "Failed to build id key schema"
    )?;

    // Create the table
    let response = client
        .create_table()
        .table_name("VendorCategories")
        .billing_mode(BillingMode::PayPerRequest)
        .attribute_definitions(ad_id)
        .key_schema(ks_id)
        .send().await
        .map_err(|e|
            AppError::DatabaseError(
                format!("Failed to create {} table: {:?}", table_name, e.to_string())
            )
        )?;

    println!("VendorCategories table created: {:?}", response);
    Ok(())
}
