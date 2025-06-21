use ore_dock_lambda_rust::models::asset_type::AssetType; // Adjust crate name
use aws_sdk_dynamodb::types::AttributeValue;
use std::collections::HashMap;
use chrono::{ DateTime, Utc };

fn create_test_asset_type() -> AssetType {
    AssetType::new(
        "type-123".to_string(),
        "Industrial Pump".to_string(),
        "High-pressure industrial water pump".to_string()
    )
}

fn create_test_asset_type_alt() -> AssetType {
    AssetType::new(
        "type-456".to_string(),
        "HVAC System".to_string(),
        "Commercial heating and cooling system".to_string()
    )
}

#[test]
fn test_to_item_converts_all_fields() {
    let asset_type = create_test_asset_type();
    let item = asset_type.to_item();

    assert_eq!(item.get("id"), Some(&AttributeValue::S("type-123".to_string())));
    assert_eq!(item.get("name"), Some(&AttributeValue::S("Industrial Pump".to_string())));
    assert_eq!(
        item.get("description"),
        Some(&AttributeValue::S("High-pressure industrial water pump".to_string()))
    );

    // Verify datetime fields are present and valid
    assert!(item.get("created_at").is_some());
    assert!(item.get("updated_at").is_some());

    // Verify all expected fields are present
    assert_eq!(item.len(), 5);
}

#[test]
fn test_to_item_datetime_format() {
    let asset_type = create_test_asset_type();
    let item = asset_type.to_item();

    let created_at_str = item.get("created_at").unwrap().as_s().unwrap();
    let updated_at_str = item.get("updated_at").unwrap().as_s().unwrap();

    // Verify the datetime strings can be parsed back
    assert!(created_at_str.parse::<DateTime<Utc>>().is_ok());
    assert!(updated_at_str.parse::<DateTime<Utc>>().is_ok());
}

#[test]
fn test_from_item_with_valid_data() {
    let mut item = HashMap::new();
    item.insert("id".to_string(), AttributeValue::S("type-789".to_string()));
    item.insert("name".to_string(), AttributeValue::S("Generator".to_string()));
    item.insert("description".to_string(), AttributeValue::S("Backup power generator".to_string()));
    item.insert("created_at".to_string(), AttributeValue::S("2023-01-15T10:30:00Z".to_string()));
    item.insert("updated_at".to_string(), AttributeValue::S("2023-01-16T15:45:00Z".to_string()));

    let asset_type = AssetType::from_item(&item);
    assert!(asset_type.is_some());

    let asset_type = asset_type.unwrap();
    assert_eq!(asset_type.id, "type-789");
    assert_eq!(asset_type.name, "Generator");
    assert_eq!(asset_type.description, "Backup power generator");

    // Verify parsed dates
    let expected_created = "2023-01-15T10:30:00Z".parse::<DateTime<Utc>>().unwrap();
    let expected_updated = "2023-01-16T15:45:00Z".parse::<DateTime<Utc>>().unwrap();
    assert_eq!(asset_type.created_at, expected_created);
    assert_eq!(asset_type.updated_at, expected_updated);
}

#[test]
fn test_from_item_missing_required_field() {
    let mut item = HashMap::new();
    item.insert("id".to_string(), AttributeValue::S("type-789".to_string()));
    // Missing name field
    item.insert("description".to_string(), AttributeValue::S("Backup power generator".to_string()));
    item.insert("created_at".to_string(), AttributeValue::S("2023-01-15T10:30:00Z".to_string()));
    item.insert("updated_at".to_string(), AttributeValue::S("2023-01-16T15:45:00Z".to_string()));

    let asset_type = AssetType::from_item(&item);
    assert!(asset_type.is_none());
}

#[test]
fn test_from_item_wrong_attribute_type() {
    let mut item = HashMap::new();
    item.insert("id".to_string(), AttributeValue::N("123".to_string())); // Wrong type
    item.insert("name".to_string(), AttributeValue::S("Generator".to_string()));
    item.insert("description".to_string(), AttributeValue::S("Backup power generator".to_string()));
    item.insert("created_at".to_string(), AttributeValue::S("2023-01-15T10:30:00Z".to_string()));
    item.insert("updated_at".to_string(), AttributeValue::S("2023-01-16T15:45:00Z".to_string()));

    let asset_type = AssetType::from_item(&item);
    assert!(asset_type.is_none());
}

#[test]
fn test_from_item_invalid_datetime() {
    let mut item = HashMap::new();
    item.insert("id".to_string(), AttributeValue::S("type-789".to_string()));
    item.insert("name".to_string(), AttributeValue::S("Generator".to_string()));
    item.insert("description".to_string(), AttributeValue::S("Backup power generator".to_string()));
    item.insert("created_at".to_string(), AttributeValue::S("invalid-date".to_string()));
    item.insert("updated_at".to_string(), AttributeValue::S("2023-01-16T15:45:00Z".to_string()));

    let asset_type = AssetType::from_item(&item);
    assert!(asset_type.is_some());

    let asset_type = asset_type.unwrap();
    // Should fallback to current time for invalid dates
    assert!(asset_type.created_at <= Utc::now());
}

#[test]
fn test_from_item_missing_datetime_fields() {
    let mut item = HashMap::new();
    item.insert("id".to_string(), AttributeValue::S("type-789".to_string()));
    item.insert("name".to_string(), AttributeValue::S("Generator".to_string()));
    item.insert("description".to_string(), AttributeValue::S("Backup power generator".to_string()));
    // Missing datetime fields

    let asset_type = AssetType::from_item(&item);
    assert!(asset_type.is_some());

    let asset_type = asset_type.unwrap();
    // Should fallback to current time
    assert!(asset_type.created_at <= Utc::now());
    assert!(asset_type.updated_at <= Utc::now());
}

#[test]
fn test_from_item_empty_map() {
    let item = HashMap::new();
    let asset_type = AssetType::from_item(&item);
    assert!(asset_type.is_none());
}

#[test]
fn test_roundtrip_to_item_from_item() {
    let original = create_test_asset_type();
    let item = original.to_item();
    let restored = AssetType::from_item(&item);

    assert!(restored.is_some());
    let restored = restored.unwrap();

    assert_eq!(original.id, restored.id);
    assert_eq!(original.name, restored.name);
    assert_eq!(original.description, restored.description);
    assert_eq!(original.created_at, restored.created_at);
    assert_eq!(original.updated_at, restored.updated_at);
}

#[test]
fn test_multiple_asset_types_roundtrip() {
    let asset_types = vec![
        create_test_asset_type(),
        create_test_asset_type_alt(),
        AssetType::new(
            "type-999".to_string(),
            "Conveyor".to_string(),
            "Material handling system".to_string()
        )
    ];

    for original in asset_types {
        let item = original.to_item();
        let restored = AssetType::from_item(&item).expect("Should restore successfully");

        assert_eq!(original.id, restored.id);
        assert_eq!(original.name, restored.name);
        assert_eq!(original.description, restored.description);
        assert_eq!(original.created_at, restored.created_at);
        assert_eq!(original.updated_at, restored.updated_at);
    }
}
