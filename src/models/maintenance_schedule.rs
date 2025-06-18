use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::error::AppError;


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MaintenanceSchedule {
    pub id: String,
    pub asset_id: String,
    pub cadence: String,
    pub last_completed_at: Option<DateTime<Utc>>,
    pub last_completed_by_user_id: Option<String>,
    pub next_due_at: DateTime<Utc>,
    pub duration_estimate: Option<i32>,
    pub recurring: bool,
    pub active: bool,
}


impl MaintenanceSchedule {
    pub fn new(
        id: String,
        asset_id: String, 
        cadence: String,
        last_completed_at: Option<DateTime<Utc>>,
        last_completed_by_user_id: Option<String>,
         next_due_at: DateTime<Utc>,
        duration_estimate: Option<i32>,
        recurring: bool,
        active: bool,
    ) -> Result<Self, AppError> {
        
    }
}