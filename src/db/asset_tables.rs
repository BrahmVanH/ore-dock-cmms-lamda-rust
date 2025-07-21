//! Asset management table definitions.
//!
//! This module contains table definitions for asset management,
//! maintenance scheduling, work orders, and related functionality.

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

/// Creates the Assets table for managing physical assets.
///
/// # Primary Key Structure
/// * Partition Key: id (Asset UUID)
///
/// # Global Secondary Indexes
/// * TypeIndex: Find assets by type
/// * LocationIndex: Find assets by location
/// * ManufacturerIndex: Find assets by manufacturer
/// * StatusIndex: Find assets by current status
/// * MaintenanceFrequencyIndex: Find assets by maintenance frequency
pub async fn create_assets_table(
    tables: &ListTablesOutput,
    client: &Client
) -> Result<(), AppError> {
    let table_name = "Assets";

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

    let ad_type_id = build(
        AttributeDefinition::builder()
            .attribute_name("type_id")
            .attribute_type(ScalarAttributeType::S)
            .build(),
        "Failed to build type_id attribute definition"
    )?;

    let ad_location_id = build(
        AttributeDefinition::builder()
            .attribute_name("location_id")
            .attribute_type(ScalarAttributeType::S)
            .build(),
        "Failed to build location_id attribute definition"
    )?;

    let ad_manufacturer_id = build(
        AttributeDefinition::builder()
            .attribute_name("manufacturer_id")
            .attribute_type(ScalarAttributeType::S)
            .build(),
        "Failed to build manufacturer_id attribute definition"
    )?;

    let ad_current_status = build(
        AttributeDefinition::builder()
            .attribute_name("current_status")
            .attribute_type(ScalarAttributeType::S)
            .build(),
        "Failed to build current_status attribute definition"
    )?;

    let ad_maintenance_frequency = build(
        AttributeDefinition::builder()
            .attribute_name("maintenance_frequency")
            .attribute_type(ScalarAttributeType::S)
            .build(),
        "Failed to build maintenance_frequency attribute definition"
    )?;

    // Define key schema
    let ks_id = build(
        KeySchemaElement::builder().attribute_name("id").key_type(KeyType::Hash).build(),
        "Failed to build id key schema"
    )?;

    // Define GSI 1: Type Index
    let gsi1_pk = build(
        KeySchemaElement::builder().attribute_name("type_id").key_type(KeyType::Hash).build(),
        "Failed to build Type GSI PK"
    )?;

    let gsi1 = build(
        GlobalSecondaryIndex::builder()
            .index_name("TypeIndex")
            .key_schema(gsi1_pk)
            .projection(Projection::builder().projection_type(ProjectionType::All).build())
            .build(),
        "Failed to build TypeIndex GSI"
    )?;

    // Define GSI 2: Location Index
    let gsi2_pk = build(
        KeySchemaElement::builder().attribute_name("location_id").key_type(KeyType::Hash).build(),
        "Failed to build Location GSI PK"
    )?;

    let gsi2 = build(
        GlobalSecondaryIndex::builder()
            .index_name("LocationIndex")
            .key_schema(gsi2_pk)
            .projection(Projection::builder().projection_type(ProjectionType::All).build())
            .build(),
        "Failed to build LocationIndex GSI"
    )?;

    // Define GSI 3: Manufacturer Index
    let gsi3_pk = build(
        KeySchemaElement::builder()
            .attribute_name("manufacturer_id")
            .key_type(KeyType::Hash)
            .build(),
        "Failed to build Manufacturer GSI PK"
    )?;

    let gsi3 = build(
        GlobalSecondaryIndex::builder()
            .index_name("ManufacturerIndex")
            .key_schema(gsi3_pk)
            .projection(Projection::builder().projection_type(ProjectionType::All).build())
            .build(),
        "Failed to build ManufacturerIndex GSI"
    )?;

    // Define GSI 4: Status Index
    let gsi4_pk = build(
        KeySchemaElement::builder()
            .attribute_name("current_status")
            .key_type(KeyType::Hash)
            .build(),
        "Failed to build Status GSI PK"
    )?;

    let gsi4 = build(
        GlobalSecondaryIndex::builder()
            .index_name("StatusIndex")
            .key_schema(gsi4_pk)
            .projection(Projection::builder().projection_type(ProjectionType::All).build())
            .build(),
        "Failed to build StatusIndex GSI"
    )?;

    // Define GSI 5: Maintenance Frequency Index
    let gsi5_pk = build(
        KeySchemaElement::builder()
            .attribute_name("maintenance_frequency")
            .key_type(KeyType::Hash)
            .build(),
        "Failed to build MaintenanceFrequency GSI PK"
    )?;

    let gsi5 = build(
        GlobalSecondaryIndex::builder()
            .index_name("MaintenanceFrequencyIndex")
            .key_schema(gsi5_pk)
            .projection(Projection::builder().projection_type(ProjectionType::All).build())
            .build(),
        "Failed to build MaintenanceFrequencyIndex GSI"
    )?;

    // Create the table
    let response = client
        .create_table()
        .table_name("Assets")
        .billing_mode(BillingMode::PayPerRequest)
        .attribute_definitions(ad_id)
        .attribute_definitions(ad_type_id)
        .attribute_definitions(ad_location_id)
        .attribute_definitions(ad_manufacturer_id)
        .attribute_definitions(ad_current_status)
        .attribute_definitions(ad_maintenance_frequency)
        .key_schema(ks_id)
        .global_secondary_indexes(gsi1)
        .global_secondary_indexes(gsi2)
        .global_secondary_indexes(gsi3)
        .global_secondary_indexes(gsi4)
        .global_secondary_indexes(gsi5)
        .send().await
        .map_err(|e|
            AppError::DatabaseError(
                format!("Failed to create {} table: {:?}", table_name, e.to_string())
            )
        )?;

    println!("Assets table created: {:?}", response);
    Ok(())
}

