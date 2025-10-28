use std::collections::HashMap;

use async_graphql::Enum;
use aws_sdk_dynamodb::types::AttributeValue;
use chrono::{ DateTime, Utc };
use rust_decimal::Decimal;
use serde::{ Deserialize, Serialize };
use tracing::info;

use crate::{ error::AppError, DynamoDbEntity };

#[derive(Enum, Copy, Clone, PartialEq, Eq, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkOrderSeverity {
    Critical,
    Important,
    Valuable,
    Nice,
}

impl WorkOrderSeverity {
    pub(crate) fn to_str(&self) -> &str {
        match self {
            WorkOrderSeverity::Critical => "critical",
            WorkOrderSeverity::Important => "important",
            WorkOrderSeverity::Valuable => "valuable",
            WorkOrderSeverity::Nice => "nice",
        }
    }

    pub(crate) fn to_string(&self) -> String {
        self.to_str().to_string()
    }

    pub(crate) fn from_string(s: &str) -> Result<WorkOrderSeverity, AppError> {
        match s {
            "critical" => Ok(Self::Critical),
            "important" => Ok(Self::Important),
            "valuable" => Ok(Self::Valuable),
            "nice" => Ok(Self::Nice),
            _ => Err(AppError::ValidationError("Invalid work order severity".to_string())),
        }
    }

    pub fn description(&self) -> &str {
        match self {
            WorkOrderSeverity::Critical => "Critical to Safety / Property Damage",
            WorkOrderSeverity::Important => "Important to facilitate sales",
            WorkOrderSeverity::Valuable => "Valuable to allow job function",
            WorkOrderSeverity::Nice => "Nice to have addressed",
        }
    }

    pub fn numeric_level(&self) -> u8 {
        match self {
            WorkOrderSeverity::Critical => 1,
            WorkOrderSeverity::Important => 2,
            WorkOrderSeverity::Valuable => 3,
            WorkOrderSeverity::Nice => 4,
        }
    }
}

#[derive(Enum, Copy, Clone, PartialEq, Eq, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkOrderDifficulty {
    Normal,
    Extended,
    Advanced,
    HireOut,
}

impl WorkOrderDifficulty {
    pub(crate) fn to_str(&self) -> &str {
        match self {
            WorkOrderDifficulty::Normal => "normal",
            WorkOrderDifficulty::Extended => "extended",
            WorkOrderDifficulty::Advanced => "advanced",
            WorkOrderDifficulty::HireOut => "hireOut",
        }
    }

    pub(crate) fn to_string(&self) -> String {
        self.to_str().to_string()
    }

    pub(crate) fn from_string(s: &str) -> Result<WorkOrderDifficulty, AppError> {
        match s {
            "normal" => Ok(Self::Normal),
            "extended" => Ok(Self::Extended),
            "advanced" => Ok(Self::Advanced),
            "hireOut" => Ok(Self::HireOut),
            _ => Err(AppError::ValidationError("Invalid work order difficulty".to_string())),
        }
    }

    pub fn description(&self) -> &str {
        match self {
            WorkOrderDifficulty::Normal => "Anticipated to be within normal maintenance",
            WorkOrderDifficulty::Extended => "Extended resources or materials needed",
            WorkOrderDifficulty::Advanced => "Coordinated work and materials involved",
            WorkOrderDifficulty::HireOut => "Need a professional",
        }
    }

    pub fn numeric_level(&self) -> u8 {
        match self {
            WorkOrderDifficulty::Normal => 1,
            WorkOrderDifficulty::Extended => 2,
            WorkOrderDifficulty::Advanced => 3,
            WorkOrderDifficulty::HireOut => 4,
        }
    }
}

#[derive(Enum, Copy, Clone, PartialEq, Eq, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkOrderCost {
    One,
    Two,
    Three,
    Four,
}

impl WorkOrderCost {
    pub(crate) fn to_str(&self) -> &str {
        match self {
            WorkOrderCost::One => "one",
            WorkOrderCost::Two => "two",
            WorkOrderCost::Three => "three",
            WorkOrderCost::Four => "four",
        }
    }

