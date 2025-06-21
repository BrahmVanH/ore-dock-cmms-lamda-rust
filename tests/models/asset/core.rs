use ore_dock_lambda_rust::models::asset::{Asset, MaintenanceFrequencyOptions, AssetCurrentStatusOptions};
use chrono::{DateTime, Utc, TimeZone, Duration};
use rust_decimal::Decimal;

fn create_test_asset_with_frequency(frequency: &str) -> Result<Asset, crate::AppError> {
    Asset::new(
        "asset-test".to_string(),
        "Test Asset".to_string(),
        "type-test".to_string(),
        "SN123".to_string(),
        "Model123".to_string(),
        Utc::now(),
        Utc::now(),
        "loc-test".to_string(),
        "mfg-test".to_string(),
        frequency.to_string(),
        None,
        None,
        None,
    )
}

#[test]
fn test_maintenance_frequency_to_interval_days_mapping() {
    let test_cases = vec![
        ("one-time", 0),
        ("monthly", 30),
        ("quarterly", 90),
        ("annually", 365),
        ("as-needed", 0),
    ];

    for (frequency, expected_days) in test_cases {
        let asset = create_test_asset_with_frequency(frequency).unwrap();
        assert_eq!(asset.interval_days, expected_days, "Frequency '{}' should map to {} days", frequency, expected_days);
    }
}

#[test]
fn test_asset_creation_with_all_maintenance_frequencies() {
    let frequencies = vec![
        "one-time",
        "monthly", 
        "quarterly",
        "annually",
        "as-needed",
    ];

    for frequency in frequencies {
        let asset = create_test_asset_with_frequency(frequency).unwrap();
        assert_eq!(asset.maintenance_frequency.to_str(), frequency);
    }
}

#[test]
fn test_asset_creation_with_invalid_maintenance_frequency() {
    let result = create_test_asset_with_frequency("invalid-frequency");
    assert!(result.is_err());
}

#[test]
fn test_asset_default_values_with_none_downtime() {
    let asset = create_test_asset_with_frequency("monthly").unwrap();

    // Verify default values
    assert_eq!(asset.current_status, "operational");
    assert_eq!(asset.documentation_keys.len(), 0);
    assert_eq!(asset.work_order_ids.len(), 0);
    assert_eq!(asset.total_downtime_hours, Decimal::new(0, 0));
    
    // Verify last_downtime_date defaults to Unix epoch when None provided
    let unix_epoch = Utc.timestamp_opt(0, 0).single().unwrap();
    assert_eq!(asset.last_downtime_date, unix_epoch);
    
    // Verify timestamps are recent
    let now = Utc::now();
    assert!(asset.created_at <= now);
    assert!(asset.updated_at <= now);
    assert_eq!(asset.created_at, asset.updated_at);
}

#[test]
fn test_asset_with_custom_downtime_date() {
    let custom_downtime = Utc::now() - Duration::days(10);
    
    let asset = Asset::new(
        "asset-custom-downtime".to_string(),
        "Custom Downtime Asset".to_string(),
        "type-custom".to_string(),
        "SN-CUSTOM".to_string(),
        "Model-Custom".to_string(),
        Utc::now(),
        Utc::now(),
        "loc-custom".to_string(),
        "mfg-custom".to_string(),
        "weekly".to_string(),
        Some(custom_downtime),
        None,
        None,
    );

    // This should fail because "weekly" is not a valid frequency
    assert!(asset.is_err());
    
    // Try with valid frequency
    let asset = Asset::new(
        "asset-custom-downtime".to_string(),
        "Custom Downtime Asset".to_string(),
        "type-custom".to_string(),
        "SN-CUSTOM".to_string(),
        "Model-Custom".to_string(),
        Utc::now(),
        Utc::now(),
        "loc-custom".to_string(),
        "mfg-custom".to_string(),
        "quarterly".to_string(),
        Some(custom_downtime),
        None,
        None,
    ).unwrap();

    assert_eq!(asset.last_downtime_date, custom_downtime);
    assert_ne!(asset.last_downtime_date, Utc.timestamp_opt(0, 0).single().unwrap());
}

#[test]
fn test_asset_with_warranty_dates() {
    let warranty_start = Utc::now();
    let warranty_end = warranty_start + Duration::days(365 * 2); // 2 years

    let asset = Asset::new(
        "asset-warranty".to_string(),
        "Warranty Asset".to_string(),
        "type-warranty".to_string(),
        "WAR123".to_string(),
        "WarrantyModel".to_string(),
        Utc::now(),
        Utc::now(),
        "loc-warranty".to_string(),
        "mfg-warranty".to_string(),
        "annually".to_string(),
        None,
        Some(warranty_start),
        Some(warranty_end),
    ).unwrap();

    assert_eq!(asset.warranty_start_date, Some(warranty_start));
    assert_eq!(asset.warranty_end_date, Some(warranty_end));
    assert_eq!(asset.interval_days, 365);
    
    // Should still default to Unix epoch for last_downtime_date
    let unix_epoch = Utc.timestamp_opt(0, 0).single().unwrap();
    assert_eq!(asset.last_downtime_date, unix_epoch);
}

