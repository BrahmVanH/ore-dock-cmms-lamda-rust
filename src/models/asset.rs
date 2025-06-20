use std::collections::HashMap;

use async_graphql::Object;
use aws_sdk_dynamodb::types::AttributeValue;
use chrono::{ DateTime, Utc };
use rust_decimal::Decimal;
use serde::{ Deserialize, Serialize };
use tracing::info;

use crate::{ error::AppError, models::maintenance_schedule };

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
enum MaintenanceFrequencyOptions {
    OneTime,
    Monthly,
    Quarterly,
    Annually,
    AsNeeded,
}

impl MaintenanceFrequencyOptions {
    fn to_string(&self) -> String {
        match self {
            &MaintenanceFrequencyOptions::OneTime => "one-time".to_string(),
            &MaintenanceFrequencyOptions::Annually => "annually".to_string(),
            &MaintenanceFrequencyOptions::Quarterly => "quarterly".to_string(),
            &MaintenanceFrequencyOptions::Monthly => "monthly".to_string(),
            &MaintenanceFrequencyOptions::AsNeeded => "as-needed".to_string(),
            _ => {
                return Err(
                    AppError::DatabaseError(
                        "Invalid maintenance frequency option for asset".to_string()
                    )
                );
            }
        }
    }
    fn to_str(&self) -> &str {
        match self {
            &MaintenanceFrequencyOptions::OneTime => "one-time",
            &MaintenanceFrequencyOptions::Annually => "annually",
            &MaintenanceFrequencyOptions::Quarterly => "quarterly",
            &MaintenanceFrequencyOptions::Monthly => "monthly",
            &MaintenanceFrequencyOptions::AsNeeded => "as-needed",
            _ => {
                return Err(
                    AppError::DatabaseError(
                        "Invalid maintenance frequency option for asset".to_string()
                    )
                );
            }
        }
    }
    fn from_string(s: &str) -> Result<MaintenanceFrequencyOptions, AppError> {
        match s {
            "one-time" => Ok(Self::OneTime),
            "annually" => Ok(Self::Annually),
            "quarterly" => Ok(Self::Quarterly),
            "monthly" => Ok(Self::Monthly),
            "as-needed" => Ok(Self::AsNeeded),
            _ => {
                return Err(
                    AppError::DatabaseError(
                        "Invalid maintenance frequency option string for asset".to_string()
                    )
                );
            }
        }
    }
    fn to_days(f: &MaintenanceFrequencyOptions) -> Result<i32, AppError> {
        match f {
            &MaintenanceFrequencyOptions::OneTime => Ok(0),
            &MaintenanceFrequencyOptions::Annually => Ok(365),
            &MaintenanceFrequencyOptions::Quarterly => Ok(90),
            &MaintenanceFrequencyOptions::Monthly => Ok(30),
            &MaintenanceFrequencyOptions::AsNeeded => Ok(0),
            _ => {
                return Err(
                    AppError::DatabaseError(
                        "Invalid maintenance frequency option for asset".to_string()
                    )
                );
            }
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AssetCurrentStatusOptions {
    Operational,
    Down,
    Maintenance,
    Retired,
    NeedsAttention,
}

impl AssetCurrentStatusOptions {
    fn to_string(&self) -> String {
        match self {
            &AssetCurrentStatusOptions::Operational => "operational".to_string(),
            &AssetCurrentStatusOptions::Down => "down".to_string(),
            &AssetCurrentStatusOptions::Maintenance => "maintenance".to_string(),
            &AssetCurrentStatusOptions::Retired => "retired".to_string(),
            &AssetCurrentStatusOptions::NeedsAttention => "needs-attention".to_string(),
            _ => {
                return Err(
                    AppError::DatabaseError(
                        "Invalid maintenance frequency option for asset".to_string()
                    )
                );
            }
        }
    }
    fn to_str(&self) -> &str {
        match self {
            &AssetCurrentStatusOptions::Operational => "operational",
            &AssetCurrentStatusOptions::Down => "down",
            &AssetCurrentStatusOptions::Maintenance => "maintenance",
            &AssetCurrentStatusOptions::Retired => "retired",
            &AssetCurrentStatusOptions::NeedsAttention => "needs-attention",
            _ => {
                return Err(
                    AppError::DatabaseError(
                        "Invalid maintenance frequency option for asset".to_string()
                    )
                );
            }
        }
    }
    fn from_string(s: &str) -> Result<AssetCurrentStatusOptions, AppError> {
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
/// * `r#type_id` - ID of the asset type
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
    pub r#type_id: String,
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
    /// * `r#type_id` - Asset type ID
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
        r#type_id: String,
        serial_number: String,
        model_number: String,
        purchase_date: Option<DateTime<Utc>>,
        installation_date: DateTime<Utc>,
        location_id: String,
        manufacturer_id: String,
        maintenance_frequency: String,
        warranty_start_date: Option<DateTime<Utc>>,
        warranty_end_date: Option<DateTime<Utc>>
    ) -> Result<Self, AppError> {
        let now = Utc::now();

        let maint_freq = MaintenanceFrequencyOptions::from_string(&maintenance_frequency)?;
        let maint_freq_days = MaintenanceFrequencyOptions::to_days(&maintenance_frequency)?;
        let curr_status = AssetCurrentStatusOptions::Operational;
        Ok(Self {
            id,
            name,
            r#type_id,
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
            documentation_keys: None,
            work_order_ids: Vec::new(),
            warranty_start_date,
            warranty_end_date,
            total_downtime_hours: 0.0,
            last_downtime_date: Utc.timestamp_opt(0, 0),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
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
    pub fn from_item(item: &HashMap<String, AttributeValue>) -> Option<Self> {
        info!("calling from_item with: {:?}", &item);

        let id = item.get("id")?.as_s().ok()?.to_string();
        let name = item.get("name")?.as_s().ok()?.to_string();
        let r#type_id = item.get("r#type_id")?.as_s().ok()?.to_string();
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
            .and_then(|s| s.parse::<DateTime<Utc>>().ok());

        let res = Some(Self {
            id,
            name,
            r#type_id,
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
    pub fn to_item(&self) -> HashMap<String, AttributeValue> {
        let mut item = HashMap::new();

        item.insert("id".to_string(), AttributeValue::S(self.id.clone()));
        item.insert("name".to_string(), AttributeValue::S(self.name.clone()));
        item.insert("r#type_id".to_string(), AttributeValue::S(self.r#type_id.clone()));
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
        let maintenance_schedule_id_attr_value = match self.maintenance_schedule_id {
            Some(id) => AttributeValue::S(id),
            None => AttributeValue::Null(true),
            None => {
                return Err("maintenance_schedule_id missing in to_item".into());
            }
            Some(other) => {
                return Err(
                    format!("Unexpected type for 'maintenance_schedule_id': {:?}", other).into()
                );
            }
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

#[Object]
impl Asset {
    async fn id(&self) -> &str {
        &self.id
    }

    async fn name(&self) -> &str {
        &self.name
    }

    async fn r#type_id(&self) -> &str {
        &self.r#type_id
    }

    async fn serial_number(&self) -> &str {
        &self.serial_number
    }

    async fn model_number(&self) -> &str {
        &self.model_number
    }

    async fn purchase_date(&self) -> &DateTime<Utc> {
        &self.purchase_date
    }

    async fn installation_date(&self) -> &DateTime<Utc> {
        &self.installation_date
    }

    async fn current_status(&self) -> &str {
        self.current_status.to_str()
    }

    async fn location_id(&self) -> &str {
        &self.location_id
    }

    async fn manufacturer_id(&self) -> &str {
        &self.manufacturer_id
    }

    async fn maintenance_frequency(&self) -> &str {
        self.maintenance_frequency.to_str()
    }

    async fn interval_days(&self) -> i32 {
        self.interval_days
    }

    async fn documentation_keys(&self) -> &Vec<String> {
        &self.documentation_keys
    }

    async fn work_order_ids(&self) -> &Vec<String> {
        &self.work_order_ids
    }

    async fn warranty_start_date(&self) -> Option<&DateTime<Utc>> {
        self.warranty_start_date.as_ref()
    }

    async fn warranty_end_date(&self) -> Option<&DateTime<Utc>> {
        self.warranty_end_date.as_ref()
    }

    async fn total_downtime_hours(&self) -> String {
        self.total_downtime_hours.to_string()
    }

    async fn last_downtime_date(&self) -> &DateTime<Utc> {
        &self.last_downtime_date
    }

    async fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }

    async fn updated_at(&self) -> &DateTime<Utc> {
        &self.updated_at
    }
}
