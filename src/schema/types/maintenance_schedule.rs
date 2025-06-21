use crate::models::{
    prelude::*,
    maintenance_schedule::{ MaintenanceSchedule, MaintenanceCadence },
};
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