#[test]
fn test_asset_enum_string_conversions_consistency() {
    // Test that to_string and to_str return consistent results
    let maintenance_frequencies = vec![
        MaintenanceFrequencyOptions::OneTime,
        MaintenanceFrequencyOptions::Monthly,
        MaintenanceFrequencyOptions::Quarterly,
        MaintenanceFrequencyOptions::Annually,
        MaintenanceFrequencyOptions::AsNeeded,
    ];

    for freq in maintenance_frequencies {
        let to_string_result = freq.to_string();
        let to_str_result = freq.to_str();
        assert_eq!(to_string_result, to_str_result);
    }

    let asset_statuses = vec![
        AssetCurrentStatusOptions::Operational,
        AssetCurrentStatusOptions::Down,
        AssetCurrentStatusOptions::Maintenance,
        AssetCurrentStatusOptions::Retired,
        AssetCurrentStatusOptions::NeedsAttention,
    ];

    for status in asset_statuses {
        let to_string_result = status.to_string();
        let to_str_result = status.to_str();
        assert_eq!(to_string_result, to_str_result);
    }
}

#[test]
fn test_asset_enum_roundtrip_conversion() {
    let maintenance_frequency_strings = vec![
        "one-time",
        "monthly",
        "quarterly", 
        "annually",
        "as-needed",
    ];

    for freq_str in maintenance_frequency_strings {
        let freq = MaintenanceFrequencyOptions::from_string(freq_str).unwrap();
        let converted_back = freq.to_str();
        assert_eq!(freq_str, converted_back);
    }

    let status_strings = vec![
        "operational",
        "down",
        "maintenance",
        "retired",
        "needs-attention",
    ];

    for status_str in status_strings {
        let status = AssetCurrentStatusOptions::from_string(status_str).unwrap();
        let converted_back = status.to_str();
        assert_eq!(status_str, converted_back);
    }
}

#[test]
fn test_unix_epoch_handling() {
    let asset = create_test_asset_with_frequency("monthly").unwrap();
    
    // Verify Unix epoch is exactly what we expect
    let unix_epoch = Utc.timestamp_opt(0, 0).single().unwrap();
    assert_eq!(asset.last_downtime_date, unix_epoch);
    assert_eq!(asset.last_downtime_date.timestamp(), 0);
    assert_eq!(asset.last_downtime_date.to_string(), "1970-01-01 00:00:00 UTC");
}

#[test]
fn test_asset_creation_with_past_dates() {
    let purchase_date = Utc::now() - Duration::days(365);
    let installation_date = purchase_date + Duration::days(30);
    let past_downtime = Utc::now() - Duration::days(60);

    let asset = Asset::new(
        "asset-past".to_string(),
        "Past Asset".to_string(),
        "type-past".to_string(),
        "PAST123".to_string(),
        "PastModel".to_string(),
        purchase_date,
        installation_date,
        "loc-past".to_string(),
        "mfg-past".to_string(),
        "quarterly".to_string(),
        Some(past_downtime),
        None,
        None,
    ).unwrap();

    assert_eq!(asset.purchase_date, purchase_date);
    assert_eq!(asset.installation_date, installation_date);
    assert_eq!(asset.last_downtime_date, past_downtime);
    assert!(asset.purchase_date < asset.installation_date);
    assert!(asset.last_downtime_date > asset.purchase_date);
}

#[test]
fn test_asset_with_edge_case_dates() {
    // Test with very old dates
    let old_date = Utc.ymd(1990, 1, 1).and_hms(0, 0, 0);
    let newer_date = Utc.ymd(1995, 6, 15).and_hms(12, 30, 45);

    let asset = Asset::new(
        "asset-old".to_string(),
        "Old Asset".to_string(),
        "type-old".to_string(),
        "OLD123".to_string(),
        "OldModel".to_string(),
        old_date,
        newer_date,
        "loc-old".to_string(),
        "mfg-old".to_string(),
        "annually".to_string(),
        Some(newer_date),
        Some(old_date),
        Some(newer_date + Duration::days(365 * 10)),
    ).unwrap();

    assert_eq!(asset.purchase_date, old_date);
    assert_eq!(asset.installation_date, newer_date);
    assert_eq!(asset.last_downtime_date, newer_date);
    assert_eq!(asset.warranty_start_date, Some(old_date));
}