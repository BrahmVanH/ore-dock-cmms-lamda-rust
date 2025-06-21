use ore_dock_lambda_rust::models::address::Address;  // Adjust crate name
use aws_sdk_dynamodb::types::AttributeValue;
use std::collections::HashMap;

// Helper functions
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


fn create_test_address_no_unit() -> Address {
    Address::new(
        "456 Oak Avenue".to_string(),
        None,
        "Chicago".to_string(),
        "IL".to_string(),
        "United States".to_string(),
        "60601".to_string(),
    )
}

#[test]
fn test_to_item_with_unit() {
    let address = create_test_address();
    let item = address.to_item();

    assert_eq!(item.get("street"), Some(&AttributeValue::S("123 Main St".to_string())));
    assert_eq!(item.get("unit"), Some(&AttributeValue::S("Apt 4B".to_string())));
    assert_eq!(item.get("city"), Some(&AttributeValue::S("Springfield".to_string())));
    assert_eq!(item.get("state"), Some(&AttributeValue::S("IL".to_string())));
    assert_eq!(item.get("country"), Some(&AttributeValue::S("United States".to_string())));
    assert_eq!(item.get("zip"), Some(&AttributeValue::S("62701".to_string())));
    
    // Verify all fields are present
    assert_eq!(item.len(), 6);
}

#[test]
fn test_to_item_without_unit() {
    let address = create_test_address_no_unit();
    let item = address.to_item();

    assert_eq!(item.get("street"), Some(&AttributeValue::S("456 Oak Avenue".to_string())));
    assert_eq!(item.get("unit"), None); // Unit should not be present when None
    assert_eq!(item.get("city"), Some(&AttributeValue::S("Chicago".to_string())));
    assert_eq!(item.get("state"), Some(&AttributeValue::S("IL".to_string())));
    assert_eq!(item.get("country"), Some(&AttributeValue::S("United States".to_string())));
    assert_eq!(item.get("zip"), Some(&AttributeValue::S("60601".to_string())));
    
    // Verify unit field is not included when None
    assert_eq!(item.len(), 5);
}

#[test]
fn test_from_item_with_unit() {
    let mut item = HashMap::new();
    item.insert("street".to_string(), AttributeValue::S("123 Main St".to_string()));
    item.insert("unit".to_string(), AttributeValue::S("Apt 4B".to_string()));
    item.insert("city".to_string(), AttributeValue::S("Springfield".to_string()));
    item.insert("state".to_string(), AttributeValue::S("IL".to_string()));
    item.insert("country".to_string(), AttributeValue::S("United States".to_string()));
    item.insert("zip".to_string(), AttributeValue::S("62701".to_string()));

    let address = Address::from_item(&item);
    assert!(address.is_some());

    let address = address.unwrap();
    assert_eq!(address.street, "123 Main St");
    assert_eq!(address.unit, Some("Apt 4B".to_string()));
    assert_eq!(address.city, "Springfield");
    assert_eq!(address.state, "IL");
    assert_eq!(address.country, "United States");
    assert_eq!(address.zip, "62701");
}

#[test]
fn test_from_item_without_unit() {
    let mut item = HashMap::new();
    item.insert("street".to_string(), AttributeValue::S("456 Oak Avenue".to_string()));
    item.insert("city".to_string(), AttributeValue::S("Chicago".to_string()));
    item.insert("state".to_string(), AttributeValue::S("IL".to_string()));
    item.insert("country".to_string(), AttributeValue::S("United States".to_string()));
    item.insert("zip".to_string(), AttributeValue::S("60601".to_string()));

    let address = Address::from_item(&item);
    assert!(address.is_some());

    let address = address.unwrap();
    assert_eq!(address.street, "456 Oak Avenue");
    assert_eq!(address.unit, None);
    assert_eq!(address.city, "Chicago");
    assert_eq!(address.state, "IL");
    assert_eq!(address.country, "United States");
    assert_eq!(address.zip, "60601");
}

#[test]
fn test_from_item_with_null_unit() {
    let mut item = HashMap::new();
    item.insert("street".to_string(), AttributeValue::S("789 Pine St".to_string()));
    item.insert("unit".to_string(), AttributeValue::Null(true));
    item.insert("city".to_string(), AttributeValue::S("Boston".to_string()));
    item.insert("state".to_string(), AttributeValue::S("MA".to_string()));
    item.insert("country".to_string(), AttributeValue::S("United States".to_string()));
    item.insert("zip".to_string(), AttributeValue::S("02101".to_string()));

    let address = Address::from_item(&item);
    assert!(address.is_some());

    let address = address.unwrap();
    assert_eq!(address.street, "789 Pine St");
    assert_eq!(address.unit, None);
    assert_eq!(address.city, "Boston");
    assert_eq!(address.state, "MA");
    assert_eq!(address.country, "United States");
    assert_eq!(address.zip, "02101");
}

#[test]
fn test_from_item_missing_required_field() {
    let mut item = HashMap::new();
    item.insert("street".to_string(), AttributeValue::S("123 Main St".to_string()));
    item.insert("unit".to_string(), AttributeValue::S("Apt 4B".to_string()));
    // Missing city
    item.insert("state".to_string(), AttributeValue::S("IL".to_string()));
    item.insert("country".to_string(), AttributeValue::S("United States".to_string()));
    item.insert("zip".to_string(), AttributeValue::S("62701".to_string()));

    let address = Address::from_item(&item);
    assert!(address.is_none());
}

#[test]
fn test_from_item_wrong_attribute_type() {
    let mut item = HashMap::new();
    item.insert("street".to_string(), AttributeValue::N("123".to_string())); // Wrong type
    item.insert("unit".to_string(), AttributeValue::S("Apt 4B".to_string()));
    item.insert("city".to_string(), AttributeValue::S("Springfield".to_string()));
    item.insert("state".to_string(), AttributeValue::S("IL".to_string()));
    item.insert("country".to_string(), AttributeValue::S("United States".to_string()));
    item.insert("zip".to_string(), AttributeValue::S("62701".to_string()));

    let address = Address::from_item(&item);
    assert!(address.is_none());
}

/// Tests full path from Asset struct to item and back and compares pre and post
/// trip values
#[test]
fn test_roundtrip_to_item_from_item_with_unit() {
    let original = create_test_address();
    let item = original.to_item();
    let restored = Address::from_item(&item);
    
    assert!(restored.is_some());
    let restored = restored.unwrap();
    
    assert_eq!(original.street, restored.street);
    assert_eq!(original.unit, restored.unit);
    assert_eq!(original.city, restored.city);
    assert_eq!(original.state, restored.state);
    assert_eq!(original.country, restored.country);
    assert_eq!(original.zip, restored.zip);
}


/// Tests full path from Asset struct with None unit field value to item and back and compares pre and post
/// trip values 
#[test]
fn test_roundtrip_to_item_from_item_without_unit() {
    let original = create_test_address_no_unit();
    let item = original.to_item();
    let restored = Address::from_item(&item);
    
    assert!(restored.is_some());
    let restored = restored.unwrap();
    
    assert_eq!(original.street, restored.street);
    assert_eq!(original.unit, restored.unit);
    assert_eq!(original.city, restored.city);
    assert_eq!(original.state, restored.state);
    assert_eq!(original.country, restored.country);
    assert_eq!(original.zip, restored.zip);
}