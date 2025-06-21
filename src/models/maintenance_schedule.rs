use std::collections::HashMap;

use aws_sdk_dynamodb::types::AttributeValue;
use chrono::{ DateTime, Utc };
use serde::{ Deserialize, Serialize };

use crate::error::AppError;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CadenceUnit {
    Hours,
    Days,
    Weeks,
    Months,
    Years,
    RunHours, // Based on equipment runtime
    Cycles, // Based on operation cycles
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MaintenanceCadence {
    pub interval: i32,
    pub unit: CadenceUnit,
}

impl MaintenanceCadence {
    pub(crate) fn to_string(&self) -> String {
        format!("{}_{}", self.interval, self.unit.to_str())
    }

    pub(crate) fn from_string(s: &str) -> Result<Self, AppError> {
        let parts: Vec<&str> = s.split('_').collect();
        if parts.len() != 2 {
            return Err(AppError::ValidationError("Invalid cadence format".to_string()));
        }

        let interval = parts[0]
            .parse::<i32>()
            .map_err(|_| AppError::ValidationError("Invalid interval".to_string()))?;
        let unit = CadenceUnit::from_string(parts[1])?;

        Ok(Self { interval, unit })
    }

    pub(crate) fn to_days(&self) -> Result<i32, AppError> {
        match self.unit {
            CadenceUnit::Hours => Ok(self.interval / 24),
            CadenceUnit::Days => Ok(self.interval),
            CadenceUnit::Weeks => Ok(self.interval * 7),
            CadenceUnit::Months => Ok(self.interval * 30), // Approximate
            CadenceUnit::Years => Ok(self.interval * 365), // Approximate
            CadenceUnit::RunHours | CadenceUnit::Cycles => {
                Err(
                    AppError::ValidationError(
                        "Cannot convert runtime-based cadence to days".to_string()
                    )
                )
            }
        }
    }
}

impl CadenceUnit {
    pub(crate) fn to_str(&self) -> &str {
        match self {
            CadenceUnit::Hours => "hours",
            CadenceUnit::Days => "days",
            CadenceUnit::Weeks => "weeks",
            CadenceUnit::Months => "months",
            CadenceUnit::Years => "years",
            CadenceUnit::RunHours => "run_hours",
            CadenceUnit::Cycles => "cycles",
        }
    }

    pub(crate) fn from_string(s: &str) -> Result<Self, AppError> {
        match s {
            "hours" => Ok(Self::Hours),
            "days" => Ok(Self::Days),
            "weeks" => Ok(Self::Weeks),
            "months" => Ok(Self::Months),
            "years" => Ok(Self::Years),
            "run_hours" => Ok(Self::RunHours),
            "cycles" => Ok(Self::Cycles),
            _ => Err(AppError::ValidationError("Invalid cadence unit".to_string())),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MaintenanceSchedule {
    pub id: String,
    pub asset_id: String,
    pub cadences: Vec<MaintenanceCadence>,
    pub last_completed_at: Option<DateTime<Utc>>,
    pub last_completed_by_user_id: Option<String>,
    pub next_due_at: DateTime<Utc>,
    pub duration_estimate: Option<i32>,
    pub recurring: bool,
    pub active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl MaintenanceSchedule {
    /// Creates new MaintenanceSchedule instance
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier
    /// * `asset_id` - Asset ID this schedule applies to
    /// * `cadences` - List of maintenance cadences
    /// * `last_completed_at` - Optional last completion time
    /// * `last_completed_by_user_id` - Optional user who last completed
    /// * `next_due_at` - When maintenance is next due
    /// * `duration_estimate` - Optional duration estimate in minutes
    /// * `recurring` - Whether schedule is recurring
    /// * `active` - Whether schedule is active
    ///
    /// # Returns
    ///
    /// New MaintenanceSchedule instance
    pub fn new(
        id: String,
        asset_id: String,
        cadences: Vec<MaintenanceCadence>,
        last_completed_at: Option<DateTime<Utc>>,
        last_completed_by_user_id: Option<String>,
        next_due_at: DateTime<Utc>,
        duration_estimate: Option<i32>,
        recurring: bool,
        active: bool
    ) -> Result<Self, AppError> {
        let now = Utc::now();

        if cadences.is_empty() {
            return Err(AppError::ValidationError("At least one cadence is required".to_string()));
        }

        Ok(Self {
            id,
            asset_id,
            cadences,
            last_completed_at,
            last_completed_by_user_id,
            next_due_at,
            duration_estimate,
            recurring,
            active,
            created_at: now,
            updated_at: now,
        })
    }

    /// Creates MaintenanceSchedule instance from DynamoDB item
    ///
    /// # Arguments
    ///
    /// * `item` - The dynamo db item
    ///
    /// # Returns
    ///
    /// 'Some' MaintenanceSchedule if item fields match, 'None' otherwise
    pub(crate) fn from_item(item: &HashMap<String, AttributeValue>) -> Option<Self> {
        let id = item.get("id")?.as_s().ok()?.to_string();
        let asset_id = item.get("asset_id")?.as_s().ok()?.to_string();

        // Handle cadences as a list of maps
        let cadences = item
            .get("cadences")
            .and_then(|v| v.as_l().ok())
            .map(|cadence_list| {
                cadence_list
                    .iter()
                    .filter_map(|cadence_attr| {
                        cadence_attr
                            .as_m()
                            .ok()
                            .and_then(|cadence_map| MaintenanceCadence::from_item(cadence_map))
                    })
                    .collect::<Vec<MaintenanceCadence>>()
            })
            .unwrap_or_default();

        let last_completed_at = item
            .get("last_completed_at")
            .and_then(|v| v.as_s().ok())
            .and_then(|s| s.parse::<DateTime<Utc>>().ok());

        let last_completed_by_user_id = item
            .get("last_completed_by_user_id")
            .and_then(|v| v.as_s().ok())
            .map(|s| s.to_string());

        let next_due_at = item
            .get("next_due_at")
            .and_then(|v| v.as_s().ok())
            .and_then(|s| s.parse::<DateTime<Utc>>().ok())
            .unwrap_or_else(|| Utc::now());

        let duration_estimate = item
            .get("duration_estimate")
            .and_then(|v| v.as_n().ok())
            .and_then(|s| s.parse::<i32>().ok());

        let recurring = item
            .get("recurring")
            .and_then(|v| v.as_bool().ok())
            .unwrap_or(&false);

        let active = item
            .get("active")
            .and_then(|v| v.as_bool().ok())
            .unwrap_or(&true);

        let created_at = item
            .get("created_at")
            .and_then(|v| v.as_s().ok())
            .and_then(|s| s.parse::<DateTime<Utc>>().ok())
            .unwrap_or_else(|| Utc::now());

        let updated_at = item
            .get("updated_at")
            .and_then(|v| v.as_s().ok())
            .and_then(|s| s.parse::<DateTime<Utc>>().ok())
            .unwrap_or_else(|| Utc::now());

        let res = Some(Self {
            id,
            asset_id,
            cadences,
            last_completed_at,
            last_completed_by_user_id,
            next_due_at,
            duration_estimate,
            recurring: *recurring,
            active: *active,
            created_at,
            updated_at,
        });

        res
    }

    /// Creates DynamoDB item from MaintenanceSchedule instance
    ///
    /// # Arguments
    ///
    /// * `self` - borrowed instance of self
    ///
    /// # Returns
    ///
    /// HashMap representing DB item for MaintenanceSchedule instance
    pub(crate)  fn to_item(&self) -> HashMap<String, AttributeValue> {
        let mut item = HashMap::new();

        item.insert("id".to_string(), AttributeValue::S(self.id.clone()));
        item.insert("asset_id".to_string(), AttributeValue::S(self.asset_id.clone()));

        // Convert cadences to list of maps
        let cadence_list: Vec<AttributeValue> = self.cadences
            .iter()
            .map(|cadence| AttributeValue::M(cadence.to_item()))
            .collect();
        item.insert("cadences".to_string(), AttributeValue::L(cadence_list));

        if let Some(last_completed) = &self.last_completed_at {
            item.insert(
                "last_completed_at".to_string(),
                AttributeValue::S(last_completed.to_string())
            );
        }

        if let Some(user_id) = &self.last_completed_by_user_id {
            item.insert(
                "last_completed_by_user_id".to_string(),
                AttributeValue::S(user_id.clone())
            );
        }

        item.insert("next_due_at".to_string(), AttributeValue::S(self.next_due_at.to_string()));

        if let Some(duration) = &self.duration_estimate {
            item.insert("duration_estimate".to_string(), AttributeValue::N(duration.to_string()));
        }

        item.insert("recurring".to_string(), AttributeValue::Bool(self.recurring));
        item.insert("active".to_string(), AttributeValue::Bool(self.active));
        item.insert("created_at".to_string(), AttributeValue::S(self.created_at.to_string()));
        item.insert("updated_at".to_string(), AttributeValue::S(self.updated_at.to_string()));

        item
    }
}

#[Object]
impl MaintenanceSchedule {
    async fn id(&self) -> &str {
        &self.id
    }

    async fn asset_id(&self) -> &str {
        &self.asset_id
    }

    async fn cadences(&self) -> &Vec<MaintenanceCadence> {
        &self.cadences
    }

    async fn last_completed_at(&self) -> Option<&DateTime<Utc>> {
        self.last_completed_at.as_ref()
    }

    async fn last_completed_by_user_id(&self) -> Option<&str> {
        self.last_completed_by_user_id.as_deref()
    }

    async fn next_due_at(&self) -> &DateTime<Utc> {
        &self.next_due_at
    }

    async fn duration_estimate(&self) -> Option<i32> {
        self.duration_estimate
    }

    async fn recurring(&self) -> bool {
        self.recurring
    }

    async fn active(&self) -> bool {
        self.active
    }

    async fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }

    async fn updated_at(&self) -> &DateTime<Utc> {
        &self.updated_at
    }
}

#[Object]
impl MaintenanceCadence {
    async fn interval(&self) -> i32 {
        self.interval
    }

    async fn unit(&self) -> &str {
        self.unit.to_str()
    }

    async fn cadence_string(&self) -> String {
        self.to_string()
    }

    async fn days(&self) -> Result<i32, String> {
        self.to_days().map_err(|e| e.to_string())
    }
}