/// Creates the AssetTypes table for categorizing assets.
pub async fn create_asset_types_table(
    tables: &ListTablesOutput,
    client: &Client
) -> Result<(), AppError> {
    let table_name = "AssetTypes";

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
        .table_name("AssetTypes")
        .billing_mode(BillingMode::PayPerRequest)
        .attribute_definitions(ad_id)
        .key_schema(ks_id)
        .send().await
        .map_err(|e|
            AppError::DatabaseError(
                format!("Failed to create {} table: {:?}", table_name, e.to_string())
            )
        )?;

    println!("AssetTypes table created: {:?}", response);
    Ok(())
}

/// Creates the Locations table for asset placement tracking.
pub async fn create_locations_table(
    tables: &ListTablesOutput,
    client: &Client
) -> Result<(), AppError> {
    let table_name = "Locations";

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

    let ad_location_type_id = build(
        AttributeDefinition::builder()
            .attribute_name("location_type_id")
            .attribute_type(ScalarAttributeType::S)
            .build(),
        "Failed to build location_type_id attribute definition"
    )?;

    // Define key schema
    let ks_id = build(
        KeySchemaElement::builder().attribute_name("id").key_type(KeyType::Hash).build(),
        "Failed to build id key schema"
    )?;

    // Define GSI 1: Location Type Index
    let gsi1_pk = build(
        KeySchemaElement::builder()
            .attribute_name("location_type_id")
            .key_type(KeyType::Hash)
            .build(),
        "Failed to build LocationType GSI PK"
    )?;

    let gsi1 = build(
        GlobalSecondaryIndex::builder()
            .index_name("LocationTypeIndex")
            .key_schema(gsi1_pk)
            .projection(Projection::builder().projection_type(ProjectionType::All).build())
            .build(),
        "Failed to build LocationTypeIndex GSI"
    )?;

    // Create the table
    let response = client
        .create_table()
        .table_name("Locations")
        .billing_mode(BillingMode::PayPerRequest)
        .attribute_definitions(ad_id)
        .attribute_definitions(ad_location_type_id)
        .key_schema(ks_id)
        .global_secondary_indexes(gsi1)
        .send().await
        .map_err(|e|
            AppError::DatabaseError(
                format!("Failed to create {} table: {:?}", table_name, e.to_string())
            )
        )?;

    println!("Locations table created: {:?}", response);
    Ok(())
}

