use ore_dock_lambda_rust::models::asset::{Asset, MaintenanceFrequencyOptions, AssetCurrentStatusOptions};
use serde_json;
use chrono::{DateTime, Utc, TimeZone};
use rust_decimal::Decimal;

fn create_test_asset() -> Asset {
    Asset::new(
        "asset-123".to_string(),
        "Test Pump".to_string(),
        "type-456".to_string(),
        "SN12345".to_string(),
        "Model-ABC".to_string(),
        Utc::now(),
        Utc::now(),
        "loc-789".to_string(),
        "mfg-101".to_string(),
        "monthly".to_string(),
        None,
        Some(Utc::now()),
        Some(Utc::now()),
    ).unwrap()
}

#[test]
fn test_serialize_asset_to_json() {
    let asset = create_test_asset();
    
    let serialized = serde_json::to_string(&asset).expect("Failed to serialize");
    
    assert!(serialized.contains("asset-123"));
    assert!(serialized.contains("Test Pump"));
    assert!(serialized.contains("type-456"));
    assert!(serialized.contains("SN12345"));
    assert!(serialized.contains("Model-ABC"));
    assert!(serialized.contains("loc-789"));
    assert!(serialized.contains("mfg-101"));
    assert!(serialized.contains("operational"));
    assert!(serialized.contains("monthly"));
}

#[test]
fn test_serialize_with_unix_epoch_downtime() {
    let asset = Asset::new(
        "asset-epoch".to_string(),
        "Epoch Asset".to_string(),
        "type-epoch".to_string(),
        "SN-EPOCH".to_string(),
        "Model-Epoch".to_string(),
        Utc::now(),
        Utc::now(),
        "loc-epoch".to_string(),
        "mfg-epoch".to_string(),
        "as-needed".to_string(),
        None, // This should default to Unix epoch
        None,
        None,
    ).unwrap();
    
    let serialized = serde_json::to_string(&asset).expect("Failed to serialize");
    
    // Should contain Unix epoch timestamp
    assert!(serialized.contains("1970-01-01T00:00:00Z"));
}

#[test]
fn test_deserialize_asset_from_json() {
    let json = r#"{
        "id": "asset-json",
        "name": "JSON Test Asset",
        "type_id": "type-json",
        "serial_number": "JSON123",
        "model_number": "JSON-Model",
        "purchase_date": "2023-01-15T10:30:00Z",
        "installation_date": "2023-01-20T14:45:00Z",
        "current_status": "operational",
        "location_id": "loc-json",
        "manufacturer_id": "mfg-json",
        "maintenance_frequency": "monthly",
        "interval_days": 30,
        "documentation_keys": ["doc1", "doc2"],
        "work_order_ids": ["wo1"],
        "warranty_start_date": "2023-01-15T00:00:00Z",
        "warranty_end_date": "2025-01-15T00:00:00Z",
        "total_downtime_hours": "24.5",
        "last_downtime_date": "1970-01-01T00:00:00Z",
        "created_at": "2023-01-10T09:00:00Z",
        "updated_at": "2023-06-15T16:30:00Z"
    }"#;
    
    let asset: Asset = serde_json::from_str(json).expect("Failed to deserialize");
    
    assert_eq!(asset.id, "asset-json");
    assert_eq!(asset.name, "JSON Test Asset");
    assert_eq!(asset.serial_number, "JSON123");
    assert_eq!(asset.current_status, "operational");
    assert!(matches!(asset.maintenance_frequency, MaintenanceFrequencyOptions::Monthly));
    assert_eq!(asset.interval_days, 30);
    assert_eq!(asset.documentation_keys, vec!["doc1".to_string(), "doc2".to_string()]);
    assert_eq!(asset.work_order_ids, vec!["wo1".to_string()]);
    assert_eq!(asset.total_downtime_hours, Decimal::try_from(24.5).unwrap());
    
    // Verify Unix epoch timestamp
    let unix_epoch = Utc.timestamp_opt(0, 0).single().unwrap();
    assert_eq!(asset.last_downtime_date, unix_epoch);
}

