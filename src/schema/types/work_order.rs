
#[Object]
impl WorkOrder {
    async fn id(&self) -> &str {
        &self.id
    }

    async fn work_order_number(&self) -> &str {
        &self.work_order_number
    }

    async fn title(&self) -> &str {
        &self.title
    }

    async fn description(&self) -> &str {
        &self.description
    }

    async fn task_id(&self) -> Option<&str> {
        self.task_id.as_deref()
    }

    async fn asset_id(&self) -> &str {
        &self.asset_id
    }

    async fn asset_location_id(&self) -> &str {
        &self.asset_location_id
    }

    async fn work_order_type(&self) -> &str {
        self.work_order_type.to_str()
    }

    async fn status(&self) -> &str {
        self.status.to_str()
    }

    async fn priority(&self) -> &str {
        self.priority.to_str()
    }

    async fn assigned_technician_id(&self) -> Option<&str> {
        self.assigned_technician_id.as_deref()
    }

    async fn assigned_team_ids(&self) -> &Vec<String> {
        &self.assigned_team_ids
    }

    async fn requested_by_user_id(&self) -> &str {
        &self.requested_by_user_id
    }

    async fn approved_by_user_id(&self) -> Option<&str> {
        self.approved_by_user_id.as_deref()
    }

    async fn scheduled_start(&self) -> &DateTime<Utc> {
        &self.scheduled_start
    }

    async fn scheduled_end(&self) -> Option<&DateTime<Utc>> {
        self.scheduled_end.as_ref()
    }

    async fn actual_start(&self) -> Option<&DateTime<Utc>> {
        self.actual_start.as_ref()
    }

    async fn actual_end(&self) -> Option<&DateTime<Utc>> {
        self.actual_end.as_ref()
    }

    async fn estimated_duration_minutes(&self) -> i32 {
        self.estimated_duration_minutes
    }

    async fn actual_duration_minutes(&self) -> Option<i32> {
        self.actual_duration_minutes
    }

    async fn estimated_cost(&self) -> String {
        self.estimated_cost.to_string()
    }

    async fn actual_cost(&self) -> Option<String> {
        self.actual_cost.as_ref().map(|c| c.to_string())
    }

    async fn labor_hours(&self) -> Option<f64> {
        self.labor_hours
    }

    async fn parts_used(&self) -> Option<String> {
        self.parts_used.as_ref().and_then(|p| serde_json::to_string(p).ok())
    }

    async fn tools_required(&self) -> &Vec<String> {
        &self.tools_required
    }

    async fn safety_requirements(&self) -> &Vec<String> {
        &self.safety_requirements
    }

    async fn completion_notes(&self) -> Option<&str> {
        self.completion_notes.as_deref()
    }

    async fn failure_reason(&self) -> Option<&str> {
        self.failure_reason.as_deref()
    }

    async fn quality_rating(&self) -> Option<i32> {
        self.quality_rating
    }

    async fn customer_satisfaction(&self) -> Option<i32> {
        self.customer_satisfaction
    }

    async fn vendor_id(&self) -> Option<&str> {
        self.vendor_id.as_deref()
    }

    async fn purchase_order_number(&self) -> Option<&str> {
        self.purchase_order_number.as_deref()
    }

    async fn warranty_expiration(&self) -> Option<&DateTime<Utc>> {
        self.warranty_expiration.as_ref()
    }

    async fn follow_up_required(&self) -> bool {
        self.follow_up_required
    }

    async fn follow_up_date(&self) -> Option<&DateTime<Utc>> {
        self.follow_up_date.as_ref()
    }

    async fn attachment_urls(&self) -> &Vec<String> {
        &self.attachment_urls
    }

    async fn tags(&self) -> &Vec<String> {
        &self.tags
    }

    async fn custom_fields(&self) -> Option<String> {
        self.custom_fields.as_ref().and_then(|cf| serde_json::to_string(cf).ok())
    }

    async fn created_by(&self) -> &str {
        &self.created_by
    }

    async fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }

    async fn updated_at(&self) -> &DateTime<Utc> {
        &self.updated_at
    }

    async fn is_in_progress(&self) -> bool {
        self.is_in_progress()
    }

    async fn is_completed(&self) -> bool {
        self.is_completed()
    }

    async fn is_overdue(&self) -> bool {
        self.is_overdue()
    }

    async fn total_cost(&self) -> String {
        self.calculate_total_cost().to_string()
    }

    async fn duration_variance_minutes(&self) -> Option<i32> {
        self.actual_duration_minutes.map(|actual| actual - self.estimated_duration_minutes)
    }

    async fn days_until_scheduled(&self) -> i64 {
        let duration = self.scheduled_start - Utc::now();
        duration.num_days().max(0)
    }

    async fn is_emergency(&self) -> bool {
        matches!(self.priority, WorkOrderPriority::Emergency) || 
        matches!(self.work_order_type, WorkOrderType::Emergency)
    }

    async fn has_vendor(&self) -> bool {
        self.vendor_id.is_some()
    }

    async fn attachment_count(&self) -> i32 {
        self.attachment_urls.len() as i32
    }

    async fn tag_count(&self) -> i32 {
        self.tags.len() as i32
    }

    async fn team_count(&self) -> i32 {
        self.assigned_team_ids.len() as i32
    }
}