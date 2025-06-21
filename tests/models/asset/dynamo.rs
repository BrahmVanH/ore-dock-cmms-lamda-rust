use ore_dock_lambda_rust::models::asset::{Asset, MaintenanceFrequencyOptions, AssetCurrentStatusOptions};
use aws_sdk_dynamodb::types::AttributeValue;
use std::collections::HashMap;
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

fn create_test_asset_with_downtime() -> Asset {
    let downtime_date = Utc::now() - chrono::Duration::days(5);
    Asset::new(
        "asset-downtime".to_string(),
        "Downtime Asset".to_string(),
        "type-downtime".to_string(),
        "SN-DOWNTIME".to_string(),
        "Model-Downtime".to_string(),
        Utc::now(),
        Utc::now(),
        "loc-downtime".to_string(),
        "mfg-downtime".to_string(),
        "quarterly".to_string(),
        Some(downtime_date),
        None,
        None,
    ).unwrap()
}

#[test]
fn test_to_item_with_all_fields() {
    let mut asset = create_test_asset();
    asset.documentation_keys = vec!["doc1".to_string(), "doc2".to_string()];
    asset.work_order_ids = vec!["wo1".to_string(), "wo2".to_string()];
    asset.total_downtime_hours = Decimal::try_from(24.5).unwrap();
    
    let item = asset.to_item();

    // Verify basic fields
    assert_eq!(item.get("id"), Some(&AttributeValue::S("asset-123".to_string())));
    assert_eq!(item.get("name"), Some(&AttributeValue::S("Test Pump".to_string())));
    assert_eq!(item.get("r#type_id"), Some(&AttributeValue::S("type-456".to_string())));
    assert_eq!(item.get("serial_number"), Some(&AttributeValue::S("SN12345".to_string())));
    assert_eq!(item.get("model_number"), Some(&AttributeValue::S("Model-ABC".to_string())));
    assert_eq!(item.get("location_id"), Some(&AttributeValue::S("loc-789".to_string())));
    assert_eq!(item.get("manufacturer_id"), Some(&AttributeValue::S("mfg-101".to_string())));

    // Verify status (stored as string)
    assert_eq!(item.get("current_status"), Some(&AttributeValue::S("operational".to_string())));

    // Verify maintenance frequency (stored as enum string representation)
    assert_eq!(item.get("maintenance_frequency"), Some(&AttributeValue::S("monthly".to_string())));

    // Verify numeric fields
    assert_eq!(item.get("interval_days"), Some(&AttributeValue::N("30".to_string())));

    // Verify collections
    assert_eq!(item.get("documentation_keys"), Some(&AttributeValue::Ss(vec!["doc1".to_string(), "doc2".to_string()])));
    assert_eq!(item.get("work_order_ids"), Some(&AttributeValue::Ss(vec!["wo1".to_string(), "wo2".to_string()])));

    // Verify optional fields
    assert!(item.get("warranty_start_date").is_some());
    assert!(item.get("warranty_end_date").is_some());

    // Verify datetime fields are present
    assert!(item.get("purchase_date").is_some());
    assert!(item.get("installation_date").is_some());
    assert!(item.get("last_downtime_date").is_some());
    assert!(item.get("created_at").is_some());
    assert!(item.get("updated_at").is_some());

    // Verify decimal field
    assert_eq!(item.get("total_downtime_hours"), Some(&AttributeValue::S("24.5".to_string())));
}

#[test]
fn test_to_item_with_minimal_fields() {
    let asset = Asset::new(
        "asset-minimal".to_string(),
        "Minimal Asset".to_string(),
        "type-minimal".to_string(),
        "SN-MIN".to_string(),
        "Model-MIN".to_string(),
        Utc::now(),
        Utc::now(),
        "loc-minimal".to_string(),
        "mfg-minimal".to_string(),
        "as-needed".to_string(),
        None,
        None,
        None,
    ).unwrap();
    
    let item = asset.to_item();

    // Should not include empty collections
    assert!(item.get("documentation_keys").is_none() || 
            item.get("documentation_keys") == Some(&AttributeValue::Ss(vec![])));
    assert!(item.get("work_order_ids").is_none() || 
            item.get("work_order_ids") == Some(&AttributeValue::Ss(vec![])));

    // Should not include None optional fields
    assert!(item.get("warranty_start_date").is_none());
    assert!(item.get("warranty_end_date").is_none());

    // Verify Unix epoch for last_downtime_date
    let unix_epoch_str = Utc.timestamp_opt(0, 0).single().unwrap().to_string();
    assert_eq!(item.get("last_downtime_date"), Some(&AttributeValue::S(unix_epoch_str)));
}