#[test]
fn test_roundtrip_json_serialization() {
    let mut original = create_test_asset();
    original.documentation_keys = vec!["manual".to_string(), "spec".to_string()];
    original.work_order_ids = vec!["wo-001".to_string()];
    original.total_downtime_hours = Decimal::try_from(12.75).unwrap();
    
    // Serialize to JSON
    let serialized = serde_json::to_string(&original).expect("Failed to serialize");
    
    // Deserialize back from JSON
    let deserialized: Asset = serde_json::from_str(&serialized).expect("Failed to deserialize");
    
    // Verify all fields match
    assert_eq!(original.id, deserialized.id);
    assert_eq!(original.name, deserialized.name);
    assert_eq!(original.r#type_id, deserialized.r#type_id);
    assert_eq!(original.serial_number, deserialized.serial_number);
    assert_eq!(original.model_number, deserialized.model_number);
    assert_eq!(original.purchase_date, deserialized.purchase_date);
    assert_eq!(original.installation_date, deserialized.installation_date);
    assert_eq!(original.current_status, deserialized.current_status);
    assert_eq!(original.location_id, deserialized.location_id);
    assert_eq!(original.manufacturer_id, deserialized.manufacturer_id);
    assert_eq!(original.maintenance_frequency.to_str(), deserialized.maintenance_frequency.to_str());
    assert_eq!(original.interval_days, deserialized.interval_days);
    assert_eq!(original.documentation_keys, deserialized.documentation_keys);
    assert_eq!(original.work_order_ids, deserialized.work_order_ids);
    assert_eq!(original.total_downtime_hours, deserialized.total_downtime_hours);
    assert_eq!(original.last_downtime_date, deserialized.last_downtime_date);
}

#[test]
fn test_serialize_maintenance_frequency_enum() {
    let frequencies = vec![
        MaintenanceFrequencyOptions::OneTime,
        MaintenanceFrequencyOptions::Monthly,
        MaintenanceFrequencyOptions::Quarterly,
        MaintenanceFrequencyOptions::Annually,
        MaintenanceFrequencyOptions::AsNeeded,
    ];

    for freq in frequencies {
        let serialized = serde_json::to_string(&freq).expect("Failed to serialize");
        assert!(serialized.contains("one_time") || 
                serialized.contains("monthly") || 
                serialized.contains("quarterly") || 
                serialized.contains("annually") || 
                serialized.contains("as_needed"));
    }
}

#[test]
fn test_deserialize_with_custom_downtime_date() {
    let json = r#"{
        "id": "asset-custom",
        "name": "Custom Downtime Asset",
        "type_id": "type-custom",
        "serial_number": "CUSTOM123",
        "model_number": "Custom-Model",
        "purchase_date": "2023-01-15T10:30:00Z",
        "installation_date": "2023-01-20T14:45:00Z",
        "current_status": "operational",
        "location_id": "loc-custom",
        "manufacturer_id": "mfg-custom",
        "maintenance_frequency": "quarterly",
        "interval_days": 90,
        "documentation_keys": [],
        "work_order_ids": [],
        "warranty_start_date": null,
        "warranty_end_date": null,
        "total_downtime_hours": "0",
        "last_downtime_date": "2023-06-01T08:00:00Z",
        "created_at": "2023-01-10T09:00:00Z",
        "updated_at": "2023-01-10T09:00:00Z"
    }"#;
    
    let asset: Asset = serde_json::from_str(json).expect("Failed to deserialize");
    
    let expected_downtime = "2023-06-01T08:00:00Z".parse::<DateTime<Utc>>().unwrap();
    assert_eq!(asset.last_downtime_date, expected_downtime);
    assert_ne!(asset.last_downtime_date, Utc.timestamp_opt(0, 0).single().unwrap());
}