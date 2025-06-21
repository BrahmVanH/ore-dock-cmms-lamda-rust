use ore_dock_lambda_rust::models::asset_type::AssetType;  // Adjust crate name
use serde_json;
use chrono::{DateTime, Utc};

fn create_test_asset_type() -> AssetType {
    AssetType::new(
        "type-123".to_string(),
        "Industrial Pump".to_string(),
        "High-pressure industrial water pump".to_string(),
    )
}

#[test]
fn test_serialize_to_json() {
    let asset_type = create_test_asset_type();
    
    let serialized = serde_json::to_string(&asset_type).expect("Failed to serialize");
    
    assert!(serialized.contains("type-123"));
    assert!(serialized.contains("Industrial Pump"));
    assert!(serialized.contains("High-pressure industrial water pump"));
    assert!(serialized.contains("created_at"));
    assert!(serialized.contains("updated_at"));
}

#[test]
fn test_serialize_to_pretty_json() {
    let asset_type = create_test_asset_type();
    
    let serialized = serde_json::to_string_pretty(&asset_type).expect("Failed to serialize");
    
    // Verify pretty formatting
    assert!(serialized.contains("{\n"));
    assert!(serialized.contains("  \"id\": \"type-123\""));
    assert!(serialized.contains("  \"name\": \"Industrial Pump\""));
}

#[test]
fn test_deserialize_from_json() {
    let json = r#"{
        "id": "type-456",
        "name": "HVAC System",
        "description": "Commercial heating and cooling system",
        "created_at": "2023-01-15T10:30:00Z",
        "updated_at": "2023-01-16T15:45:00Z"
    }"#;
    
    let asset_type: AssetType = serde_json::from_str(json).expect("Failed to deserialize");
    
    assert_eq!(asset_type.id, "type-456");
    assert_eq!(asset_type.name, "HVAC System");
    assert_eq!(asset_type.description, "Commercial heating and cooling system");
    
    let expected_created = "2023-01-15T10:30:00Z".parse::<DateTime<Utc>>().unwrap();
    let expected_updated = "2023-01-16T15:45:00Z".parse::<DateTime<Utc>>().unwrap();
    assert_eq!(asset_type.created_at, expected_created);
    assert_eq!(asset_type.updated_at, expected_updated);
}

#[test]
fn test_deserialize_minimal_json() {
    let json = r#"{
        "id": "type-minimal",
        "name": "Minimal Type",
        "description": "Minimal description",
        "created_at": "2023-01-01T00:00:00Z",
        "updated_at": "2023-01-01T00:00:00Z"
    }"#;
    
    let asset_type: AssetType = serde_json::from_str(json).expect("Failed to deserialize");
    
    assert_eq!(asset_type.id, "type-minimal");
    assert_eq!(asset_type.name, "Minimal Type");
    assert_eq!(asset_type.description, "Minimal description");
}

#[test]
fn test_roundtrip_json_serialization() {
    let original = create_test_asset_type();
    
    // Serialize to JSON
    let serialized = serde_json::to_string(&original).expect("Failed to serialize");
    
    // Deserialize back from JSON
    let deserialized: AssetType = serde_json::from_str(&serialized).expect("Failed to deserialize");
    
    // Verify all fields match
    assert_eq!(original.id, deserialized.id);
    assert_eq!(original.name, deserialized.name);
    assert_eq!(original.description, deserialized.description);
    assert_eq!(original.created_at, deserialized.created_at);
    assert_eq!(original.updated_at, deserialized.updated_at);
}

#[test]
fn test_serialize_with_special_characters() {
    let asset_type = AssetType::new(
        "type-special".to_string(),
        "Pump & Filter System".to_string(),
        "High-pressure pump with \"smart\" filtering capabilities".to_string(),
    );
    
    let serialized = serde_json::to_string(&asset_type).expect("Failed to serialize");
    let deserialized: AssetType = serde_json::from_str(&serialized).expect("Failed to deserialize");
    
    assert_eq!(asset_type.id, deserialized.id);
    assert_eq!(asset_type.name, deserialized.name);
    assert_eq!(asset_type.description, deserialized.description);
}

#[test]
fn test_serialize_with_unicode() {
    let asset_type = AssetType::new(
        "type-unicode".to_string(),
        "Système de Pompage".to_string(),
        "Système de pompage haute pression avec contrôle intelligent".to_string(),
    );
    
    let serialized = serde_json::to_string(&asset_type).expect("Failed to serialize");
    let deserialized: AssetType = serde_json::from_str(&serialized).expect("Failed to deserialize");
    
    assert_eq!(asset_type.id, deserialized.id);
    assert_eq!(asset_type.name, deserialized.name);
    assert_eq!(asset_type.description, deserialized.description);
}

#[test]
fn test_deserialize_invalid_json_fails() {
    let invalid_json = r#"{
        "id": "type-456",
        "name": "HVAC System",
        // Missing description and invalid comment
        "created_at": "2023-01-15T10:30:00Z"
    }"#;
    
    let result = serde_json::from_str::<AssetType>(invalid_json);
    assert!(result.is_err());
}

#[test]
fn test_deserialize_missing_required_field_fails() {
    let json = r#"{
        "id": "type-456",
        "description": "Missing name field",
        "created_at": "2023-01-15T10:30:00Z",
        "updated_at": "2023-01-16T15:45:00Z"
    }"#;
    
    let result = serde_json::from_str::<AssetType>(json);
    assert!(result.is_err());
}