#[test]
fn test_to_item_with_custom_downtime_date() {
    let asset = create_test_asset_with_downtime();
    let item = asset.to_item();

    // Verify custom downtime date is preserved
    let downtime_str = item.get("last_downtime_date").unwrap().as_s().unwrap();
    let parsed_downtime = downtime_str.parse::<DateTime<Utc>>().unwrap();
    assert_eq!(parsed_downtime, asset.last_downtime_date);
}

#[test]
fn test_from_item_with_all_fields() {
    let mut item = HashMap::new();
    item.insert("id".to_string(), AttributeValue::S("asset-789".to_string()));
    item.insert("name".to_string(), AttributeValue::S("Industrial Compressor".to_string()));
    item.insert("r#type_id".to_string(), AttributeValue::S("type-compressor".to_string()));
    item.insert("serial_number".to_string(), AttributeValue::S("COMP001".to_string()));
    item.insert("model_number".to_string(), AttributeValue::S("IC-2000".to_string()));
    item.insert("purchase_date".to_string(), AttributeValue::S("2023-01-15T10:30:00Z".to_string()));
    item.insert("installation_date".to_string(), AttributeValue::S("2023-01-20T14:45:00Z".to_string()));
    item.insert("current_status".to_string(), AttributeValue::S("operational".to_string()));
    item.insert("location_id".to_string(), AttributeValue::S("loc-factory".to_string()));
    item.insert("manufacturer_id".to_string(), AttributeValue::S("mfg-acme".to_string()));
    item.insert("maintenance_frequency".to_string(), AttributeValue::S("quarterly".to_string()));
    item.insert("interval_days".to_string(), AttributeValue::N("90".to_string()));
    item.insert("documentation_keys".to_string(), AttributeValue::Ss(vec!["manual".to_string(), "warranty".to_string()]));
    item.insert("work_order_ids".to_string(), AttributeValue::Ss(vec!["wo-001".to_string(), "wo-002".to_string()]));
    item.insert("warranty_start_date".to_string(), AttributeValue::S("2023-01-15T00:00:00Z".to_string()));
    item.insert("warranty_end_date".to_string(), AttributeValue::S("2025-01-15T00:00:00Z".to_string()));
    item.insert("total_downtime_hours".to_string(), AttributeValue::S("24.5".to_string()));
    item.insert("last_downtime_date".to_string(), AttributeValue::S("2023-06-01T08:00:00Z".to_string()));
    item.insert("created_at".to_string(), AttributeValue::S("2023-01-10T09:00:00Z".to_string()));
    item.insert("updated_at".to_string(), AttributeValue::S("2023-06-15T16:30:00Z".to_string()));

    let asset = Asset::from_item(&item);
    assert!(asset.is_some());

    let asset = asset.unwrap();
    assert_eq!(asset.id, "asset-789");
    assert_eq!(asset.name, "Industrial Compressor");
    assert_eq!(asset.r#type_id, "type-compressor");
    assert_eq!(asset.serial_number, "COMP001");
    assert_eq!(asset.model_number, "IC-2000");
    assert_eq!(asset.location_id, "loc-factory");
    assert_eq!(asset.manufacturer_id, "mfg-acme");
    assert_eq!(asset.current_status, "operational");
    assert!(matches!(asset.maintenance_frequency, MaintenanceFrequencyOptions::Quarterly));
    assert_eq!(asset.interval_days, 90);
    assert_eq!(asset.documentation_keys, vec!["manual".to_string(), "warranty".to_string()]);
    assert_eq!(asset.work_order_ids, vec!["wo-001".to_string(), "wo-002".to_string()]);
    assert_eq!(asset.total_downtime_hours, Decimal::try_from(24.5).unwrap());

    // Verify parsed dates
    let expected_purchase = "2023-01-15T10:30:00Z".parse::<DateTime<Utc>>().unwrap();
    let expected_installation = "2023-01-20T14:45:00Z".parse::<DateTime<Utc>>().unwrap();
    let expected_warranty_start = "2023-01-15T00:00:00Z".parse::<DateTime<Utc>>().unwrap();
    let expected_warranty_end = "2025-01-15T00:00:00Z".parse::<DateTime<Utc>>().unwrap();
    let expected_last_downtime = "2023-06-01T08:00:00Z".parse::<DateTime<Utc>>().unwrap();
    let expected_created = "2023-01-10T09:00:00Z".parse::<DateTime<Utc>>().unwrap();
    let expected_updated = "2023-06-15T16:30:00Z".parse::<DateTime<Utc>>().unwrap();

    assert_eq!(asset.purchase_date, expected_purchase);
    assert_eq!(asset.installation_date, expected_installation);
    assert_eq!(asset.warranty_start_date, Some(expected_warranty_start));
    assert_eq!(asset.warranty_end_date, Some(expected_warranty_end));
    assert_eq!(asset.last_downtime_date, expected_last_downtime);
    assert_eq!(asset.created_at, expected_created);
    assert_eq!(asset.updated_at, expected_updated);
}

#[test]
fn test_from_item_unix_epoch_downtime() {
    let mut item = HashMap::new();
    item.insert("id".to_string(), AttributeValue::S("asset-epoch".to_string()));
    item.insert("name".to_string(), AttributeValue::S("Epoch Asset".to_string()));
    item.insert("r#type_id".to_string(), AttributeValue::S("type-epoch".to_string()));
    item.insert("serial_number".to_string(), AttributeValue::S("EPOCH001".to_string()));
    item.insert("model_number".to_string(), AttributeValue::S("EPOCH-1".to_string()));
    item.insert("purchase_date".to_string(), AttributeValue::S("2023-01-15T10:30:00Z".to_string()));
    item.insert("installation_date".to_string(), AttributeValue::S("2023-01-20T14:45:00Z".to_string()));
    item.insert("current_status".to_string(), AttributeValue::S("operational".to_string()));
    item.insert("location_id".to_string(), AttributeValue::S("loc-epoch".to_string()));
    item.insert("manufacturer_id".to_string(), AttributeValue::S("mfg-epoch".to_string()));
    item.insert("maintenance_frequency".to_string(), AttributeValue::S("monthly".to_string()));
    item.insert("interval_days".to_string(), AttributeValue::N("30".to_string()));
    item.insert("total_downtime_hours".to_string(), AttributeValue::S("0".to_string()));
    item.insert("last_downtime_date".to_string(), AttributeValue::S("1970-01-01T00:00:00Z".to_string()));
    item.insert("created_at".to_string(), AttributeValue::S("2023-01-10T09:00:00Z".to_string()));
    item.insert("updated_at".to_string(), AttributeValue::S("2023-01-10T09:00:00Z".to_string()));

    let asset = Asset::from_item(&item).unwrap();
    
    // Verify Unix epoch timestamp
    let unix_epoch = Utc.timestamp_opt(0, 0).single().unwrap();
    assert_eq!(asset.last_downtime_date, unix_epoch);
}

#[test]
fn test_roundtrip_to_item_from_item() {
    let original = create_test_asset();
    let item = original.to_item();
    let restored = Asset::from_item(&item).unwrap();
    
    assert_eq!(original.id, restored.id);
    assert_eq!(original.name, restored.name);
    assert_eq!(original.r#type_id, restored.r#type_id);
    assert_eq!(original.serial_number, restored.serial_number);
    assert_eq!(original.model_number, restored.model_number);
    assert_eq!(original.purchase_date, restored.purchase_date);
    assert_eq!(original.installation_date, restored.installation_date);
    assert_eq!(original.current_status, restored.current_status);
    assert_eq!(original.location_id, restored.location_id);
    assert_eq!(original.manufacturer_id, restored.manufacturer_id);
    assert_eq!(original.maintenance_frequency.to_str(), restored.maintenance_frequency.to_str());
    assert_eq!(original.interval_days, restored.interval_days);
    assert_eq!(original.documentation_keys, restored.documentation_keys);
    assert_eq!(original.work_order_ids, restored.work_order_ids);
    assert_eq!(original.warranty_start_date, restored.warranty_start_date);
    assert_eq!(original.warranty_end_date, restored.warranty_end_date);
    assert_eq!(original.total_downtime_hours, restored.total_downtime_hours);
    assert_eq!(original.last_downtime_date, restored.last_downtime_date);
    assert_eq!(original.created_at, restored.created_at);
    assert_eq!(original.updated_at, restored.updated_at);
}

#[test]
fn test_roundtrip_with_custom_downtime() {
    let original = create_test_asset_with_downtime();
    let item = original.to_item();
    let restored = Asset::from_item(&item).unwrap();
    
    assert_eq!(original.last_downtime_date, restored.last_downtime_date);
    assert_ne!(original.last_downtime_date, Utc.timestamp_opt(0, 0).single().unwrap());
}