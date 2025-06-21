use std::collections::HashMap;

use aws_sdk_dynamodb::types::AttributeValue;
use chrono::{ DateTime, Utc };
use rust_decimal::Decimal;
use serde::{ Deserialize, Serialize };
use serde_json::Value as Json;
use tracing::info;

use crate::error::AppError;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkOrderStatus {
    Draft, // Draft work order not yet scheduled
    Scheduled, // Scheduled but not started
    InProgress, // Currently being worked on
    OnHold, // Temporarily paused
    Completed, // Work completed successfully
    Cancelled, // Work order cancelled
    Failed, // Work failed and needs to be rescheduled
    Deferred, // Deferred to a later time
    WaitingParts, // Waiting for parts/materials
    WaitingApproval, // Waiting for approval to proceed
}

impl WorkOrderStatus {
    pub(crate) fn to_str(&self) -> &str {
        match self {
            WorkOrderStatus::Draft => "draft",
            WorkOrderStatus::Scheduled => "scheduled",
            WorkOrderStatus::InProgress => "in_progress",
            WorkOrderStatus::OnHold => "on_hold",
            WorkOrderStatus::Completed => "completed",
            WorkOrderStatus::Cancelled => "cancelled",
            WorkOrderStatus::Failed => "failed",
            WorkOrderStatus::Deferred => "deferred",
            WorkOrderStatus::WaitingParts => "waiting_parts",
            WorkOrderStatus::WaitingApproval => "waiting_approval",
        }
    }

    pub(crate) fn to_string(&self) -> String {
        self.to_str().to_string()
    }

