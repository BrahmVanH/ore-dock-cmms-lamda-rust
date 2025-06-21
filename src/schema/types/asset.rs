use crate::models::{ prelude::* };
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
