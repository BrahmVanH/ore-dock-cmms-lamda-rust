use ore_dock_lambda_rust::models::asset_type::AssetType;  // Adjust crate name

fn create_test_asset_type() -> AssetType {
    AssetType::new(
        "type-123".to_string(),
        "Industrial Pump".to_string(),
        "High-pressure industrial water pump".to_string(),
    )
}

#[test]
fn test_validate_various_name_lengths() {
    let test_cases = vec![
        ("A", true),                    // Single character
        ("AB", true),                   // Two characters
        ("Normal Asset Type", true),    // Normal length
        ("", false),                    // Empty
        ("   ", false),                 // Whitespace only
        ("\t\n", false),               // Other whitespace
    ];

    for (name, should_be_valid) in test_cases {
        let asset_type = AssetType::new(
            "test-id".to_string(),
            name.to_string(),
            "Valid description".to_string(),
        );

        let result = asset_type.validate();
        if should_be_valid {
            assert!(result.is_ok(), "Name '{}' should be valid", name);
        } else {
            assert!(result.is_err(), "Name '{}' should be invalid", name);
        }
    }
}

#[test]
fn test_validate_various_description_lengths() {
    let test_cases = vec![
        ("A", true),                                    // Single character
        ("Short desc", true),                           // Short description
        ("A".repeat(1000), true),                       // Very long description
        ("", false),                                    // Empty
        ("   ", false),                                 // Whitespace only
        ("\t\n\r", false),                             // Other whitespace
    ];

    for (description, should_be_valid) in test_cases {
        let asset_type = AssetType::new(
            "test-id".to_string(),
            "Valid Name".to_string(),
            description.clone(),
        );

        let result = asset_type.validate();
        if should_be_valid {
            assert!(result.is_ok(), "Description should be valid");
        } else {
            assert!(result.is_err(), "Description should be invalid");
        }
    }
}

#[test]
fn test_validate_special_characters_in_fields() {
    let special_chars_test_cases = vec![
        ("Pump & Filter", "High-pressure system with 100% efficiency", true),
        ("Type/System", "Input/Output processing unit", true),
        ("System-v2.0", "Version 2.0 of the system", true),
        ("Type (New)", "New type (improved version)", true),
        ("System #1", "Primary system #1", true),
        ("Type@Location", "Type at specific location", true),
    ];

    for (name, description, should_be_valid) in special_chars_test_cases {
        let asset_type = AssetType::new(
            "test-id".to_string(),
            name.to_string(),
            description.to_string(),
        );

        let result = asset_type.validate();
        if should_be_valid {
            assert!(result.is_ok(), "Name '{}' and description '{}' should be valid", name, description);
        } else {
            assert!(result.is_err(), "Name '{}' and description '{}' should be invalid", name, description);
        }
    }
}

#[test]
fn test_validate_unicode_characters() {
    let unicode_test_cases = vec![
        ("Système de Pompage", "Système de pompage haute pression", true),
        ("Насос", "Высокого давления", true),
        ("ポンプシステム", "高圧ポンプシステム", true),
        ("Système & Contrôle", "Système de contrôle intelligent", true),
    ];

    for (name, description, should_be_valid) in unicode_test_cases {
        let asset_type = AssetType::new(
            "test-id".to_string(),
            name.to_string(),
            description.to_string(),
        );

        let result = asset_type.validate();
        if should_be_valid {
            assert!(result.is_ok(), "Unicode name '{}' should be valid", name);
        } else {
            assert!(result.is_err(), "Unicode name '{}' should be invalid", name);
        }
    }
}

#[test]
fn test_validate_edge_cases() {
    // Test leading/trailing whitespace
    let mut asset_type = AssetType::new(
        "test-id".to_string(),
        "  Valid Name  ".to_string(),
        "  Valid Description  ".to_string(),
    );

    // Should be valid because we trim in validation
    assert!(asset_type.validate().is_ok());

    // Test mixed whitespace
    asset_type.name = " \t Name \n ".to_string();
    asset_type.description = " \r Description \t ".to_string();
    assert!(asset_type.validate().is_ok());

    // Test only whitespace
    asset_type.name = " \t \n \r ".to_string();
    assert!(asset_type.validate().is_err());
}

#[test]
fn test_validate_boundary_conditions() {
    // Test with very long strings
    let long_name = "A".repeat(10000);
    let long_description = "B".repeat(50000);

    let asset_type = AssetType::new(
        "test-id".to_string(),
        long_name,
        long_description,
    );

    // Should be valid (no length limits in current validation)
    assert!(asset_type.validate().is_ok());
}

#[test]
fn test_validate_consistency_after_creation() {
    let asset_type = create_test_asset_type();
    
    // Should be valid immediately after creation
    assert!(asset_type.validate().is_ok());
    
    // Should remain valid
    assert!(asset_type.validate().is_ok());
}