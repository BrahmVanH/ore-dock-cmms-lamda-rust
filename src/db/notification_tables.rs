//! Notification and alerting system table definitions.
//!
//! This module contains table definitions for notifications, templates,
//! delivery logs, and user preferences for notifications.

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

/// Creates the Notifications table.
pub async fn create_notifications_table(
    tables: &ListTablesOutput,
    client: &Client
) -> Result<(), AppError> {
    let table_name = "Notifications";

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

    let ad_notification_type = build(
        AttributeDefinition::builder()
            .attribute_name("notification_type")
            .attribute_type(ScalarAttributeType::S)
            .build(),
        "Failed to build notification_type attribute definition"
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

    // Define GSI 2: Type Index
    let gsi2_pk = build(
        KeySchemaElement::builder()
            .attribute_name("notification_type")
            .key_type(KeyType::Hash)
            .build(),
        "Failed to build Type GSI PK"
    )?;

    let gsi2 = build(
        GlobalSecondaryIndex::builder()
            .index_name("TypeIndex")
            .key_schema(gsi2_pk)
            .projection(Projection::builder().projection_type(ProjectionType::All).build())
            .build(),
        "Failed to build TypeIndex GSI"
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
        .table_name("Notifications")
        .billing_mode(BillingMode::PayPerRequest)
        .attribute_definitions(ad_id)
        .attribute_definitions(ad_user_id)
        .attribute_definitions(ad_notification_type)
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

    println!("Notifications table created: {:?}", response);
    Ok(())
}

/// Creates the NotificationTemplates table.
pub async fn create_notification_templates_table(
    tables: &ListTablesOutput,
    client: &Client
) -> Result<(), AppError> {
    let table_name = "NotificationTemplates";

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

    let ad_template_type = build(
        AttributeDefinition::builder()
            .attribute_name("template_type")
            .attribute_type(ScalarAttributeType::S)
            .build(),
        "Failed to build template_type attribute definition"
    )?;

    // Define key schema
    let ks_id = build(
        KeySchemaElement::builder().attribute_name("id").key_type(KeyType::Hash).build(),
        "Failed to build id key schema"
    )?;

    // Define GSI 1: Type Index
    let gsi1_pk = build(
        KeySchemaElement::builder().attribute_name("template_type").key_type(KeyType::Hash).build(),
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

    // Create the table
    let response = client
        .create_table()
        .table_name("NotificationTemplates")
        .billing_mode(BillingMode::PayPerRequest)
        .attribute_definitions(ad_id)
        .attribute_definitions(ad_template_type)
        .key_schema(ks_id)
        .global_secondary_indexes(gsi1)
        .send().await
        .map_err(|e|
            AppError::DatabaseError(
                format!("Failed to create {} table: {:?}", table_name, e.to_string())
            )
        )?;

    println!("NotificationTemplates table created: {:?}", response);
    Ok(())
}

/// Creates the NotificationDeliveryLogs table.
pub async fn create_notification_delivery_logs_table(
    tables: &ListTablesOutput,
    client: &Client
) -> Result<(), AppError> {
    let table_name = "NotificationDeliveryLogs";

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

    let ad_notification_id = build(
        AttributeDefinition::builder()
            .attribute_name("notification_id")
            .attribute_type(ScalarAttributeType::S)
            .build(),
        "Failed to build notification_id attribute definition"
    )?;

    let ad_delivery_status = build(
        AttributeDefinition::builder()
            .attribute_name("delivery_status")
            .attribute_type(ScalarAttributeType::S)
            .build(),
        "Failed to build delivery_status attribute definition"
    )?;

    let ad_delivery_channel = build(
        AttributeDefinition::builder()
            .attribute_name("delivery_channel")
            .attribute_type(ScalarAttributeType::S)
            .build(),
        "Failed to build delivery_channel attribute definition"
    )?;

    // Define key schema
    let ks_id = build(
        KeySchemaElement::builder().attribute_name("id").key_type(KeyType::Hash).build(),
        "Failed to build id key schema"
    )?;

    // Define GSI 1: Notification Index
    let gsi1_pk = build(
        KeySchemaElement::builder()
            .attribute_name("notification_id")
            .key_type(KeyType::Hash)
            .build(),
        "Failed to build Notification GSI PK"
    )?;

    let gsi1 = build(
        GlobalSecondaryIndex::builder()
            .index_name("NotificationIndex")
            .key_schema(gsi1_pk)
            .projection(Projection::builder().projection_type(ProjectionType::All).build())
            .build(),
        "Failed to build NotificationIndex GSI"
    )?;

    // Define GSI 2: Status Index
    let gsi2_pk = build(
        KeySchemaElement::builder()
            .attribute_name("delivery_status")
            .key_type(KeyType::Hash)
            .build(),
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

    // Define GSI 3: Channel Index
    let gsi3_pk = build(
        KeySchemaElement::builder()
            .attribute_name("delivery_channel")
            .key_type(KeyType::Hash)
            .build(),
        "Failed to build Channel GSI PK"
    )?;

    let gsi3 = build(
        GlobalSecondaryIndex::builder()
            .index_name("ChannelIndex")
            .key_schema(gsi3_pk)
            .projection(Projection::builder().projection_type(ProjectionType::All).build())
            .build(),
        "Failed to build ChannelIndex GSI"
    )?;

    // Create the table
    let response = client
        .create_table()
        .table_name("NotificationDeliveryLogs")
        .billing_mode(BillingMode::PayPerRequest)
        .attribute_definitions(ad_id)
        .attribute_definitions(ad_notification_id)
        .attribute_definitions(ad_delivery_status)
        .attribute_definitions(ad_delivery_channel)
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

    println!("NotificationDeliveryLogs table created: {:?}", response);
    Ok(())
}

/// Creates the UserNotificationPreferences table.
pub async fn create_user_notification_preferences_table(
    tables: &ListTablesOutput,
    client: &Client
) -> Result<(), AppError> {
    let table_name = "UserNotificationPreferences";

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

    let ad_notification_type = build(
        AttributeDefinition::builder()
            .attribute_name("notification_type")
            .attribute_type(ScalarAttributeType::S)
            .build(),
        "Failed to build notification_type attribute definition"
    )?;

    // Define key schema - composite key
    let ks_user_id = build(
        KeySchemaElement::builder().attribute_name("user_id").key_type(KeyType::Hash).build(),
        "Failed to build user_id key schema"
    )?;

    let ks_notification_type = build(
        KeySchemaElement::builder()
            .attribute_name("notification_type")
            .key_type(KeyType::Range)
            .build(),
        "Failed to build notification_type key schema"
    )?;

    // Define GSI 1: Type Index
    let gsi1_pk = build(
        KeySchemaElement::builder()
            .attribute_name("notification_type")
            .key_type(KeyType::Hash)
            .build(),
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

    // Create the table
    let response = client
        .create_table()
        .table_name("UserNotificationPreferences")
        .billing_mode(BillingMode::PayPerRequest)
        .attribute_definitions(ad_user_id)
        .attribute_definitions(ad_notification_type)
        .key_schema(ks_user_id)
        .key_schema(ks_notification_type)
        .global_secondary_indexes(gsi1)
        .send().await
        .map_err(|e|
            AppError::DatabaseError(
                format!("Failed to create {} table: {:?}", table_name, e.to_string())
            )
        )?;

    println!("UserNotificationPreferences table created: {:?}", response);
    Ok(())
}