/// Creates the LocationTypes table.
pub async fn create_location_types_table(
    tables: &ListTablesOutput,
    client: &Client
) -> Result<(), AppError> {
    let table_name = "LocationTypes";

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
        .table_name("LocationTypes")
        .billing_mode(BillingMode::PayPerRequest)
        .attribute_definitions(ad_id)
        .key_schema(ks_id)
        .send().await
        .map_err(|e|
            AppError::DatabaseError(
                format!("Failed to create {} table: {:?}", table_name, e.to_string())
            )
        )?;

    println!("LocationTypes table created: {:?}", response);
    Ok(())
}

/// Creates the Manufacturers table.
pub async fn create_manufacturers_table(
    tables: &ListTablesOutput,
    client: &Client
) -> Result<(), AppError> {
    let table_name = "Manufacturers";

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
        .table_name("Manufacturers")
        .billing_mode(BillingMode::PayPerRequest)
        .attribute_definitions(ad_id)
        .key_schema(ks_id)
        .send().await
        .map_err(|e|
            AppError::DatabaseError(
                format!("Failed to create {} table: {:?}", table_name, e.to_string())
            )
        )?;

    println!("Manufacturers table created: {:?}", response);
    Ok(())
}

/// Creates the MaintenanceSchedules table.
pub async fn create_maintenance_schedules_table(
    tables: &ListTablesOutput,
    client: &Client
) -> Result<(), AppError> {
    let table_name = "MaintenanceSchedules";

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

    let ad_asset_id = build(
        AttributeDefinition::builder()
            .attribute_name("asset_id")
            .attribute_type(ScalarAttributeType::S)
            .build(),
        "Failed to build asset_id attribute definition"
    )?;

    let ad_next_due_date = build(
        AttributeDefinition::builder()
            .attribute_name("next_due_date")
            .attribute_type(ScalarAttributeType::S)
            .build(),
        "Failed to build next_due_date attribute definition"
    )?;

    // Define key schema
    let ks_id = build(
        KeySchemaElement::builder().attribute_name("id").key_type(KeyType::Hash).build(),
        "Failed to build id key schema"
    )?;

    // Define GSI 1: Asset Index
    let gsi1_pk = build(
        KeySchemaElement::builder().attribute_name("asset_id").key_type(KeyType::Hash).build(),
        "Failed to build Asset GSI PK"
    )?;

    let gsi1 = build(
        GlobalSecondaryIndex::builder()
            .index_name("AssetIndex")
            .key_schema(gsi1_pk)
            .projection(Projection::builder().projection_type(ProjectionType::All).build())
            .build(),
        "Failed to build AssetIndex GSI"
    )?;

    // Define GSI 2: Due Date Index
    let gsi2_pk = build(
        KeySchemaElement::builder().attribute_name("next_due_date").key_type(KeyType::Hash).build(),
        "Failed to build DueDate GSI PK"
    )?;

    let gsi2 = build(
        GlobalSecondaryIndex::builder()
            .index_name("DueDateIndex")
            .key_schema(gsi2_pk)
            .projection(Projection::builder().projection_type(ProjectionType::All).build())
            .build(),
        "Failed to build DueDateIndex GSI"
    )?;

    // Create the table
    let response = client
        .create_table()
        .table_name("MaintenanceSchedules")
        .billing_mode(BillingMode::PayPerRequest)
        .attribute_definitions(ad_id)
        .attribute_definitions(ad_asset_id)
        .attribute_definitions(ad_next_due_date)
        .key_schema(ks_id)
        .global_secondary_indexes(gsi1)
        .global_secondary_indexes(gsi2)
        .send().await
        .map_err(|e|
            AppError::DatabaseError(
                format!("Failed to create {} table: {:?}", table_name, e.to_string())
            )
        )?;

    println!("MaintenanceSchedules table created: {:?}", response);
    Ok(())
}