    pub(crate) fn from_string(s: &str) -> Result<WorkOrderStatus, AppError> {
        match s {
            "draft" => Ok(Self::Draft),
            "scheduled" => Ok(Self::Scheduled),
            "in_progress" => Ok(Self::InProgress),
            "on_hold" => Ok(Self::OnHold),
            "completed" => Ok(Self::Completed),
            "cancelled" => Ok(Self::Cancelled),
            "failed" => Ok(Self::Failed),
            "deferred" => Ok(Self::Deferred),
            "waiting_parts" => Ok(Self::WaitingParts),
            "waiting_approval" => Ok(Self::WaitingApproval),
            _ => Err(AppError::ValidationError("Invalid work order status".to_string())),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkOrderPriority {
    Low, // Low priority, can be scheduled later
    Normal, // Normal priority
    High, // High priority, should be addressed soon
    Urgent, // Urgent, needs immediate attention
    Emergency, // Emergency, critical safety or operational issue
}

impl WorkOrderPriority {
    pub(crate) fn to_str(&self) -> &str {
        match self {
            WorkOrderPriority::Low => "low",
            WorkOrderPriority::Normal => "normal",
            WorkOrderPriority::High => "high",
            WorkOrderPriority::Urgent => "urgent",
            WorkOrderPriority::Emergency => "emergency",
        }
    }

    pub(crate) fn to_string(&self) -> String {
        self.to_str().to_string()
    }

    pub(crate) fn from_string(s: &str) -> Result<WorkOrderPriority, AppError> {
        match s {
            "low" => Ok(Self::Low),
            "normal" => Ok(Self::Normal),
            "high" => Ok(Self::High),
            "urgent" => Ok(Self::Urgent),
            "emergency" => Ok(Self::Emergency),
            _ => Err(AppError::ValidationError("Invalid work order priority".to_string())),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkOrderType {
    Preventive, // Preventive maintenance
    Corrective, // Corrective/repair work
    Emergency, // Emergency repairs
    Inspection, // Inspection work
    Calibration, // Equipment calibration
    Installation, // New equipment installation
    Upgrade, // Equipment upgrades
    Replacement, // Equipment replacement
    Cleaning, // Cleaning/housekeeping
    Safety, // Safety-related work
}

impl WorkOrderType {
    pub(crate) fn to_str(&self) -> &str {
        match self {
            WorkOrderType::Preventive => "preventive",
            WorkOrderType::Corrective => "corrective",
            WorkOrderType::Emergency => "emergency",
            WorkOrderType::Inspection => "inspection",
            WorkOrderType::Calibration => "calibration",
            WorkOrderType::Installation => "installation",
            WorkOrderType::Upgrade => "upgrade",
            WorkOrderType::Replacement => "replacement",
            WorkOrderType::Cleaning => "cleaning",
            WorkOrderType::Safety => "safety",
        }
    }

    pub(crate) fn to_string(&self) -> String {
        self.to_str().to_string()
    }

    pub(crate) fn from_string(s: &str) -> Result<WorkOrderType, AppError> {
        match s {
            "preventive" => Ok(Self::Preventive),
            "corrective" => Ok(Self::Corrective),
            "emergency" => Ok(Self::Emergency),
            "inspection" => Ok(Self::Inspection),
            "calibration" => Ok(Self::Calibration),
            "installation" => Ok(Self::Installation),
            "upgrade" => Ok(Self::Upgrade),
            "replacement" => Ok(Self::Replacement),
            "cleaning" => Ok(Self::Cleaning),
            "safety" => Ok(Self::Safety),
            _ => Err(AppError::ValidationError("Invalid work order type".to_string())),
        }
    }
}

/// Represents a Work Order in the system
///
/// # Fields
///
/// * `id` - Unique identifier for the work order
/// * `work_order_number` - Human-readable work order number
/// * `title` - Brief title/description of the work
/// * `description` - Detailed description of the work to be performed
/// * `task_id` - Optional ID of the maintenance task this is based on
/// * `asset_id` - ID of the asset this work order is for
/// * `asset_location_id` - Location ID of the asset
/// * `work_order_type` - Type of work order (preventive, corrective, etc.)
/// * `status` - Current status of the work order
/// * `priority` - Priority level of the work order
/// * `assigned_technician_id` - ID of the primary assigned technician
/// * `assigned_team_ids` - List of team IDs assigned to this work order
/// * `requested_by_user_id` - ID of the user who requested this work
/// * `approved_by_user_id` - ID of the user who approved this work
/// * `scheduled_start` - Scheduled start date and time
/// * `scheduled_end` - Scheduled completion date and time
/// * `actual_start` - Actual start date and time
/// * `actual_end` - Actual completion date and time
/// * `estimated_duration_minutes` - Estimated duration in minutes
/// * `actual_duration_minutes` - Actual duration in minutes
/// * `estimated_cost` - Estimated total cost
/// * `actual_cost` - Actual total cost
/// * `labor_hours` - Total labor hours spent
/// * `parts_used` - JSON array of parts/materials used
/// * `tools_required` - List of tools required for the work
/// * `safety_requirements` - Safety requirements and procedures
/// * `completion_notes` - Notes upon completion
/// * `failure_reason` - Reason if work failed
/// * `quality_rating` - Quality rating of completed work (1-5)
/// * `customer_satisfaction` - Customer satisfaction rating (1-5)
/// * `vendor_id` - ID of external vendor if outsourced
/// * `purchase_order_number` - PO number for external work
/// * `warranty_expiration` - Warranty expiration for completed work
/// * `follow_up_required` - Whether follow-up work is required
/// * `follow_up_date` - Date for follow-up work
/// * `attachment_urls` - List of attachment URLs (photos, documents)
/// * `tags` - List of tags for categorization
/// * `custom_fields` - Custom fields as JSON
/// * `created_by` - User who created this work order
/// * `created_at` - Date and time of creation
/// * `updated_at` - Date and time of last update
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WorkOrder {
    pub id: String,
    pub work_order_number: String,
    pub title: String,
    pub description: String,
    pub task_id: Option<String>,
    pub asset_id: String,
    pub asset_location_id: String,
    pub work_order_type: WorkOrderType,
    pub status: WorkOrderStatus,
    pub priority: WorkOrderPriority,
    pub assigned_technician_id: Option<String>,
    pub assigned_team_ids: Vec<String>,
    pub requested_by_user_id: String,
    pub approved_by_user_id: Option<String>,
    pub scheduled_start: DateTime<Utc>,
    pub scheduled_end: Option<DateTime<Utc>>,
    pub actual_start: Option<DateTime<Utc>>,
    pub actual_end: Option<DateTime<Utc>>,
    pub estimated_duration_minutes: i32,
    pub actual_duration_minutes: Option<i32>,
    pub estimated_cost: Decimal,
    pub actual_cost: Option<Decimal>,
    pub labor_hours: Option<f64>,
    pub parts_used: Option<Json>,
    pub tools_required: Vec<String>,
    pub safety_requirements: Vec<String>,
    pub completion_notes: Option<String>,
    pub failure_reason: Option<String>,
    pub quality_rating: Option<i32>,
    pub customer_satisfaction: Option<i32>,
    pub vendor_id: Option<String>,
    pub purchase_order_number: Option<String>,
    pub warranty_expiration: Option<DateTime<Utc>>,
    pub follow_up_required: bool,
    pub follow_up_date: Option<DateTime<Utc>>,
    pub attachment_urls: Vec<String>,
    pub tags: Vec<String>,
    pub custom_fields: Option<Json>,
    pub created_by: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Defines methods for WorkOrder
impl WorkOrder {
    /// Creates new WorkOrder instance
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier
    /// * `work_order_number` - Work order number
    /// * `title` - Brief title
    /// * `description` - Detailed description
    /// * `task_id` - Optional task ID
    /// * `asset_id` - Asset ID
    /// * `asset_location_id` - Asset location ID
    /// * `work_order_type` - Type as string
    /// * `priority` - Priority as string
    /// * `assigned_technician_id` - Optional technician ID
    /// * `assigned_team_ids` - List of team IDs
    /// * `requested_by_user_id` - Requester user ID
    /// * `scheduled_start` - Scheduled start time
    /// * `scheduled_end` - Optional scheduled end time
    /// * `estimated_duration_minutes` - Estimated duration
    /// * `estimated_cost` - Estimated cost
    /// * `tools_required` - List of required tools
    /// * `safety_requirements` - List of safety requirements
    /// * `follow_up_required` - Whether follow-up is needed
    /// * `follow_up_date` - Optional follow-up date
    /// * `attachment_urls` - List of attachment URLs
    /// * `tags` - List of tags
    /// * `custom_fields` - Optional custom fields JSON
    /// * `created_by` - Creator user ID
    ///
    /// # Returns
    ///
    /// New WorkOrder instance
    pub fn new(
        id: String,
        work_order_number: String,
        title: String,
        description: String,
        task_id: Option<String>,
        asset_id: String,
        asset_location_id: String,
        work_order_type: String,
        priority: String,
        assigned_technician_id: Option<String>,
        assigned_team_ids: Vec<String>,
        requested_by_user_id: String,
        scheduled_start: DateTime<Utc>,
        scheduled_end: Option<DateTime<Utc>>,
        estimated_duration_minutes: i32,
        estimated_cost: Decimal,
        tools_required: Vec<String>,
        safety_requirements: Vec<String>,
        follow_up_required: bool,
        follow_up_date: Option<DateTime<Utc>>,
        attachment_urls: Vec<String>,
        tags: Vec<String>,
        custom_fields: Option<Json>,
        created_by: String
    ) -> Result<Self, AppError> {
        let now = Utc::now();

        // Validate required fields
        if work_order_number.trim().is_empty() {
            return Err(AppError::ValidationError("Work order number cannot be empty".to_string()));
        }

        if title.trim().is_empty() {
            return Err(AppError::ValidationError("Title cannot be empty".to_string()));
        }

        if description.trim().is_empty() {
            return Err(AppError::ValidationError("Description cannot be empty".to_string()));
        }

        if asset_id.trim().is_empty() {
            return Err(AppError::ValidationError("Asset ID cannot be empty".to_string()));
        }

        if asset_location_id.trim().is_empty() {
            return Err(AppError::ValidationError("Asset location ID cannot be empty".to_string()));
        }

        if requested_by_user_id.trim().is_empty() {
            return Err(
                AppError::ValidationError("Requested by user ID cannot be empty".to_string())
            );
        }

        if created_by.trim().is_empty() {
            return Err(AppError::ValidationError("Created by cannot be empty".to_string()));
        }

        // Validate estimated duration
        if estimated_duration_minutes <= 0 {
            return Err(
                AppError::ValidationError("Estimated duration must be positive".to_string())
            );
        }

        // Validate estimated cost
        if estimated_cost < Decimal::new(0, 0) {
            return Err(AppError::ValidationError("Estimated cost cannot be negative".to_string()));
        }

        // Validate scheduled dates
        if let Some(end) = &scheduled_end {
            if *end <= scheduled_start {
                return Err(
                    AppError::ValidationError(
                        "Scheduled end must be after scheduled start".to_string()
                    )
                );
            }
        }

        let work_order_type_enum = WorkOrderType::from_string(&work_order_type)?;
        let priority_enum = WorkOrderPriority::from_string(&priority)?;

        Ok(Self {
            id,
            work_order_number,
            title,
            description,
            task_id,
            asset_id,
            asset_location_id,
            work_order_type: work_order_type_enum,
            status: WorkOrderStatus::Draft,
            priority: priority_enum,
            assigned_technician_id,
            assigned_team_ids,
            requested_by_user_id,
            approved_by_user_id: None,
            scheduled_start,
            scheduled_end,
            actual_start: None,
            actual_end: None,
            estimated_duration_minutes,
            actual_duration_minutes: None,
            estimated_cost,
            actual_cost: None,
            labor_hours: None,
            parts_used: None,
            tools_required,
            safety_requirements,
            completion_notes: None,
            failure_reason: None,
            quality_rating: None,
            customer_satisfaction: None,
            vendor_id: None,
            purchase_order_number: None,
            warranty_expiration: None,
            follow_up_required,
            follow_up_date,
            attachment_urls,
            tags,
            custom_fields,
            created_by,
            created_at: now,
            updated_at: now,
        })
    }

    /// Creates WorkOrder instance from DynamoDB item
    ///
    /// # Arguments
    ///
    /// * `item` - The dynamo db item
    ///
    /// # Returns
    ///
    /// 'Some' WorkOrder if item fields match, 'None' otherwise
    pub(crate) fn from_item(item: &HashMap<String, AttributeValue>) -> Option<Self> {
        info!("calling from_item with: {:?}", &item);

        let id = item.get("id")?.as_s().ok()?.to_string();
        let work_order_number = item.get("work_order_number")?.as_s().ok()?.to_string();
        let title = item.get("title")?.as_s().ok()?.to_string();
        let description = item.get("description")?.as_s().ok()?.to_string();

        let task_id = item
            .get("task_id")
            .and_then(|v| v.as_s().ok())
            .map(|s| s.to_string());

        let asset_id = item.get("asset_id")?.as_s().ok()?.to_string();
        let asset_location_id = item.get("asset_location_id")?.as_s().ok()?.to_string();

        let work_order_type_str = item.get("work_order_type")?.as_s().ok()?;
        let work_order_type = WorkOrderType::from_string(&work_order_type_str)
            .map_err(|e| e)
            .ok()?;

        let status_str = item.get("status")?.as_s().ok()?;
        let status = WorkOrderStatus::from_string(&status_str)
            .map_err(|e| e)
            .ok()?;

        let priority_str = item.get("priority")?.as_s().ok()?;
        let priority = WorkOrderPriority::from_string(&priority_str)
            .map_err(|e| e)
            .ok()?;

        let assigned_technician_id = item
            .get("assigned_technician_id")
            .and_then(|v| v.as_s().ok())
            .map(|s| s.to_string());

        let assigned_team_ids = item
            .get("assigned_team_ids")
            .and_then(|v| v.as_ss().ok())
            .map(|ids| {
                ids.iter()
                    .map(|s| s.to_string())
                    .collect()
            })
            .unwrap_or_default();

        let requested_by_user_id = item.get("requested_by_user_id")?.as_s().ok()?.to_string();

        let approved_by_user_id = item
            .get("approved_by_user_id")
            .and_then(|v| v.as_s().ok())
            .map(|s| s.to_string());

        let scheduled_start = item
            .get("scheduled_start")
            .and_then(|v| v.as_s().ok())
            .and_then(|s| s.parse::<DateTime<Utc>>().ok())
            .unwrap_or_else(|| Utc::now());

        let scheduled_end = item
            .get("scheduled_end")
            .and_then(|v| v.as_s().ok())
            .and_then(|s| s.parse::<DateTime<Utc>>().ok());

        let actual_start = item
            .get("actual_start")
            .and_then(|v| v.as_s().ok())
            .and_then(|s| s.parse::<DateTime<Utc>>().ok());

        let actual_end = item
            .get("actual_end")
            .and_then(|v| v.as_s().ok())
            .and_then(|s| s.parse::<DateTime<Utc>>().ok());

        let estimated_duration_minutes = item
            .get("estimated_duration_minutes")
            .and_then(|v| v.as_n().ok())
            .and_then(|s| s.parse::<i32>().ok())
            .unwrap_or(60);

        let actual_duration_minutes = item
            .get("actual_duration_minutes")
            .and_then(|v| v.as_n().ok())
            .and_then(|s| s.parse::<i32>().ok());

        let estimated_cost = item
            .get("estimated_cost")
            .and_then(|v| v.as_s().ok())
            .and_then(|s| s.parse::<Decimal>().ok())
            .unwrap_or_else(|| Decimal::new(0, 0));

        let actual_cost = item
            .get("actual_cost")
            .and_then(|v| v.as_s().ok())
            .and_then(|s| s.parse::<Decimal>().ok());

        let labor_hours = item
            .get("labor_hours")
            .and_then(|v| v.as_n().ok())
            .and_then(|s| s.parse::<f64>().ok());

        let parts_used = item
            .get("parts_used")
            .and_then(|v| v.as_s().ok())
            .and_then(|s| serde_json::from_str::<Json>(s).ok());

        let tools_required = item
            .get("tools_required")
            .and_then(|v| v.as_ss().ok())
            .map(|tools| {
                tools
                    .iter()
                    .map(|s| s.to_string())
                    .collect()
            })
            .unwrap_or_default();

        let safety_requirements = item
            .get("safety_requirements")
            .and_then(|v| v.as_ss().ok())
            .map(|reqs| {
                reqs.iter()
                    .map(|s| s.to_string())
                    .collect()
            })
            .unwrap_or_default();

        let completion_notes = item
            .get("completion_notes")
            .and_then(|v| v.as_s().ok())
            .map(|s| s.to_string());

        let failure_reason = item
            .get("failure_reason")
            .and_then(|v| v.as_s().ok())
            .map(|s| s.to_string());

        let quality_rating = item
            .get("quality_rating")
            .and_then(|v| v.as_n().ok())
            .and_then(|s| s.parse::<i32>().ok());

        let customer_satisfaction = item
            .get("customer_satisfaction")
            .and_then(|v| v.as_n().ok())
            .and_then(|s| s.parse::<i32>().ok());

        let vendor_id = item
            .get("vendor_id")
            .and_then(|v| v.as_s().ok())
            .map(|s| s.to_string());

        let purchase_order_number = item
            .get("purchase_order_number")
            .and_then(|v| v.as_s().ok())
            .map(|s| s.to_string());

        let warranty_expiration = item
            .get("warranty_expiration")
            .and_then(|v| v.as_s().ok())
            .and_then(|s| s.parse::<DateTime<Utc>>().ok());

        let follow_up_required = item
            .get("follow_up_required")
            .and_then(|v| v.as_bool().ok())
            .unwrap_or(&false);

        let follow_up_date = item
            .get("follow_up_date")
            .and_then(|v| v.as_s().ok())
            .and_then(|s| s.parse::<DateTime<Utc>>().ok());

        let attachment_urls = item
            .get("attachment_urls")
            .and_then(|v| v.as_ss().ok())
            .map(|urls| {
                urls.iter()
                    .map(|s| s.to_string())
                    .collect()
            })
            .unwrap_or_default();

        let tags = item
            .get("tags")
            .and_then(|v| v.as_ss().ok())
            .map(|tag_list| {
                tag_list
                    .iter()
                    .map(|s| s.to_string())
                    .collect()
            })
            .unwrap_or_default();

        let custom_fields = item
            .get("custom_fields")
            .and_then(|v| v.as_s().ok())
            .and_then(|s| serde_json::from_str::<Json>(s).ok());

        let created_by = item.get("created_by")?.as_s().ok()?.to_string();

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
            work_order_number,
            title,
            description,
            task_id,
            asset_id,
            asset_location_id,
            work_order_type,
            status,
            priority,
            assigned_technician_id,
            assigned_team_ids,
            requested_by_user_id,
            approved_by_user_id,
            scheduled_start,
            scheduled_end,
            actual_start,
            actual_end,
            estimated_duration_minutes,
            actual_duration_minutes,
            estimated_cost,
            actual_cost,
            labor_hours,
            parts_used,
            tools_required,
            safety_requirements,
            completion_notes,
            failure_reason,
            quality_rating,
            customer_satisfaction,
            vendor_id,
            purchase_order_number,
            warranty_expiration,
            follow_up_required: *follow_up_required,
            follow_up_date,
            attachment_urls,
            tags,
            custom_fields,
            created_by,
            created_at,
            updated_at,
        });

        info!("result of from_item on work_order: {:?}", res);
        res
    }

    /// Creates DynamoDB item from WorkOrder instance
    ///
    /// # Arguments
    ///
    /// * `self` - borrowed instance of self
    ///
    /// # Returns
    ///
    /// HashMap representing DB item for WorkOrder instance
    pub(crate) fn to_item(&self) -> HashMap<String, AttributeValue> {
        let mut item = HashMap::new();

        item.insert("id".to_string(), AttributeValue::S(self.id.clone()));
        item.insert(
            "work_order_number".to_string(),
            AttributeValue::S(self.work_order_number.clone())
        );
        item.insert("title".to_string(), AttributeValue::S(self.title.clone()));
        item.insert("description".to_string(), AttributeValue::S(self.description.clone()));

        if let Some(task_id) = &self.task_id {
            item.insert("task_id".to_string(), AttributeValue::S(task_id.clone()));
        }

        item.insert("asset_id".to_string(), AttributeValue::S(self.asset_id.clone()));
        item.insert(
            "asset_location_id".to_string(),
            AttributeValue::S(self.asset_location_id.clone())
        );
        item.insert(
            "work_order_type".to_string(),
            AttributeValue::S(self.work_order_type.to_str().to_string())
        );
        item.insert("status".to_string(), AttributeValue::S(self.status.to_str().to_string()));
        item.insert("priority".to_string(), AttributeValue::S(self.priority.to_str().to_string()));

        if let Some(tech_id) = &self.assigned_technician_id {
            item.insert("assigned_technician_id".to_string(), AttributeValue::S(tech_id.clone()));
        }

        if !self.assigned_team_ids.is_empty() {
            item.insert(
                "assigned_team_ids".to_string(),
                AttributeValue::Ss(self.assigned_team_ids.clone())
            );
        }

        item.insert(
            "requested_by_user_id".to_string(),
            AttributeValue::S(self.requested_by_user_id.clone())
        );

        if let Some(approved_by) = &self.approved_by_user_id {
            item.insert("approved_by_user_id".to_string(), AttributeValue::S(approved_by.clone()));
        }

        item.insert(
            "scheduled_start".to_string(),
            AttributeValue::S(self.scheduled_start.to_string())
        );

        if let Some(scheduled_end) = &self.scheduled_end {
            item.insert("scheduled_end".to_string(), AttributeValue::S(scheduled_end.to_string()));
        }

        if let Some(actual_start) = &self.actual_start {
            item.insert("actual_start".to_string(), AttributeValue::S(actual_start.to_string()));
        }

        if let Some(actual_end) = &self.actual_end {
            item.insert("actual_end".to_string(), AttributeValue::S(actual_end.to_string()));
        }

        item.insert(
            "estimated_duration_minutes".to_string(),
            AttributeValue::N(self.estimated_duration_minutes.to_string())
        );

        if let Some(actual_duration) = &self.actual_duration_minutes {
            item.insert(
                "actual_duration_minutes".to_string(),
                AttributeValue::N(actual_duration.to_string())
            );
        }

        item.insert(
            "estimated_cost".to_string(),
            AttributeValue::S(self.estimated_cost.to_string())
        );

        if let Some(actual_cost) = &self.actual_cost {
            item.insert("actual_cost".to_string(), AttributeValue::S(actual_cost.to_string()));
        }

        if let Some(labor) = &self.labor_hours {
            item.insert("labor_hours".to_string(), AttributeValue::N(labor.to_string()));
        }

        if let Some(parts) = &self.parts_used {
            if let Ok(parts_json) = serde_json::to_string(parts) {
                item.insert("parts_used".to_string(), AttributeValue::S(parts_json));
            }
        }

        if !self.tools_required.is_empty() {
            item.insert(
                "tools_required".to_string(),
                AttributeValue::Ss(self.tools_required.clone())
            );
        }

        if !self.safety_requirements.is_empty() {
            item.insert(
                "safety_requirements".to_string(),
                AttributeValue::Ss(self.safety_requirements.clone())
            );
        }

        if let Some(notes) = &self.completion_notes {
            item.insert("completion_notes".to_string(), AttributeValue::S(notes.clone()));
        }

        if let Some(reason) = &self.failure_reason {
            item.insert("failure_reason".to_string(), AttributeValue::S(reason.clone()));
        }

        if let Some(rating) = &self.quality_rating {
            item.insert("quality_rating".to_string(), AttributeValue::N(rating.to_string()));
        }

        if let Some(satisfaction) = &self.customer_satisfaction {
            item.insert(
                "customer_satisfaction".to_string(),
                AttributeValue::N(satisfaction.to_string())
            );
        }

        if let Some(vendor) = &self.vendor_id {
            item.insert("vendor_id".to_string(), AttributeValue::S(vendor.clone()));
        }

        if let Some(po) = &self.purchase_order_number {
            item.insert("purchase_order_number".to_string(), AttributeValue::S(po.clone()));
        }

        if let Some(warranty) = &self.warranty_expiration {
            item.insert("warranty_expiration".to_string(), AttributeValue::S(warranty.to_string()));
        }

        item.insert(
            "follow_up_required".to_string(),
            AttributeValue::Bool(self.follow_up_required)
        );

        if let Some(follow_up) = &self.follow_up_date {
            item.insert("follow_up_date".to_string(), AttributeValue::S(follow_up.to_string()));
        }

        if !self.attachment_urls.is_empty() {
            item.insert(
                "attachment_urls".to_string(),
                AttributeValue::Ss(self.attachment_urls.clone())
            );
        }

        if !self.tags.is_empty() {
            item.insert("tags".to_string(), AttributeValue::Ss(self.tags.clone()));
        }

        if let Some(custom) = &self.custom_fields {
            if let Ok(custom_json) = serde_json::to_string(custom) {
                item.insert("custom_fields".to_string(), AttributeValue::S(custom_json));
            }
        }

        item.insert("created_by".to_string(), AttributeValue::S(self.created_by.clone()));
        item.insert("created_at".to_string(), AttributeValue::S(self.created_at.to_string()));
        item.insert("updated_at".to_string(), AttributeValue::S(self.updated_at.to_string()));

        item
    }

    /// Checks if the work order is currently in progress
    pub fn is_in_progress(&self) -> bool {
        matches!(self.status, WorkOrderStatus::InProgress)
    }

    /// Checks if the work order is completed
    pub fn is_completed(&self) -> bool {
        matches!(self.status, WorkOrderStatus::Completed)
    }

    /// Checks if the work order is overdue
    pub fn is_overdue(&self) -> bool {
        if let Some(scheduled_end) = &self.scheduled_end {
            Utc::now() > *scheduled_end && !self.is_completed()
        } else {
            false
        }
    }

    /// Starts the work order
    pub fn start_work(&mut self, technician_id: String) -> Result<(), AppError> {
        if !matches!(self.status, WorkOrderStatus::Scheduled) {
            return Err(
                AppError::ValidationError("Only scheduled work orders can be started".to_string())
            );
        }

        self.status = WorkOrderStatus::InProgress;
        self.actual_start = Some(Utc::now());
        self.assigned_technician_id = Some(technician_id);
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Completes the work order
    pub fn complete_work(
        &mut self,
        completion_notes: Option<String>,
        quality_rating: Option<i32>
    ) -> Result<(), AppError> {
        if !matches!(self.status, WorkOrderStatus::InProgress) {
            return Err(
                AppError::ValidationError(
                    "Only in-progress work orders can be completed".to_string()
                )
            );
        }

        // Validate quality rating
        if let Some(rating) = quality_rating {
            if rating < 1 || rating > 5 {
                return Err(
                    AppError::ValidationError("Quality rating must be between 1 and 5".to_string())
                );
            }
        }

        let now = Utc::now();
        self.status = WorkOrderStatus::Completed;
        self.actual_end = Some(now);
        self.completion_notes = completion_notes;
        self.quality_rating = quality_rating;

        // Calculate actual duration if we have actual start time
        if let Some(start) = &self.actual_start {
            let duration = now - *start;
            self.actual_duration_minutes = Some(duration.num_minutes() as i32);
        }

        self.updated_at = now;
        Ok(())
    }

    /// Cancels the work order
    pub fn cancel_work(&mut self, reason: String) -> Result<(), AppError> {
        if matches!(self.status, WorkOrderStatus::Completed | WorkOrderStatus::Cancelled) {
            return Err(
                AppError::ValidationError(
                    "Cannot cancel completed or already cancelled work order".to_string()
                )
            );
        }

        self.status = WorkOrderStatus::Cancelled;
        self.failure_reason = Some(reason);
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Puts the work order on hold
    pub fn put_on_hold(&mut self, reason: String) -> Result<(), AppError> {
        if !matches!(self.status, WorkOrderStatus::InProgress | WorkOrderStatus::Scheduled) {
            return Err(
                AppError::ValidationError(
                    "Only scheduled or in-progress work orders can be put on hold".to_string()
                )
            );
        }

        self.status = WorkOrderStatus::OnHold;
        self.failure_reason = Some(reason);
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Resumes work from hold
    pub fn resume_from_hold(&mut self) -> Result<(), AppError> {
        if !matches!(self.status, WorkOrderStatus::OnHold) {
            return Err(
                AppError::ValidationError("Only work orders on hold can be resumed".to_string())
            );
        }

        self.status = if self.actual_start.is_some() {
            WorkOrderStatus::InProgress
        } else {
            WorkOrderStatus::Scheduled
        };
        self.failure_reason = None;
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Calculates the total cost including labor and parts
    pub fn calculate_total_cost(&self) -> Decimal {
        if let Some(actual) = &self.actual_cost {
            actual.clone()
        } else {
            self.estimated_cost.clone()
        }
    }

    /// Adds a tag to the work order
    pub fn add_tag(&mut self, tag: String) {
        if !self.tags.contains(&tag) {
            self.tags.push(tag);
            self.updated_at = Utc::now();
        }
    }

    /// Removes a tag from the work order
    pub fn remove_tag(&mut self, tag: &str) {
        if let Some(pos) = self.tags.iter().position(|x| x == tag) {
            self.tags.remove(pos);
            self.updated_at = Utc::now();
        }
    }
}
