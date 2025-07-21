//! GraphQL schema implementation for WorkOrder entity.

use crate::models::{
    prelude::*,
    work_order::{
        WorkOrder,
        WorkOrderCost,
        WorkOrderDifficulty,
        WorkOrderPriority,
        WorkOrderSeverity,
        WorkOrderStatus,
        WorkOrderType,
    },
};

/// GraphQL Object implementation for WorkOrder.
#[Object]
impl WorkOrder {
    /// Work order unique identifier.
    async fn id(&self) -> &str {
        &self.id
    }

    /// Human-readable work order number.
    async fn work_order_number(&self) -> &str {
        &self.work_order_number
    }

    /// Work order title.
    async fn title(&self) -> &str {
        &self.title
    }

    /// Detailed work description.
    async fn description(&self) -> &str {
        &self.description
    }

    /// Work order notes.
    async fn notes(&self) -> Option<&str> {
        self.notes.as_deref()
    }

    /// Target asset ID.
    async fn asset_id(&self) -> &str {
        &self.asset_id
    }

    /// Work order type as string representation.
    async fn work_order_type(&self) -> WorkOrderType {
        self.work_order_type
    }

    /// Current status as string representation.
    async fn status(&self) -> WorkOrderStatus {
        self.status
    }

    /// Priority level as string representation.
    async fn priority(&self) -> WorkOrderPriority {
        self.priority
    }

    /// Severity level as string representation.
    async fn severity(&self) -> WorkOrderSeverity {
        self.severity
    }

    /// Severity description.
    async fn severity_description(&self) -> &str {
        self.severity.description()
    }

    /// Severity numeric level.
    async fn severity_level(&self) -> u8 {
        self.severity.numeric_level()
    }

    /// Difficulty level as string representation.
    async fn difficulty(&self) -> WorkOrderDifficulty {
        self.difficulty
    }

    /// Difficulty description.
    async fn difficulty_description(&self) -> &str {
        self.difficulty.description()
    }

    /// Difficulty numeric level.
    async fn difficulty_level(&self) -> u8 {
        self.difficulty.numeric_level()
    }

    /// Primary assigned technician ID.
    async fn assigned_technician_id(&self) -> Option<&str> {
        self.assigned_technician_id.as_deref()
    }

    /// Estimated duration in minutes.
    async fn estimated_duration_minutes(&self) -> i32 {
        self.estimated_duration_minutes
    }

    /// Actual duration in minutes for completed work.
    async fn actual_duration_minutes(&self) -> Option<i32> {
        self.actual_duration_minutes
    }

    /// Estimated cost level as string representation.
    async fn estimated_cost(&self) -> WorkOrderCost {
        self.estimated_cost
    }

    /// Estimated cost description.
    async fn estimated_cost_description(&self) -> &str {
        self.estimated_cost.description()
    }

    /// Estimated cost numeric level.
    async fn estimated_cost_level(&self) -> u8 {
        self.estimated_cost.numeric_level()
    }

    /// Actual total cost as string (preserves decimal precision).
    async fn actual_cost(&self) -> Option<String> {
        self.actual_cost.as_ref().map(|c| c.to_string())
    }

    /// Total labor hours invested in this work order.
    async fn labor_hours(&self) -> Option<f64> {
        self.labor_hours
    }

    /// Notes provided upon work order completion.
    async fn completion_notes(&self) -> Option<&str> {
        self.completion_notes.as_deref()
    }

    /// Date of completion.
    async fn completed_date(&self) -> Option<DateTime<Utc>> {
        self.completed_date.clone()
    }

    /// User who created this work order.
    async fn created_by(&self) -> &str {
        &self.created_by
    }

    /// Creation timestamp.
    async fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }

    /// Last modification timestamp.
    async fn updated_at(&self) -> &DateTime<Utc> {
        &self.updated_at
    }

    /// Computed field: checks if work order is currently in progress.
    #[graphql(name = "is_in_progress")]
    async fn check_is_in_progress(&self) -> bool {
        self.is_in_progress()
    }

    /// Computed field: checks if work order is completed.
    #[graphql(name = "is_completed")]
    async fn check_is_completed(&self) -> bool {
        self.is_completed()
    }

    /// Computed field: checks if work order is overdue.
    #[graphql(name = "is_overdue")]
    async fn check_is_overdue(&self) -> bool {
        self.is_overdue()
    }
}
