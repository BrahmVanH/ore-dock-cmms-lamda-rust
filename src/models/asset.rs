use std::collections::HashMap;

use async_graphql::Enum;
use aws_sdk_dynamodb::types::AttributeValue;
use chrono::{ offset::LocalResult, DateTime, TimeZone, Utc };
use rust_decimal::Decimal;
use serde::{ Deserialize, Serialize };
use tracing::info;

use crate::{ error::AppError, repository::DynamoDbEntity };

#[derive(Enum, Copy, Clone, PartialEq, Eq, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum MaintenanceFrequencyOptions {
    OneTime,
    Monthly,
    Quarterly,
    Annually,
    AsNeeded,
}

impl MaintenanceFrequencyOptions {
    pub(crate) fn to_string(&self) -> String {
        match self {
            &MaintenanceFrequencyOptions::OneTime => "one-time".to_string(),
            &MaintenanceFrequencyOptions::Annually => "annually".to_string(),
            &MaintenanceFrequencyOptions::Quarterly => "quarterly".to_string(),
            &MaintenanceFrequencyOptions::Monthly => "monthly".to_string(),
            &MaintenanceFrequencyOptions::AsNeeded => "as-needed".to_string(),
        }
    }
    pub(crate) fn to_str(&self) -> &str {
        match self {
            &MaintenanceFrequencyOptions::OneTime => "one-time",
            &MaintenanceFrequencyOptions::Annually => "annually",
            &MaintenanceFrequencyOptions::Quarterly => "quarterly",
            &MaintenanceFrequencyOptions::Monthly => "monthly",
            &MaintenanceFrequencyOptions::AsNeeded => "as-needed",
        }
    }
    pub(crate) fn from_string(s: &str) -> Result<MaintenanceFrequencyOptions, AppError> {
        match s {
            "one-time" => Ok(Self::OneTime),
            "annually" => Ok(Self::Annually),
            "quarterly" => Ok(Self::Quarterly),
            "monthly" => Ok(Self::Monthly),
            "as-needed" => Ok(Self::AsNeeded),
            _ =>
                Err(
                    AppError::DatabaseError(
                        "Cannot perform from_string on MaintenanceFrequencyOption input".to_string()
                    )
                ),
        }
    }
    pub(crate) fn to_days(f: &MaintenanceFrequencyOptions) -> Result<i32, AppError> {
        match f {
            &MaintenanceFrequencyOptions::OneTime => Ok(0),
            &MaintenanceFrequencyOptions::Annually => Ok(365),
            &MaintenanceFrequencyOptions::Quarterly => Ok(90),
            &MaintenanceFrequencyOptions::Monthly => Ok(30),
            &MaintenanceFrequencyOptions::AsNeeded => Ok(0),
        }
    }
}

#[derive(Enum, Copy, Clone, PartialEq, Eq, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AssetCurrentStatusOptions {
    Operational,
    Down,
    Maintenance,
    Retired,
    NeedsAttention,
}

impl AssetCurrentStatusOptions {
    pub(crate) fn to_string(&self) -> String {
        match self {
            &AssetCurrentStatusOptions::Operational => "operational".to_string(),
            &AssetCurrentStatusOptions::Down => "down".to_string(),
            &AssetCurrentStatusOptions::Maintenance => "maintenance".to_string(),
            &AssetCurrentStatusOptions::Retired => "retired".to_string(),
            &AssetCurrentStatusOptions::NeedsAttention => "needs-attention".to_string(),
        }
    }
    pub(crate) fn to_str(&self) -> &str {
        match self {
            &AssetCurrentStatusOptions::Operational => "operational",
            &AssetCurrentStatusOptions::Down => "down",
            &AssetCurrentStatusOptions::Maintenance => "maintenance",
            &AssetCurrentStatusOptions::Retired => "retired",
            &AssetCurrentStatusOptions::NeedsAttention => "needs-attention",
        }
    }
    pub(crate) fn from_string(s: &str) -> Result<AssetCurrentStatusOptions, AppError> {
        match s {
            "operational" => Ok(Self::Operational),
            "down" => Ok(Self::Down),
            "maintenance" => Ok(Self::Maintenance),
            "retired" => Ok(Self::Retired),
            "needs-attention" => Ok(Self::NeedsAttention),
            _ => {
                return Err(
                    AppError::DatabaseError(
                        "Invalid maintenance frequency option string for asset".to_string()
                    )
                );
            }
        }
    }
}

