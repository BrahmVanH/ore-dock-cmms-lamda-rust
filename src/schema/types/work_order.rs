//! GraphQL schema implementation for WorkOrder entity.
//!
//! This module provides GraphQL field resolvers for the WorkOrder domain model.
//! For detailed field documentation and business logic, see [`crate::models::work_order::WorkOrder`].
//!
//! # GraphQL Specific Notes
//!
//! - All enum fields return string representations via `to_str()`
//! - Optional fields properly handle `None` values as GraphQL nulls
//! - JSON fields are serialized to strings for GraphQL compatibility
//! - Computed fields delegate to business logic methods on the domain model
//! - Decimal fields are converted to strings to preserve precision

use crate::models::{ prelude::*, work_order::{ WorkOrderPriority, WorkOrderType } };

/// GraphQL Object implementation for WorkOrder.
///
/// Field documentation available in [`crate::models::work_order::WorkOrder`].
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

    /// Optional maintenance task ID that generated this work order.
    async fn task_id(&self) -> Option<&str> {
        self.task_id.as_deref()
    }

    /// Target asset ID.
    async fn asset_id(&self) -> &str {
        &self.asset_id
    }

    /// Asset location ID.
    async fn asset_location_id(&self) -> &str {
        &self.asset_location_id
    }

    /// Work order type as string representation.
    async fn work_order_type(&self) -> &str {
        self.work_order_type.to_str()
    }

    /// Current status as string representation.
    async fn status(&self) -> &str {
        self.status.to_str()
    }

    /// Priority level as string representation.
    async fn priority(&self) -> &str {
        self.priority.to_str()
    }

    /// Primary assigned technician ID.
    async fn assigned_technician_id(&self) -> Option<&str> {
        self.assigned_technician_id.as_deref()
    }

    /// List of assigned team IDs.
    async fn assigned_team_ids(&self) -> &Vec<String> {
        &self.assigned_team_ids
    }

    /// User who requested this work order.
    async fn requested_by_user_id(&self) -> &str {
        &self.requested_by_user_id
    }

    /// User who approved this work order.
    async fn approved_by_user_id(&self) -> Option<&str> {
        self.approved_by_user_id.as_deref()
    }

    /// Planned start date and time.
    async fn scheduled_start(&self) -> &DateTime<Utc> {
        &self.scheduled_start
    }

    /// Planned completion date and time.
    async fn scheduled_end(&self) -> Option<&DateTime<Utc>> {
        self.scheduled_end.as_ref()
    }

    /// Actual start time when work began.
    async fn actual_start(&self) -> Option<&DateTime<Utc>> {
        self.actual_start.as_ref()
    }

    /// Actual completion time when work finished.
    async fn actual_end(&self) -> Option<&DateTime<Utc>> {
        self.actual_end.as_ref()
    }

    /// Estimated duration in minutes.
    async fn estimated_duration_minutes(&self) -> i32 {
        self.estimated_duration_minutes
    }

    /// Actual duration in minutes for completed work.
    async fn actual_duration_minutes(&self) -> Option<i32> {
        self.actual_duration_minutes
    }

    /// Estimated total cost as string (preserves decimal precision).
    async fn estimated_cost(&self) -> String {
        self.estimated_cost.to_string()
    }

    /// Actual total cost as string (preserves decimal precision).
    async fn actual_cost(&self) -> Option<String> {
        self.actual_cost.as_ref().map(|c| c.to_string())
    }

    /// Total labor hours invested in this work order.
    async fn labor_hours(&self) -> Option<f64> {
        self.labor_hours
    }

    /// Parts and materials consumed as JSON string.
    async fn parts_used(&self) -> Option<String> {
        self.parts_used.as_ref().and_then(|p| serde_json::to_string(p).ok())
    }

    /// Tools and equipment required.
    async fn tools_required(&self) -> &Vec<String> {
        &self.tools_required
    }

    /// Safety requirements and protocols.
    async fn safety_requirements(&self) -> &Vec<String> {
        &self.safety_requirements
    }

    /// Notes provided upon work order completion.
    async fn completion_notes(&self) -> Option<&str> {
        self.completion_notes.as_deref()
    }

    /// Reason for work order failure or cancellation.
    async fn failure_reason(&self) -> Option<&str> {
        self.failure_reason.as_deref()
    }

    /// Quality rating for completed work (1-5 scale).
    async fn quality_rating(&self) -> Option<i32> {
        self.quality_rating
    }

    /// Customer satisfaction rating (1-5 scale).
    async fn customer_satisfaction(&self) -> Option<i32> {
        self.customer_satisfaction
    }

    /// External vendor assigned to perform this work.
    async fn vendor_id(&self) -> Option<&str> {
        self.vendor_id.as_deref()
    }

    /// Purchase order number for externally contracted work.
    async fn purchase_order_number(&self) -> Option<&str> {
        self.purchase_order_number.as_deref()
    }

    /// Warranty expiration date for completed work.
    async fn warranty_expiration(&self) -> Option<&DateTime<Utc>> {
        self.warranty_expiration.as_ref()
    }

    /// Indicates if follow-up work is required.
    async fn follow_up_required(&self) -> bool {
        self.follow_up_required
    }

    /// Scheduled date for follow-up work or inspection.
    async fn follow_up_date(&self) -> Option<&DateTime<Utc>> {
        self.follow_up_date.as_ref()
    }

    /// URLs to attached documents and files.
    async fn attachment_urls(&self) -> &Vec<String> {
        &self.attachment_urls
    }

    /// Classification tags for categorization.
    async fn tags(&self) -> &Vec<String> {
        &self.tags
    }

    /// Custom fields as JSON string.
    async fn custom_fields(&self) -> Option<String> {
        self.custom_fields.as_ref().and_then(|cf| serde_json::to_string(cf).ok())
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
    /// Delegates to [`WorkOrder::is_in_progress`].
    #[graphql(name = "is_in_progress")]
    async fn check_is_in_progress(&self) -> bool {
        self.is_in_progress()
    }

    /// Computed field: checks if work order is completed.
    /// Delegates to [`WorkOrder::is_completed`].
    #[graphql(name = "is_completed")]
    async fn check_is_completed(&self) -> bool {
        self.is_completed()
    }

    /// Computed field: checks if work order is overdue.
    /// Delegates to [`WorkOrder::is_overdue`].
    #[graphql(name = "is_overdue")]
    async fn check_is_overdue(&self) -> bool {
        self.is_overdue()
    }

    /// Computed field: total cost (actual if available, otherwise estimated).
    /// Delegates to [`WorkOrder::calculate_total_cost`].
    async fn total_cost(&self) -> String {
        self.calculate_total_cost().to_string()
    }

    /// Computed field: variance between estimated and actual duration in minutes.
    async fn duration_variance_minutes(&self) -> Option<i32> {
        self.actual_duration_minutes.map(|actual| actual - self.estimated_duration_minutes)
    }

    /// Computed field: days remaining until scheduled start (minimum 0).
    async fn days_until_scheduled(&self) -> i64 {
        let duration = self.scheduled_start - Utc::now();
        duration.num_days().max(0)
    }

    /// Computed field: checks if this is an emergency work order.
    async fn is_emergency(&self) -> bool {
        matches!(self.priority, WorkOrderPriority::Emergency) ||
            matches!(self.work_order_type, WorkOrderType::Emergency)
    }

    /// Computed field: checks if work is assigned to an external vendor.
    async fn has_vendor(&self) -> bool {
        self.vendor_id.is_some()
    }

    /// Computed field: number of attached documents and files.
    async fn attachment_count(&self) -> i32 {
        self.attachment_urls.len() as i32
    }

    /// Computed field: number of classification tags applied.
    async fn tag_count(&self) -> i32 {
        self.tags.len() as i32
    }

    /// Computed field: number of teams assigned to this work order.
    async fn team_count(&self) -> i32 {
        self.assigned_team_ids.len() as i32
    }
}