/// Creates the WorkOrders table.
pub async fn create_work_orders_table(
    tables: &ListTablesOutput,
    client: &Client
) -> Result<(), AppError> {
    let table_name = "WorkOrders";

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

    let ad_asset_id = build(
        AttributeDefinition::builder()
            .attribute_name("asset_id")
            .attribute_type(ScalarAttributeType::S)
            .build(),
        "Failed to build asset_id attribute definition"
    )?;

    let ad_assigned_to = build(
        AttributeDefinition::builder()
            .attribute_name("assigned_to")
            .attribute_type(ScalarAttributeType::S)
            .build(),
        "Failed to build assigned_to attribute definition"
    )?;

    let ad_status = build(
        AttributeDefinition::builder()
            .attribute_name("status")
            .attribute_type(ScalarAttributeType::S)
            .build(),
        "Failed to build status attribute definition"
    )?;

    let ad_priority = build(
        AttributeDefinition::builder()
            .attribute_name("priority")
            .attribute_type(ScalarAttributeType::S)
            .build(),
        "Failed to build priority attribute definition"
    )?;

    // Define key schema
    let ks_id = build(
        KeySchemaElement::builder().attribute_name("id").key_type(KeyType::Hash).build(),
        "Failed to build id key schema"
    )?;

    // Define GSI 1: Asset Index
    let gsi1_pk = build(
        KeySchemaElement::builder().attribute_name("asset_id").key_type(KeyType::Hash).build(),
        "Failed to build Asset GSI PK"
    )?;

    let gsi1 = build(
        GlobalSecondaryIndex::builder()
            .index_name("AssetIndex")
            .key_schema(gsi1_pk)
            .projection(Projection::builder().projection_type(ProjectionType::All).build())
            .build(),
        "Failed to build AssetIndex GSI"
    )?;

    // Define GSI 2: Assigned To Index
    let gsi2_pk = build(
        KeySchemaElement::builder().attribute_name("assigned_to").key_type(KeyType::Hash).build(),
        "Failed to build AssignedTo GSI PK"
    )?;

    let gsi2 = build(
        GlobalSecondaryIndex::builder()
            .index_name("AssignedToIndex")
            .key_schema(gsi2_pk)
            .projection(Projection::builder().projection_type(ProjectionType::All).build())
            .build(),
        "Failed to build AssignedToIndex GSI"
    )?;

    // Define GSI 3: Status Index
    let gsi3_pk = build(
        KeySchemaElement::builder().attribute_name("status").key_type(KeyType::Hash).build(),
        "Failed to build Status GSI PK"
    )?;

    let gsi3 = build(
        GlobalSecondaryIndex::builder()
            .index_name("StatusIndex")
            .key_schema(gsi3_pk)
            .projection(Projection::builder().projection_type(ProjectionType::All).build())
            .build(),
        "Failed to build StatusIndex GSI"
    )?;

    // Define GSI 4: Priority Index
    let gsi4_pk = build(
        KeySchemaElement::builder().attribute_name("priority").key_type(KeyType::Hash).build(),
        "Failed to build Priority GSI PK"
    )?;

    let gsi4 = build(
        GlobalSecondaryIndex::builder()
            .index_name("PriorityIndex")
            .key_schema(gsi4_pk)
            .projection(Projection::builder().projection_type(ProjectionType::All).build())
            .build(),
        "Failed to build PriorityIndex GSI"
    )?;

    // Create the table
    let response = client
        .create_table()
        .table_name("WorkOrders")
        .billing_mode(BillingMode::PayPerRequest)
        .attribute_definitions(ad_id)
        .attribute_definitions(ad_asset_id)
        .attribute_definitions(ad_assigned_to)
        .attribute_definitions(ad_status)
        .attribute_definitions(ad_priority)
        .key_schema(ks_id)
        .global_secondary_indexes(gsi1)
        .global_secondary_indexes(gsi2)
        .global_secondary_indexes(gsi3)
        .global_secondary_indexes(gsi4)
        .send().await
        .map_err(|e|
            AppError::DatabaseError(
                format!("Failed to create {} table: {:?}", table_name, e.to_string())
            )
        )?;

    println!("WorkOrders table created: {:?}", response);
    Ok(())
}

