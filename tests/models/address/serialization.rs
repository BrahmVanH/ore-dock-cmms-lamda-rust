use ore_dock_lambda_rust::models::address::Address;  // Adjust crate name
use serde_json;

fn create_test_address() -> Address {
    Address::new(
        "123 Main St".to_string(),
        Some("Apt 4B".to_string()),
        "Springfield".to_string(),
        "IL".to_string(),
        "United States".to_string(),
        "62701".to_string(),
    )
}

#[test]
fn test_serialize_to_json() {
    let address = create_test_address();
    
    let serialized = serde_json::to_string(&address).expect("Failed to serialize");
    assert!(serialized.contains("123 Main St"));
    assert!(serialized.contains("Apt 4B"));
    assert!(serialized.contains("Springfield"));
    assert!(serialized.contains("IL"));
    assert!(serialized.contains("United States"));
    assert!(serialized.contains("62701"));
}

#[test]
fn test_deserialize_from_json() {
    let json = r#"{
        "street": "123 Main St",
        "unit": "Apt 4B",
        "city": "Springfield",
        "state": "IL",
        "country": "United States",
        "zip": "62701"
    }"#;
    
    let address: Address = serde_json::from_str(json).expect("Failed to deserialize");
    
    assert_eq!(address.street, "123 Main St");
    assert_eq!(address.unit, Some("Apt 4B".to_string()));
    assert_eq!(address.city, "Springfield");
    assert_eq!(address.state, "IL");
    assert_eq!(address.country, "United States");
    assert_eq!(address.zip, "62701");
}

#[test]
fn test_deserialize_from_json_no_unit() {
    let json = r#"{
        "street": "456 Oak Ave",
        "unit": null,
        "city": "Chicago",
        "state": "IL",
        "country": "United States",
        "zip": "60601"
    }"#;
    
    let address: Address = serde_json::from_str(json).expect("Failed to deserialize");
    
    assert_eq!(address.street, "456 Oak Ave");
    assert_eq!(address.unit, None);
    assert_eq!(address.city, "Chicago");
}

#[test]
fn test_roundtrip_json_serialization() {
    let original = create_test_address();
    
    // Serialize to JSON
    let serialized = serde_json::to_string(&original).expect("Failed to serialize");
    
    // Deserialize back from JSON
    let deserialized: Address = serde_json::from_str(&serialized).expect("Failed to deserialize");
    
    // Verify all fields match
    assert_eq!(original.street, deserialized.street);
    assert_eq!(original.unit, deserialized.unit);
    assert_eq!(original.city, deserialized.city);
    assert_eq!(original.state, deserialized.state);
    assert_eq!(original.country, deserialized.country);
    assert_eq!(original.zip, deserialized.zip);
}

#[test]
fn test_serialize_with_unicode() {
    let address = Address::new(
        "123 Straße".to_string(),
        Some("Café #1".to_string()),
        "São Paulo".to_string(),
        "São Paulo".to_string(),
        "Brasil".to_string(),
        "01310-100".to_string(),
    );
    
    let serialized = serde_json::to_string(&address).expect("Failed to serialize");
    let deserialized: Address = serde_json::from_str(&serialized).expect("Failed to deserialize");
    
    assert_eq!(address.street, deserialized.street);
    assert_eq!(address.unit, deserialized.unit);
    assert_eq!(address.city, deserialized.city);
}