    pub(crate) fn to_string(&self) -> String {
        self.to_str().to_string()
    }

    pub(crate) fn from_string(s: &str) -> Result<WorkOrderCost, AppError> {
        match s {
            "one" => Ok(Self::One),
            "two" => Ok(Self::Two),
            "three" => Ok(Self::Three),
            "four" => Ok(Self::Four),
            _ => Err(AppError::ValidationError("Invalid work order cost".to_string())),
        }
    }

    pub fn description(&self) -> &str {
        match self {
            WorkOrderCost::One => "Under $250",
            WorkOrderCost::Two => "$250 - $499",
            WorkOrderCost::Three => "$500 - $2K",
            WorkOrderCost::Four => "Over $2k",
        }
    }

    pub fn numeric_level(&self) -> u8 {
        match self {
            WorkOrderCost::One => 1,
            WorkOrderCost::Two => 2,
            WorkOrderCost::Three => 3,
            WorkOrderCost::Four => 4,
        }
    }
}

#[derive(Enum, Copy, Clone, PartialEq, Eq, Debug, Deserialize, Serialize, Hash)]
#[serde(rename_all = "snake_case")]
pub enum WorkOrderStatus {
    Draft,
    Scheduled,
    InProgress,
    OnHold,
    Completed,
    Cancelled,
    Failed,
    Deferred,
    WaitingParts,
    WaitingApproval,
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

#[derive(Enum, Copy, Clone, PartialEq, Eq, Debug, Deserialize, Serialize, Hash)]
#[serde(rename_all = "snake_case")]
pub enum WorkOrderPriority {
    Low,
    Normal,
    High,
    Urgent,
    Emergency,
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

#[derive(Enum, Copy, Clone, PartialEq, Eq, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkOrderType {
    Preventive,
    Corrective,
    Emergency,
    Inspection,
    Calibration,
    Installation,
    Upgrade,
    Replacement,
    Cleaning,
    Safety,
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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WorkOrder {
    pub id: String,
    pub work_order_number: String,
    pub title: String,
    pub description: String,
    pub asset_id: String,
    pub work_order_type: WorkOrderType,
    pub status: WorkOrderStatus,
    pub notes: Option<String>,
    pub priority: WorkOrderPriority,
    pub severity: WorkOrderSeverity,
    pub difficulty: WorkOrderDifficulty,
    pub assigned_technician_id: Option<String>,
    pub estimated_duration_minutes: i32,
    pub actual_duration_minutes: Option<i32>,
    pub completed_date: Option<DateTime<Utc>>,
    pub estimated_cost: WorkOrderCost,
    pub actual_cost: Option<Decimal>,
    pub labor_hours: Option<f64>,
    pub completion_notes: Option<String>,
    pub created_by: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl WorkOrder {
    pub fn new(
        id: String,
        work_order_number: String,
        title: String,
        description: String,
        notes: Option<String>,
        asset_id: String,
        work_order_type: String,
        priority: String,
        severity: WorkOrderSeverity,
        difficulty: WorkOrderDifficulty,
        assigned_technician_id: Option<String>,
        estimated_duration_minutes: i32,
        estimated_cost: WorkOrderCost,
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

        if created_by.trim().is_empty() {
            return Err(AppError::ValidationError("Created by cannot be empty".to_string()));
        }

        // Validate estimated duration
        if estimated_duration_minutes <= 0 {
            return Err(
                AppError::ValidationError("Estimated duration must be positive".to_string())
            );
        }

        let work_order_type_enum = WorkOrderType::from_string(&work_order_type)?;
        let priority_enum = WorkOrderPriority::from_string(&priority)?;

        Ok(Self {
            id,
            work_order_number,
            title,
            description,
            asset_id,
            notes,
            work_order_type: work_order_type_enum,
            status: WorkOrderStatus::Draft,
            priority: priority_enum,
            severity,
            difficulty,
            assigned_technician_id,
            estimated_duration_minutes,
            actual_duration_minutes: None,
            estimated_cost,
            actual_cost: None,
            completed_date: None,
            labor_hours: None,
            completion_notes: None,
            created_by,
            created_at: now,
            updated_at: now,
        })
    }

    pub fn is_in_progress(&self) -> bool {
        matches!(self.status, WorkOrderStatus::InProgress)
    }

    pub fn is_completed(&self) -> bool {
        matches!(self.status, WorkOrderStatus::Completed)
    }

    pub fn start_work(&mut self, technician_id: String) -> Result<(), AppError> {
        if !matches!(self.status, WorkOrderStatus::Scheduled) {
            return Err(
                AppError::ValidationError("Only scheduled work orders can be started".to_string())
            );
        }

        self.status = WorkOrderStatus::InProgress;
        self.assigned_technician_id = Some(technician_id);
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn complete_work(&mut self, completion_notes: Option<String>) -> Result<(), AppError> {
        if !matches!(self.status, WorkOrderStatus::InProgress) {
            return Err(
                AppError::ValidationError(
                    "Only in-progress work orders can be completed".to_string()
                )
            );
        }

        let now = Utc::now();
        self.status = WorkOrderStatus::Completed;
        self.completion_notes = completion_notes;
        self.updated_at = now;
        self.completed_date = Some(now);
        Ok(())
    }

    pub fn cancel_work(&mut self, reason: String) -> Result<(), AppError> {
        if matches!(self.status, WorkOrderStatus::Completed | WorkOrderStatus::Cancelled) {
            return Err(
                AppError::ValidationError(
                    "Cannot cancel completed or already cancelled work order".to_string()
                )
            );
        }
        let now = Utc::now();

        self.status = WorkOrderStatus::Cancelled;
        self.completion_notes = Some(reason);
        self.updated_at = now;
        self.completed_date = Some(now);
        Ok(())
    }

    pub fn put_on_hold(&mut self, reason: String) -> Result<(), AppError> {
        if !matches!(self.status, WorkOrderStatus::InProgress | WorkOrderStatus::Scheduled) {
            return Err(
                AppError::ValidationError(
                    "Only scheduled or in-progress work orders can be put on hold".to_string()
                )
            );
        }

        self.status = WorkOrderStatus::OnHold;
        self.completion_notes = Some(reason);
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn resume_from_hold(&mut self) -> Result<(), AppError> {
        if !matches!(self.status, WorkOrderStatus::OnHold) {
            return Err(
                AppError::ValidationError("Only work orders on hold can be resumed".to_string())
            );
        }

        self.status = WorkOrderStatus::Scheduled;
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn is_overdue(&self) -> bool {
        if matches!(self.status, WorkOrderStatus::Completed | WorkOrderStatus::Cancelled) {
            return false;
        }

        let estimated_completion =
            self.created_at + chrono::Duration::minutes(self.estimated_duration_minutes as i64);
        Utc::now() > estimated_completion
    }

    pub fn set_classification(
        &mut self,
        severity: WorkOrderSeverity,
        difficulty: WorkOrderDifficulty
    ) {
        self.severity = severity;
        self.difficulty = difficulty;
        self.updated_at = Utc::now();
    }
}

impl DynamoDbEntity for WorkOrder {
    fn table_name() -> &'static str {
        "WorkOrders"
    }

    fn primary_key(&self) -> String {
        self.id.clone()
    }

    fn from_item(item: &HashMap<String, AttributeValue>) -> Option<Self> {
        // info!("calling from_item with: {:?}", &item);

        let id = item.get("id")?.as_s().ok()?.to_string();
        let work_order_number = item.get("work_order_number")?.as_s().ok()?.to_string();
        let title = item.get("title")?.as_s().ok()?.to_string();
        let description = item.get("description")?.as_s().ok()?.to_string();

        let notes = item
            .get("completion_notes")
            .and_then(|v| v.as_s().ok())
            .map(|s| s.to_string());

        let asset_id = item.get("asset_id")?.as_s().ok()?.to_string();

        let work_order_type_str = item.get("work_order_type")?.as_s().ok()?;
        let work_order_type = WorkOrderType::from_string(&work_order_type_str).ok()?;

        let status_str = item.get("status")?.as_s().ok()?;
        let status = WorkOrderStatus::from_string(&status_str).ok()?;

        let priority_str = item.get("priority")?.as_s().ok()?;
        let priority = WorkOrderPriority::from_string(&priority_str).ok()?;

        let severity_str = item.get("severity")?.as_s().ok()?;
        let severity = WorkOrderSeverity::from_string(&severity_str).ok()?;

        let difficulty_str = item.get("difficulty")?.as_s().ok()?;
        let difficulty = WorkOrderDifficulty::from_string(&difficulty_str).ok()?;

        let assigned_technician_id = item
            .get("assigned_technician_id")
            .and_then(|v| v.as_s().ok())
            .map(|s| s.to_string());

        let estimated_duration_minutes = item
            .get("estimated_duration_minutes")
            .and_then(|v| v.as_n().ok())
            .and_then(|s| s.parse::<i32>().ok())
            .unwrap_or(60);

        let completed_date = item
            .get("completed_date")
            .and_then(|v| v.as_s().ok())
            .and_then(|s| s.parse::<DateTime<Utc>>().ok());

        let actual_duration_minutes = item
            .get("actual_duration_minutes")
            .and_then(|v| v.as_n().ok())
            .and_then(|s| s.parse::<i32>().ok());

        let estimated_cost_str = item.get("estimated_cost")?.as_s().ok()?;
        let estimated_cost = WorkOrderCost::from_string(&estimated_cost_str).ok()?;

        let actual_cost = item
            .get("actual_cost")
            .and_then(|v| v.as_s().ok())
            .and_then(|s| s.parse::<Decimal>().ok());

        let labor_hours = item
            .get("labor_hours")
            .and_then(|v| v.as_n().ok())
            .and_then(|s| s.parse::<f64>().ok());

        let completion_notes = item
            .get("completion_notes")
            .and_then(|v| v.as_s().ok())
            .map(|s| s.to_string());

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

        Some(Self {
            id,
            work_order_number,
            title,
            description,
            notes,
            asset_id,
            work_order_type,
            status,
            priority,
            severity,
            difficulty,
            assigned_technician_id,
            estimated_duration_minutes,
            actual_duration_minutes,
            completed_date,
            estimated_cost,
            actual_cost,
            labor_hours,
            completion_notes,
            created_by,
            created_at,
            updated_at,
        })
    }

    fn to_item(&self) -> HashMap<String, AttributeValue> {
        let mut item = HashMap::new();

        item.insert("id".to_string(), AttributeValue::S(self.id.clone()));
        item.insert(
            "work_order_number".to_string(),
            AttributeValue::S(self.work_order_number.clone())
        );
        item.insert("title".to_string(), AttributeValue::S(self.title.clone()));
        item.insert("description".to_string(), AttributeValue::S(self.description.clone()));
        if let Some(notes) = &self.notes {
            item.insert("notes".to_string(), AttributeValue::S(notes.clone()));
        }
        item.insert("asset_id".to_string(), AttributeValue::S(self.asset_id.clone()));
        item.insert(
            "work_order_type".to_string(),
            AttributeValue::S(self.work_order_type.to_string())
        );
        item.insert("status".to_string(), AttributeValue::S(self.status.to_string()));
        item.insert("priority".to_string(), AttributeValue::S(self.priority.to_string()));
        item.insert("severity".to_string(), AttributeValue::S(self.severity.to_string()));
        item.insert("difficulty".to_string(), AttributeValue::S(self.difficulty.to_string()));

        if let Some(tech_id) = &self.assigned_technician_id {
            item.insert("assigned_technician_id".to_string(), AttributeValue::S(tech_id.clone()));
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

        if let Some(completed_date) = &self.completed_date {
            item.insert(
                "completed_date".to_string(),
                AttributeValue::S(completed_date.to_string())
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

        if let Some(notes) = &self.completion_notes {
            item.insert("completion_notes".to_string(), AttributeValue::S(notes.clone()));
        }

        item.insert("created_by".to_string(), AttributeValue::S(self.created_by.clone()));
        item.insert("created_at".to_string(), AttributeValue::S(self.created_at.to_string()));
        item.insert("updated_at".to_string(), AttributeValue::S(self.updated_at.to_string()));

        item
    }
}