pub async fn create_maintenance_requests_table(
    tables: &ListTablesOutput,
    client: &Client
) -> Result<(), AppError> {
    let table_name = "MaintenanceRequests";

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

    let ad_submitted_by = build(
        AttributeDefinition::builder()
            .attribute_name("submitted_by")
            .attribute_type(ScalarAttributeType::S)
            .build(),
        "Failed to build submitted_by attribute definition"
    )?;

    let ad_status = build(
        AttributeDefinition::builder()
            .attribute_name("status")
            .attribute_type(ScalarAttributeType::S)
            .build(),
        "Failed to build status attribute definition"
    )?;

    let ad_severity = build(
        AttributeDefinition::builder()
            .attribute_name("severity")
            .attribute_type(ScalarAttributeType::S)
            .build(),
        "Failed to build severity attribute definition"
    )?;

    let ad_created_at = build(
        AttributeDefinition::builder()
            .attribute_name("created_at")
            .attribute_type(ScalarAttributeType::S)
            .build(),
        "Failed to build created_at attribute definition"
    )?;

    // Define key schema
    let ks_id = build(
        KeySchemaElement::builder().attribute_name("id").key_type(KeyType::Hash).build(),
        "Failed to build id key schema"
    )?;

    // Define GSI 1: Submitted By Index
    let gsi1_pk = build(
        KeySchemaElement::builder().attribute_name("submitted_by").key_type(KeyType::Hash).build(),
        "Failed to build SubmittedBy GSI PK"
    )?;

    let gsi1 = build(
        GlobalSecondaryIndex::builder()
            .index_name("SubmittedByIndex")
            .key_schema(gsi1_pk)
            .projection(Projection::builder().projection_type(ProjectionType::All).build())
            .build(),
        "Failed to build SubmittedByIndex GSI"
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

    // Define GSI 3: Severity Index
    let gsi3_pk = build(
        KeySchemaElement::builder().attribute_name("severity").key_type(KeyType::Hash).build(),
        "Failed to build Severity GSI PK"
    )?;

    let gsi3 = build(
        GlobalSecondaryIndex::builder()
            .index_name("SeverityIndex")
            .key_schema(gsi3_pk)
            .projection(Projection::builder().projection_type(ProjectionType::All).build())
            .build(),
        "Failed to build SeverityIndex GSI"
    )?;

    // Define GSI 4: Created At Index (for time-based queries)
    let gsi4_pk = build(
        KeySchemaElement::builder().attribute_name("created_at").key_type(KeyType::Hash).build(),
        "Failed to build CreatedAt GSI PK"
    )?;

    let gsi4 = build(
        GlobalSecondaryIndex::builder()
            .index_name("CreatedAtIndex")
            .key_schema(gsi4_pk)
            .projection(Projection::builder().projection_type(ProjectionType::All).build())
            .build(),
        "Failed to build CreatedAtIndex GSI"
    )?;

    // Create the table
    let response = client
        .create_table()
        .table_name("MaintenanceRequests")
        .billing_mode(BillingMode::PayPerRequest)
        .attribute_definitions(ad_id)
        .attribute_definitions(ad_submitted_by)
        .attribute_definitions(ad_status)
        .attribute_definitions(ad_severity)
        .attribute_definitions(ad_created_at)
        .key_schema(ks_id)
        .global_secondary_indexes(gsi1)
        .global_secondary_indexes(gsi2)
        .global_secondary_indexes(gsi3)
        .global_secondary_indexes(gsi4)
        .send().await
        .map_err(|e|
            AppError::DatabaseError(
                format!("Failed to create {} table: {:?}", table_name, e.to_string())
            )
        )?;

    println!("MaintenanceRequests table created: {:?}", response);
    Ok(())
}
