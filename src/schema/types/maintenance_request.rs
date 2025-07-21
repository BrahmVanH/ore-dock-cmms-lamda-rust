use crate::models::{
    maintenance_request::{ MaintenanceRequest, MaintenanceRequestStatus },
    prelude::*,
    work_order::WorkOrderSeverity,
};

#[Object]
impl MaintenanceRequest {
    async fn id(&self) -> &str {
        &self.id
    }

    /// Human-readable work order number.
    async fn submitted_by(&self) -> &str {
        &self.submitted_by
    }

    /// Work order title.
    async fn manager_on_site(&self) -> &str {
        &self.manager_on_site
    }

    /// Detailed work description.
    async fn work_order_ids(&self) -> &Vec<String> {
        &self.work_order_ids
    }

    /// Work order notes.
    async fn read_by_id(&self) -> Option<&str> {
        self.read_by_id.as_deref()
    }

    async fn status(&self) -> MaintenanceRequestStatus {
        self.status
    }
    /// Target asset ID.
    async fn description(&self) -> &str {
        &self.description
    }

    /// Work order type as string representation.

    /// Current status as string representation.
    async fn reported_location(&self) -> &str {
        &self.reported_location
    }

    /// Priority level as string representation.
    async fn troubleshooting_performed(&self) -> &str {
        &self.troubleshooting_performed
    }

    /// Severity level as string representation.
    async fn severity(&self) -> WorkOrderSeverity {
        self.severity
    }

    /// Creation timestamp.
    async fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }

    /// Last modification timestamp.
    async fn updated_at(&self) -> &DateTime<Utc> {
        &self.updated_at
    }
}
