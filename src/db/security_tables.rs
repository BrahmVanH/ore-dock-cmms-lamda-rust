//! Security and permissions system table definitions.
//!
//! This module contains table definitions for roles, permissions, user roles,
//! and related security functionality.

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

/// Creates the Roles table.
pub async fn create_roles_table(
    tables: &ListTablesOutput,
    client: &Client
) -> Result<(), AppError> {
    let table_name = "Roles";

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

    let ad_role_type = build(
        AttributeDefinition::builder()
            .attribute_name("role_type")
            .attribute_type(ScalarAttributeType::S)
            .build(),
        "Failed to build role_type attribute definition"
    )?;

    let ad_status = build(
        AttributeDefinition::builder()
            .attribute_name("status")
            .attribute_type(ScalarAttributeType::S)
            .build(),
        "Failed to build status attribute definition"
    )?;

    // Define key schema
    let ks_id = build(
        KeySchemaElement::builder().attribute_name("id").key_type(KeyType::Hash).build(),
        "Failed to build id key schema"
    )?;

    // Define GSI 1: Type Index
    let gsi1_pk = build(
        KeySchemaElement::builder().attribute_name("role_type").key_type(KeyType::Hash).build(),
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

    // Create the table
    let response = client
        .create_table()
        .table_name("Roles")
        .billing_mode(BillingMode::PayPerRequest)
        .attribute_definitions(ad_id)
        .attribute_definitions(ad_role_type)
        .attribute_definitions(ad_status)
        .key_schema(ks_id)
        .global_secondary_indexes(gsi1)
        .global_secondary_indexes(gsi2)
        .send().await
        .map_err(|e|
            AppError::DatabaseError(
                format!("Failed to create {} table: {:?}", table_name, e.to_string())
            )
        )?;

    println!("Roles table created: {:?}", response);
    Ok(())
}

/// Creates the UserRoles table.
pub async fn create_user_roles_table(
    tables: &ListTablesOutput,
    client: &Client
) -> Result<(), AppError> {
    let table_name = "UserRoles";

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

    let ad_user_id = build(
        AttributeDefinition::builder()
            .attribute_name("user_id")
            .attribute_type(ScalarAttributeType::S)
            .build(),
        "Failed to build user_id attribute definition"
    )?;

    let ad_role_id = build(
        AttributeDefinition::builder()
            .attribute_name("role_id")
            .attribute_type(ScalarAttributeType::S)
            .build(),
        "Failed to build role_id attribute definition"
    )?;

    let ad_status = build(
        AttributeDefinition::builder()
            .attribute_name("status")
            .attribute_type(ScalarAttributeType::S)
            .build(),
        "Failed to build status attribute definition"
    )?;

    // Define key schema
    let ks_id = build(
        KeySchemaElement::builder().attribute_name("id").key_type(KeyType::Hash).build(),
        "Failed to build id key schema"
    )?;

    // Define GSI 1: User Index
    let gsi1_pk = build(
        KeySchemaElement::builder().attribute_name("user_id").key_type(KeyType::Hash).build(),
        "Failed to build User GSI PK"
    )?;

    let gsi1 = build(
        GlobalSecondaryIndex::builder()
            .index_name("UserIndex")
            .key_schema(gsi1_pk)
            .projection(Projection::builder().projection_type(ProjectionType::All).build())
            .build(),
        "Failed to build UserIndex GSI"
    )?;

    // Define GSI 2: Role Index
    let gsi2_pk = build(
        KeySchemaElement::builder().attribute_name("role_id").key_type(KeyType::Hash).build(),
        "Failed to build Role GSI PK"
    )?;

    let gsi2 = build(
        GlobalSecondaryIndex::builder()
            .index_name("RoleIndex")
            .key_schema(gsi2_pk)
            .projection(Projection::builder().projection_type(ProjectionType::All).build())
            .build(),
        "Failed to build RoleIndex GSI"
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

    // Create the table
    let response = client
        .create_table()
        .table_name("UserRoles")
        .billing_mode(BillingMode::PayPerRequest)
        .attribute_definitions(ad_id)
        .attribute_definitions(ad_user_id)
        .attribute_definitions(ad_role_id)
        .attribute_definitions(ad_status)
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

    println!("UserRoles table created: {:?}", response);
    Ok(())
}

/// Creates the Permissions table.
pub async fn create_permissions_table(
    tables: &ListTablesOutput,
    client: &Client
) -> Result<(), AppError> {
    let table_name = "Permissions";

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

    let ad_resource_type = build(
        AttributeDefinition::builder()
            .attribute_name("resource_type")
            .attribute_type(ScalarAttributeType::S)
            .build(),
        "Failed to build resource_type attribute definition"
    )?;

    let ad_permission_type = build(
        AttributeDefinition::builder()
            .attribute_name("permission_type")
            .attribute_type(ScalarAttributeType::S)
            .build(),
        "Failed to build permission_type attribute definition"
    )?;

    // Define key schema
    let ks_id = build(
        KeySchemaElement::builder().attribute_name("id").key_type(KeyType::Hash).build(),
        "Failed to build id key schema"
    )?;

    // Define GSI 1: Resource Type Index
    let gsi1_pk = build(
        KeySchemaElement::builder().attribute_name("resource_type").key_type(KeyType::Hash).build(),
        "Failed to build ResourceType GSI PK"
    )?;

    let gsi1 = build(
        GlobalSecondaryIndex::builder()
            .index_name("ResourceTypeIndex")
            .key_schema(gsi1_pk)
            .projection(Projection::builder().projection_type(ProjectionType::All).build())
            .build(),
        "Failed to build ResourceTypeIndex GSI"
    )?;

    // Define GSI 2: Permission Type Index
    let gsi2_pk = build(
        KeySchemaElement::builder()
            .attribute_name("permission_type")
            .key_type(KeyType::Hash)
            .build(),
        "Failed to build PermissionType GSI PK"
    )?;

    let gsi2 = build(
        GlobalSecondaryIndex::builder()
            .index_name("PermissionTypeIndex")
            .key_schema(gsi2_pk)
            .projection(Projection::builder().projection_type(ProjectionType::All).build())
            .build(),
        "Failed to build PermissionTypeIndex GSI"
    )?;

    // Create the table
    let response = client
        .create_table()
        .table_name("Permissions")
        .billing_mode(BillingMode::PayPerRequest)
        .attribute_definitions(ad_id)
        .attribute_definitions(ad_resource_type)
        .attribute_definitions(ad_permission_type)
        .key_schema(ks_id)
        .global_secondary_indexes(gsi1)
        .global_secondary_indexes(gsi2)
        .send().await
        .map_err(|e|
            AppError::DatabaseError(
                format!("Failed to create {} table: {:?}", table_name, e.to_string())
            )
        )?;

    println!("Permissions table created: {:?}", response);
    Ok(())
}

/// Creates the PermissionLogs table.
pub async fn create_permission_logs_table(
    tables: &ListTablesOutput,
    client: &Client
) -> Result<(), AppError> {
    let table_name = "PermissionLogs";

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

    let ad_user_id = build(
        AttributeDefinition::builder()
            .attribute_name("user_id")
            .attribute_type(ScalarAttributeType::S)
            .build(),
        "Failed to build user_id attribute definition"
    )?;

    let ad_action_type = build(
        AttributeDefinition::builder()
            .attribute_name("action_type")
            .attribute_type(ScalarAttributeType::S)
            .build(),
        "Failed to build action_type attribute definition"
    )?;

    let ad_resource_id = build(
        AttributeDefinition::builder()
            .attribute_name("resource_id")
            .attribute_type(ScalarAttributeType::S)
            .build(),
        "Failed to build resource_id attribute definition"
    )?;

    // Define key schema
    let ks_id = build(
        KeySchemaElement::builder().attribute_name("id").key_type(KeyType::Hash).build(),
        "Failed to build id key schema"
    )?;

    // Define GSI 1: User Index
    let gsi1_pk = build(
        KeySchemaElement::builder().attribute_name("user_id").key_type(KeyType::Hash).build(),
        "Failed to build User GSI PK"
    )?;

    let gsi1 = build(
        GlobalSecondaryIndex::builder()
            .index_name("UserIndex")
            .key_schema(gsi1_pk)
            .projection(Projection::builder().projection_type(ProjectionType::All).build())
            .build(),
        "Failed to build UserIndex GSI"
    )?;

    // Define GSI 2: Action Type Index
    let gsi2_pk = build(
        KeySchemaElement::builder().attribute_name("action_type").key_type(KeyType::Hash).build(),
        "Failed to build ActionType GSI PK"
    )?;

    let gsi2 = build(
        GlobalSecondaryIndex::builder()
            .index_name("ActionTypeIndex")
            .key_schema(gsi2_pk)
            .projection(Projection::builder().projection_type(ProjectionType::All).build())
            .build(),
        "Failed to build ActionTypeIndex GSI"
    )?;

    // Define GSI 3: Resource Index
    let gsi3_pk = build(
        KeySchemaElement::builder().attribute_name("resource_id").key_type(KeyType::Hash).build(),
        "Failed to build Resource GSI PK"
    )?;

    let gsi3 = build(
        GlobalSecondaryIndex::builder()
            .index_name("ResourceIndex")
            .key_schema(gsi3_pk)
            .projection(Projection::builder().projection_type(ProjectionType::All).build())
            .build(),
        "Failed to build ResourceIndex GSI"
    )?;

    // Create the table
    let response = client
        .create_table()
        .table_name("PermissionLogs")
        .billing_mode(BillingMode::PayPerRequest)
        .attribute_definitions(ad_id)
        .attribute_definitions(ad_user_id)
        .attribute_definitions(ad_action_type)
        .attribute_definitions(ad_resource_id)
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

    println!("PermissionLogs table created: {:?}", response);
    Ok(())
}

/// Creates the RoleHierarchy table.
pub async fn create_role_hierarchy_table(
    tables: &ListTablesOutput,
    client: &Client
) -> Result<(), AppError> {
    let table_name = "RoleHierarchy";

    if tables.table_names().contains(&table_name.to_string()) {
        println!("Table '{}' already exists", table_name);
        return Ok(());
    }

    // Define attribute definitions
    let ad_parent_role_id = build(
        AttributeDefinition::builder()
            .attribute_name("parent_role_id")
            .attribute_type(ScalarAttributeType::S)
            .build(),
        "Failed to build parent_role_id attribute definition"
    )?;

    let ad_child_role_id = build(
        AttributeDefinition::builder()
            .attribute_name("child_role_id")
            .attribute_type(ScalarAttributeType::S)
            .build(),
        "Failed to build child_role_id attribute definition"
    )?;

    // Define key schema - composite key
    let ks_parent_role_id = build(
        KeySchemaElement::builder()
            .attribute_name("parent_role_id")
            .key_type(KeyType::Hash)
            .build(),
        "Failed to build parent_role_id key schema"
    )?;

    let ks_child_role_id = build(
        KeySchemaElement::builder()
            .attribute_name("child_role_id")
            .key_type(KeyType::Range)
            .build(),
        "Failed to build child_role_id key schema"
    )?;

    // Define GSI 1: Child Role Index
    let gsi1_pk = build(
        KeySchemaElement::builder().attribute_name("child_role_id").key_type(KeyType::Hash).build(),
        "Failed to build ChildRole GSI PK"
    )?;

    let gsi1 = build(
        GlobalSecondaryIndex::builder()
            .index_name("ChildRoleIndex")
            .key_schema(gsi1_pk)
            .projection(Projection::builder().projection_type(ProjectionType::All).build())
            .build(),
        "Failed to build ChildRoleIndex GSI"
    )?;

    // Create the table
    let response = client
        .create_table()
        .table_name("RoleHierarchy")
        .billing_mode(BillingMode::PayPerRequest)
        .attribute_definitions(ad_parent_role_id)
        .attribute_definitions(ad_child_role_id)
        .key_schema(ks_parent_role_id)
        .key_schema(ks_child_role_id)
        .global_secondary_indexes(gsi1)
        .send().await
        .map_err(|e|
            AppError::DatabaseError(
                format!("Failed to create {} table: {:?}", table_name, e.to_string())
            )
        )?;

    println!("RoleHierarchy table created: {:?}", response);
    Ok(())
}

/// Creates the TempRoleElevation table.
pub async fn create_temp_role_elevation_table(
    tables: &ListTablesOutput,
    client: &Client
) -> Result<(), AppError> {
    let table_name = "TempRoleElevation";

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

    let ad_user_id = build(
        AttributeDefinition::builder()
            .attribute_name("user_id")
            .attribute_type(ScalarAttributeType::S)
            .build(),
        "Failed to build user_id attribute definition"
    )?;

    let ad_target_role_id = build(
        AttributeDefinition::builder()
            .attribute_name("target_role_id")
            .attribute_type(ScalarAttributeType::S)
            .build(),
        "Failed to build target_role_id attribute definition"
    )?;

    let ad_status = build(
        AttributeDefinition::builder()
            .attribute_name("status")
            .attribute_type(ScalarAttributeType::S)
            .build(),
        "Failed to build status attribute definition"
    )?;

    // Define key schema
    let ks_id = build(
        KeySchemaElement::builder().attribute_name("id").key_type(KeyType::Hash).build(),
        "Failed to build id key schema"
    )?;

    // Define GSI 1: User Index
    let gsi1_pk = build(
        KeySchemaElement::builder().attribute_name("user_id").key_type(KeyType::Hash).build(),
        "Failed to build User GSI PK"
    )?;

    let gsi1 = build(
        GlobalSecondaryIndex::builder()
            .index_name("UserIndex")
            .key_schema(gsi1_pk)
            .projection(Projection::builder().projection_type(ProjectionType::All).build())
            .build(),
        "Failed to build UserIndex GSI"
    )?;

    // Define GSI 2: Target Role Index
    let gsi2_pk = build(
        KeySchemaElement::builder()
            .attribute_name("target_role_id")
            .key_type(KeyType::Hash)
            .build(),
        "Failed to build TargetRole GSI PK"
    )?;

    let gsi2 = build(
        GlobalSecondaryIndex::builder()
            .index_name("TargetRoleIndex")
            .key_schema(gsi2_pk)
            .projection(Projection::builder().projection_type(ProjectionType::All).build())
            .build(),
        "Failed to build TargetRoleIndex GSI"
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

    // Create the table
    let response = client
        .create_table()
        .table_name("TempRoleElevation")
        .billing_mode(BillingMode::PayPerRequest)
        .attribute_definitions(ad_id)
        .attribute_definitions(ad_user_id)
        .attribute_definitions(ad_target_role_id)
        .attribute_definitions(ad_status)
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

    println!("TempRoleElevation table created: {:?}", response);
    Ok(())
}