/// Represents an Asset in the system
///
/// # Fields
///
/// * `id` - Unique identifier for the asset
/// * `name` - Name of the asset
/// * `asset_type_id` - ID of the asset type
/// * `serial_number` - Serial number of the asset
/// * `model_number` - Model number of the asset
/// * `purchase_date` - Date when asset was purchased
/// * `installation_date` - Date when asset was installed
/// * `current_status` - Current operational status of the asset
/// * `location_id` - ID of the location where asset is installed
/// * `manufacturer_id` - ID of the manufacturer
/// * `maintenance_frequency` - How often maintenance is required
/// * `interval_days` - Number of days between maintenance
/// * `documentation_keys` - Keys for related documentation
/// * `work_order_ids` - IDs of related work orders
/// * `warranty_start_date` - Start date of warranty
/// * `warranty_end_date` - End date of warranty
/// * `total_downtime_hours` - Total hours the asset has been down
/// * `last_downtime_date` - Last date the asset was down
/// * `created_at` - Date and time of creation
/// * `updated_at` - Date and time of last update
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Asset {
    pub id: String,
    pub name: String,
    pub asset_type_id: String,
    pub serial_number: String,
    pub model_number: String,
    pub purchase_date: DateTime<Utc>,
    pub installation_date: DateTime<Utc>,
    pub current_status: AssetCurrentStatusOptions,
    pub location_id: String,
    pub manufacturer_id: String,
    pub maintenance_frequency: MaintenanceFrequencyOptions,
    pub maintenance_schedule_id: Option<String>,
    pub interval_days: i32,
    pub documentation_keys: Vec<String>,
    pub work_order_ids: Vec<String>,
    pub warranty_start_date: Option<DateTime<Utc>>,
    pub warranty_end_date: Option<DateTime<Utc>>,
    pub total_downtime_hours: Decimal,
    pub last_downtime_date: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Asset {
    /// Creates new Asset instance
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier
    /// * `name` - Name of the asset
    /// * `asset_type_id` - Asset type ID
    /// * `serial_number` - Serial number
    /// * `model_number` - Model number
    /// * `purchase_date` - Purchase date
    /// * `installation_date` - Installation date
    /// * `location_id` - Location ID
    /// * `manufacturer_id` - Manufacturer ID
    /// * `maintenance_frequency` - Maintenance frequency as string
    /// * `warranty_start_date` - Optional warranty start
    /// * `warranty_end_date` - Optional warranty end
    ///
    /// # Returns
    ///
    /// New Asset instance
    pub fn new(
        id: String,
        name: String,
        asset_type_id: String,
        serial_number: String,
        model_number: String,
        purchase_date: DateTime<Utc>,
        installation_date: DateTime<Utc>,
        location_id: String,
        manufacturer_id: String,
        maintenance_frequency: String,
        warranty_start_date: Option<DateTime<Utc>>,
        warranty_end_date: Option<DateTime<Utc>>
    ) -> Result<Self, AppError> {
        let now = Utc::now();

        let maint_freq = MaintenanceFrequencyOptions::from_string(&maintenance_frequency)?;
        let maint_freq_days = MaintenanceFrequencyOptions::to_days(&maint_freq)?;
        let curr_status = AssetCurrentStatusOptions::Operational;
        Ok(Self {
            id,
            name,
            asset_type_id,
            serial_number,
            model_number,
            purchase_date,
            installation_date,
            current_status: curr_status,
            location_id,
            manufacturer_id,
            maintenance_frequency: maint_freq,
            maintenance_schedule_id: None,
            interval_days: maint_freq_days,
            documentation_keys: Vec::new(),
            work_order_ids: Vec::new(),
            warranty_start_date,
            warranty_end_date,
            total_downtime_hours: Decimal::new(0, 0),
            last_downtime_date: match Utc.timestamp_opt(0, 0) {
                LocalResult::Single(d) => d,
                _ => DateTime::<Utc>::UNIX_EPOCH,
            },
            created_at: now,
            updated_at: now,
        })
    }

    /// Calculates the next maintenance due date for this asset
    ///
    /// # Returns
    ///
    /// DateTime<Utc> representing when the next maintenance is due
    pub(crate) fn next_maintenance_due(&self) -> DateTime<Utc> {
        // If we have a specific maintenance schedule, we should use that
        // For now, calculate based on maintenance frequency and last maintenance

        let base_date = if self.last_downtime_date == DateTime::<Utc>::UNIX_EPOCH {
            // If no previous maintenance recorded, use installation date
            self.installation_date
        } else {
            // Use the last downtime/maintenance date
            self.last_downtime_date
        };

        // Add the maintenance interval to the base date
        base_date + chrono::Duration::days(self.interval_days as i64)
    }

    /// Checks if maintenance is overdue
    ///
    /// # Returns
    ///
    /// true if maintenance is overdue, false otherwise
    pub(crate) fn is_maintenance_overdue(&self) -> bool {
        self.next_maintenance_due() < Utc::now()
    }

    /// Gets days until next maintenance (negative if overdue)
    ///
    /// # Returns
    ///
    /// Number of days until maintenance (negative if overdue)
    pub(crate) fn days_until_maintenance(&self) -> i64 {
        let next_due = self.next_maintenance_due();
        let now = Utc::now();
        (next_due - now).num_days()
    }
}

impl DynamoDbEntity for Asset {
    fn table_name() -> &'static str {
        "Assets"
    }

    fn primary_key(&self) -> String {
        self.id.clone()
    }

    /// Creates Asset instance from DynamoDB item
    ///
    /// # Arguments
    ///
    /// * `item` - The dynamo db item
    ///
    /// # Returns
    ///
    /// 'Some' Asset if item fields match, 'None' otherwise
    fn from_item(item: &HashMap<String, AttributeValue>) -> Option<Self> {
        info!("calling from_item with: {:?}", &item);

        let id = item.get("id")?.as_s().ok()?.to_string();
        let name = item.get("name")?.as_s().ok()?.to_string();
        let asset_type_id = item.get("asset_type_id")?.as_s().ok()?.to_string();
        let serial_number = item.get("serial_number")?.as_s().ok()?.to_string();
        let model_number = item.get("model_number")?.as_s().ok()?.to_string();
        let location_id = item.get("location_id")?.as_s().ok()?.to_string();
        let manufacturer_id = item.get("manufacturer_id")?.as_s().ok()?.to_string();

        let purchase_date = item
            .get("purchase_date")
            .and_then(|v| v.as_s().ok())
            .and_then(|s| s.parse::<DateTime<Utc>>().ok())
            .unwrap_or_else(|| Utc::now());

        let installation_date = item
            .get("installation_date")
            .and_then(|v| v.as_s().ok())
            .and_then(|s| s.parse::<DateTime<Utc>>().ok())
            .unwrap_or_else(|| Utc::now());

        let current_status_str = item.get("current_status")?.as_s().ok()?;
        let current_status = AssetCurrentStatusOptions::from_string(&current_status_str)
            .map_err(|e| e)
            .ok()?;

        let maintenance_frequency_str = item.get("maintenance_frequency")?.as_s().ok()?;

        let maintenance_frequency = MaintenanceFrequencyOptions::from_string(
            &maintenance_frequency_str
        )
            .map_err(|e| e)
            .ok()?;

        let maintenance_schedule_id = item.get("maintenance_schedule_id").and_then(|v| {
            match v {
                AttributeValue::S(s) => Some(s.clone()),
                AttributeValue::Null(_) => None,
                _ => None,
            }
        });

        let interval_days = item
            .get("interval_days")
            .and_then(|v| v.as_n().ok())
            .and_then(|s| s.parse::<i32>().ok())
            .unwrap_or(0);

        let documentation_keys = item
            .get("documentation_keys")
            .and_then(|v| v.as_ss().ok())
            .cloned()
            .unwrap_or_default();

        let work_order_ids = item
            .get("work_order_ids")
            .and_then(|v| v.as_ss().ok())
            .cloned()
            .unwrap_or_default();

        let warranty_start_date = item
            .get("warranty_start_date")
            .and_then(|v| v.as_s().ok())
            .and_then(|s| s.parse::<DateTime<Utc>>().ok());

        let warranty_end_date = item
            .get("warranty_end_date")
            .and_then(|v| v.as_s().ok())
            .and_then(|s| s.parse::<DateTime<Utc>>().ok());

        let total_downtime_hours = item
            .get("total_downtime_hours")
            .and_then(|v| v.as_s().ok())
            .and_then(|s| s.parse::<Decimal>().ok())
            .unwrap_or_else(|| Decimal::new(0, 0));

        let last_downtime_date = item
            .get("last_downtime_date")
            .and_then(|v| v.as_s().ok())
            .and_then(|s| s.parse::<DateTime<Utc>>().ok())
            .unwrap_or_else(|| Utc::now());

        let created_at: DateTime<Utc> = item
            .get("created_at")
            .and_then(|v| v.as_s().ok())
            .and_then(|s| s.parse::<DateTime<Utc>>().ok())
            .unwrap_or_else(|| Utc::now());

        let updated_at: DateTime<Utc> = item
            .get("updated_at")
            .and_then(|v| v.as_s().ok())
            .and_then(|s| s.parse::<DateTime<Utc>>().ok())
            .unwrap_or_else(|| Utc::now());

        let res = Some(Self {
            id,
            name,
            asset_type_id,
            serial_number,
            model_number,
            purchase_date,
            installation_date,
            current_status,
            location_id,
            manufacturer_id,
            maintenance_frequency,
            maintenance_schedule_id,
            interval_days,
            documentation_keys,
            work_order_ids,
            warranty_start_date,
            warranty_end_date,
            total_downtime_hours,
            last_downtime_date,
            created_at,
            updated_at,
        });

        info!("result of from_item on asset: {:?}", res);
        res
    }

    /// Creates DynamoDB item from Asset instance
    ///
    /// # Arguments
    ///
    /// * `self` - borrowed instance of self
    ///
    /// # Returns
    ///
    /// HashMap representing DB item for Asset instance
    fn to_item(&self) -> HashMap<String, AttributeValue> {
        let mut item = HashMap::new();

        item.insert("id".to_string(), AttributeValue::S(self.id.clone()));
        item.insert("name".to_string(), AttributeValue::S(self.name.clone()));
        item.insert("asset_type_id".to_string(), AttributeValue::S(self.asset_type_id.clone()));
        item.insert("serial_number".to_string(), AttributeValue::S(self.serial_number.clone()));
        item.insert("model_number".to_string(), AttributeValue::S(self.model_number.clone()));
        item.insert("purchase_date".to_string(), AttributeValue::S(self.purchase_date.to_string()));
        item.insert(
            "installation_date".to_string(),
            AttributeValue::S(self.installation_date.to_string())
        );
        item.insert(
            "current_status".to_string(),
            AttributeValue::S(self.current_status.to_str().to_string())
        );
        item.insert("location_id".to_string(), AttributeValue::S(self.location_id.clone()));
        item.insert("manufacturer_id".to_string(), AttributeValue::S(self.manufacturer_id.clone()));
        item.insert(
            "maintenance_frequency".to_string(),
            AttributeValue::S(self.maintenance_frequency.to_str().to_string())
        );
        let maintenance_schedule_id_attr_value = match &self.maintenance_schedule_id {
            Some(id) => AttributeValue::S(id.clone()),
            None => AttributeValue::Null(true),
        };
        item.insert("maintenance_interval_id".to_string(), maintenance_schedule_id_attr_value);
        item.insert("interval_days".to_string(), AttributeValue::N(self.interval_days.to_string()));

        if !self.documentation_keys.is_empty() {
            item.insert(
                "documentation_keys".to_string(),
                AttributeValue::Ss(self.documentation_keys.clone())
            );
        }

        if !self.work_order_ids.is_empty() {
            item.insert(
                "work_order_ids".to_string(),
                AttributeValue::Ss(self.work_order_ids.clone())
            );
        }

        if let Some(warranty_start) = &self.warranty_start_date {
            item.insert(
                "warranty_start_date".to_string(),
                AttributeValue::S(warranty_start.to_string())
            );
        }

        if let Some(warranty_end) = &self.warranty_end_date {
            item.insert(
                "warranty_end_date".to_string(),
                AttributeValue::S(warranty_end.to_string())
            );
        }

        item.insert(
            "total_downtime_hours".to_string(),
            AttributeValue::S(self.total_downtime_hours.to_string())
        );
        item.insert(
            "last_downtime_date".to_string(),
            AttributeValue::S(self.last_downtime_date.to_string())
        );
        item.insert("created_at".to_string(), AttributeValue::S(self.created_at.to_string()));
        item.insert("updated_at".to_string(), AttributeValue::S(self.updated_at.to_string()));

        item
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{ DateTime, Utc, TimeZone };
    use rust_decimal::Decimal;

    // Helper functions
    fn create_valid_asset() -> Result<Asset, AppError> {
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
            None, // last_downtime_date
            Some(Utc::now())
        )
    }

    fn create_minimal_asset() -> Result<Asset, AppError> {
        Asset::new(
            "asset-minimal".to_string(),
            "Minimal Asset".to_string(),
            "type-minimal".to_string(),
            "SN00000".to_string(),
            "Model-MIN".to_string(),
            Utc::now(),
            Utc::now(),
            "loc-minimal".to_string(),
            "mfg-minimal".to_string(),
            "as-needed".to_string(),
            None,
            None
        )
    }

    // MaintenanceFrequencyOptions tests
    #[test]
    fn test_maintenance_frequency_to_string() {
        assert_eq!(MaintenanceFrequencyOptions::OneTime.to_string(), "one-time");
        assert_eq!(MaintenanceFrequencyOptions::Monthly.to_string(), "monthly");
        assert_eq!(MaintenanceFrequencyOptions::Quarterly.to_string(), "quarterly");
        assert_eq!(MaintenanceFrequencyOptions::Annually.to_string(), "annually");
        assert_eq!(MaintenanceFrequencyOptions::AsNeeded.to_string(), "as-needed");
    }

    #[test]
    fn test_maintenance_frequency_to_str() {
        assert_eq!(MaintenanceFrequencyOptions::OneTime.to_str(), "one-time");
        assert_eq!(MaintenanceFrequencyOptions::Monthly.to_str(), "monthly");
        assert_eq!(MaintenanceFrequencyOptions::Quarterly.to_str(), "quarterly");
        assert_eq!(MaintenanceFrequencyOptions::Annually.to_str(), "annually");
        assert_eq!(MaintenanceFrequencyOptions::AsNeeded.to_str(), "as-needed");
    }

    #[test]
    fn test_maintenance_frequency_from_string_valid() {
        assert!(
            matches!(
                MaintenanceFrequencyOptions::from_string("one-time").unwrap(),
                MaintenanceFrequencyOptions::OneTime
            )
        );
        assert!(
            matches!(
                MaintenanceFrequencyOptions::from_string("monthly").unwrap(),
                MaintenanceFrequencyOptions::Monthly
            )
        );
        assert!(
            matches!(
                MaintenanceFrequencyOptions::from_string("quarterly").unwrap(),
                MaintenanceFrequencyOptions::Quarterly
            )
        );
        assert!(
            matches!(
                MaintenanceFrequencyOptions::from_string("annually").unwrap(),
                MaintenanceFrequencyOptions::Annually
            )
        );
        assert!(
            matches!(
                MaintenanceFrequencyOptions::from_string("as-needed").unwrap(),
                MaintenanceFrequencyOptions::AsNeeded
            )
        );
    }

    #[test]
    fn test_maintenance_frequency_from_string_invalid() {
        let result = MaintenanceFrequencyOptions::from_string("invalid");
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AppError::DatabaseError(_)));
    }

    #[test]
    fn test_maintenance_frequency_to_days() {
        assert_eq!(
            MaintenanceFrequencyOptions::to_days(&MaintenanceFrequencyOptions::OneTime).unwrap(),
            0
        );
        assert_eq!(
            MaintenanceFrequencyOptions::to_days(&MaintenanceFrequencyOptions::Monthly).unwrap(),
            30
        );
        assert_eq!(
            MaintenanceFrequencyOptions::to_days(&MaintenanceFrequencyOptions::Quarterly).unwrap(),
            90
        );
        assert_eq!(
            MaintenanceFrequencyOptions::to_days(&MaintenanceFrequencyOptions::Annually).unwrap(),
            365
        );
        assert_eq!(
            MaintenanceFrequencyOptions::to_days(&MaintenanceFrequencyOptions::AsNeeded).unwrap(),
            0
        );
    }

    // AssetCurrentStatusOptions tests
    #[test]
    fn test_asset_status_to_string() {
        assert_eq!(AssetCurrentStatusOptions::Operational.to_string(), "operational");
        assert_eq!(AssetCurrentStatusOptions::Down.to_string(), "down");
        assert_eq!(AssetCurrentStatusOptions::Maintenance.to_string(), "maintenance");
        assert_eq!(AssetCurrentStatusOptions::Retired.to_string(), "retired");
        assert_eq!(AssetCurrentStatusOptions::NeedsAttention.to_string(), "needs-attention");
    }

    #[test]
    fn test_asset_status_to_str() {
        assert_eq!(AssetCurrentStatusOptions::Operational.to_str(), "operational");
        assert_eq!(AssetCurrentStatusOptions::Down.to_str(), "down");
        assert_eq!(AssetCurrentStatusOptions::Maintenance.to_str(), "maintenance");
        assert_eq!(AssetCurrentStatusOptions::Retired.to_str(), "retired");
        assert_eq!(AssetCurrentStatusOptions::NeedsAttention.to_str(), "needs-attention");
    }

    #[test]
    fn test_asset_status_from_string_valid() {
        assert!(
            matches!(
                AssetCurrentStatusOptions::from_string("operational").unwrap(),
                AssetCurrentStatusOptions::Operational
            )
        );
        assert!(
            matches!(
                AssetCurrentStatusOptions::from_string("down").unwrap(),
                AssetCurrentStatusOptions::Down
            )
        );
        assert!(
            matches!(
                AssetCurrentStatusOptions::from_string("maintenance").unwrap(),
                AssetCurrentStatusOptions::Maintenance
            )
        );
        assert!(
            matches!(
                AssetCurrentStatusOptions::from_string("retired").unwrap(),
                AssetCurrentStatusOptions::Retired
            )
        );
        assert!(
            matches!(
                AssetCurrentStatusOptions::from_string("needs-attention").unwrap(),
                AssetCurrentStatusOptions::NeedsAttention
            )
        );
    }

    #[test]
    fn test_asset_status_from_string_invalid() {
        let result = AssetCurrentStatusOptions::from_string("invalid-status");
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AppError::DatabaseError(_)));
    }

    // Asset constructor tests
    #[test]
    fn test_asset_new_with_valid_data() {
        let asset = create_valid_asset().unwrap();

        assert_eq!(asset.id, "asset-123");
        assert_eq!(asset.name, "Test Pump");
        assert_eq!(asset.asset_type_id, "type-456");
        assert_eq!(asset.serial_number, "SN12345");
        assert_eq!(asset.model_number, "Model-ABC");
        assert_eq!(asset.location_id, "loc-789");
        assert_eq!(asset.manufacturer_id, "mfg-101");
        assert_eq!(asset.current_status.to_str(), "operational");
        assert!(matches!(asset.maintenance_frequency, MaintenanceFrequencyOptions::Monthly));
        assert_eq!(asset.interval_days, 30);
        assert_eq!(asset.documentation_keys.len(), 0);
        assert_eq!(asset.work_order_ids.len(), 0);
        assert_eq!(asset.total_downtime_hours, Decimal::new(0, 0));

        // Check that last_downtime_date defaults to Unix epoch when None provided
        let unix_epoch = Utc.timestamp_opt(0, 0).single().unwrap();
        assert_eq!(asset.last_downtime_date, unix_epoch);

        // Check timestamps
        assert!(asset.created_at <= Utc::now());
        assert!(asset.updated_at <= Utc::now());
        assert_eq!(asset.created_at, asset.updated_at);
    }

    #[test]
    fn test_asset_new_with_minimal_data() {
        let asset = create_minimal_asset().unwrap();

        assert_eq!(asset.id, "asset-minimal");
        assert_eq!(asset.name, "Minimal Asset");
        assert_eq!(asset.current_status.to_str(), "operational");
        assert!(matches!(asset.maintenance_frequency, MaintenanceFrequencyOptions::AsNeeded));
        assert_eq!(asset.interval_days, 0);
        assert_eq!(asset.warranty_start_date, None);
        assert_eq!(asset.warranty_end_date, None);

        // Check that last_downtime_date defaults to Unix epoch
        let unix_epoch = Utc.timestamp_opt(0, 0).single().unwrap();
        assert_eq!(asset.last_downtime_date, unix_epoch);
    }

    #[test]
    fn test_asset_new_with_custom_last_downtime_date() {
        let custom_downtime = Utc::now() - chrono::Duration::days(5);

        let asset = Asset::new(
            "asset-custom".to_string(),
            "Custom Asset".to_string(),
            "type-custom".to_string(),
            "SN-CUSTOM".to_string(),
            "Model-Custom".to_string(),
            Utc::now(),
            Utc::now(),
            "loc-custom".to_string(),
            "mfg-custom".to_string(),
            "monthly".to_string(),
            Some(custom_downtime),
            None
        ).unwrap();

        assert_eq!(asset.last_downtime_date, custom_downtime);
    }

    #[test]
    fn test_asset_new_with_warranty_dates() {
        let warranty_start = Utc::now();
        let warranty_end = warranty_start + chrono::Duration::days(365);

        let asset = Asset::new(
            "asset-warranty".to_string(),
            "Warranty Asset".to_string(),
            "type-warranty".to_string(),
            "SN-WARRANTY".to_string(),
            "Model-Warranty".to_string(),
            Utc::now(),
            Utc::now(),
            "loc-warranty".to_string(),
            "mfg-warranty".to_string(),
            "annually".to_string(),
            None,
            Some(warranty_start)
        ).unwrap();

        assert_eq!(asset.warranty_start_date, Some(warranty_start));
        assert_eq!(asset.warranty_end_date, Some(warranty_end));
        assert_eq!(asset.interval_days, 365);
    }

    #[test]
    fn test_asset_new_with_invalid_maintenance_frequency() {
        let result = Asset::new(
            "asset-invalid".to_string(),
            "Invalid Asset".to_string(),
            "type-invalid".to_string(),
            "SN-INVALID".to_string(),
            "Model-Invalid".to_string(),
            Utc::now(),
            Utc::now(),
            "loc-invalid".to_string(),
            "mfg-invalid".to_string(),
            "invalid-frequency".to_string(),
            None,
            None
        );

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AppError::DatabaseError(_)));
    }

    #[test]
    fn test_asset_default_values() {
        let asset = create_valid_asset().unwrap();

        // Verify default collections are empty
        assert!(asset.documentation_keys.is_empty());
        assert!(asset.work_order_ids.is_empty());

        // Verify default numeric values
        assert_eq!(asset.total_downtime_hours, Decimal::new(0, 0));

        // Verify default status
        assert_eq!(asset.current_status.to_str(), "operational");

        // Verify Unix epoch default for last_downtime_date
        let unix_epoch = Utc.timestamp_opt(0, 0).single().unwrap();
        assert_eq!(asset.last_downtime_date, unix_epoch);
    }

    #[test]
    fn test_asset_timestamps_consistency() {
        let before = Utc::now();
        let asset = create_valid_asset().unwrap();
        let after = Utc::now();

        assert!(asset.created_at >= before);
        assert!(asset.created_at <= after);
        assert!(asset.updated_at >= before);
        assert!(asset.updated_at <= after);
        assert_eq!(asset.created_at, asset.updated_at);
    }

    #[test]
    fn test_asset_clone() {
        let asset = create_valid_asset().unwrap();
        let cloned = asset.clone();

        assert_eq!(asset.id, cloned.id);
        assert_eq!(asset.name, cloned.name);
        assert_eq!(asset.asset_type_id, cloned.asset_type_id);
        assert_eq!(asset.serial_number, cloned.serial_number);
        assert_eq!(asset.model_number, cloned.model_number);
        assert_eq!(asset.purchase_date, cloned.purchase_date);
        assert_eq!(asset.installation_date, cloned.installation_date);
        assert_eq!(asset.current_status, cloned.current_status);
        assert_eq!(asset.location_id, cloned.location_id);
        assert_eq!(asset.manufacturer_id, cloned.manufacturer_id);
        assert_eq!(asset.interval_days, cloned.interval_days);
        assert_eq!(asset.documentation_keys, cloned.documentation_keys);
        assert_eq!(asset.work_order_ids, cloned.work_order_ids);
        assert_eq!(asset.warranty_start_date, cloned.warranty_start_date);
        assert_eq!(asset.warranty_end_date, cloned.warranty_end_date);
        assert_eq!(asset.total_downtime_hours, cloned.total_downtime_hours);
        assert_eq!(asset.last_downtime_date, cloned.last_downtime_date);
        assert_eq!(asset.created_at, cloned.created_at);
        assert_eq!(asset.updated_at, cloned.updated_at);
    }

    #[test]
    fn test_maintenance_frequency_enum_consistency() {
        let frequencies = vec![
            MaintenanceFrequencyOptions::OneTime,
            MaintenanceFrequencyOptions::Monthly,
            MaintenanceFrequencyOptions::Quarterly,
            MaintenanceFrequencyOptions::Annually,
            MaintenanceFrequencyOptions::AsNeeded
        ];

        for freq in frequencies {
            let to_string_result = freq.to_string();
            let to_str_result = freq.to_str();
            assert_eq!(to_string_result, to_str_result);

            // Test roundtrip conversion
            let parsed = MaintenanceFrequencyOptions::from_string(&to_string_result).unwrap();
            assert_eq!(freq.to_str(), parsed.to_str());
        }
    }

    #[test]
    fn test_asset_status_enum_consistency() {
        let statuses = vec![
            AssetCurrentStatusOptions::Operational,
            AssetCurrentStatusOptions::Down,
            AssetCurrentStatusOptions::Maintenance,
            AssetCurrentStatusOptions::Retired,
            AssetCurrentStatusOptions::NeedsAttention
        ];

        for status in statuses {
            let to_string_result = status.to_string();
            let to_str_result = status.to_str();
            assert_eq!(to_string_result, to_str_result);

            // Test roundtrip conversion
            let parsed = AssetCurrentStatusOptions::from_string(&to_string_result).unwrap();
            assert_eq!(status.to_str(), parsed.to_str());
        }
    }
}
