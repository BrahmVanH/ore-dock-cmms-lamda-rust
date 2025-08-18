use std::collections::HashMap;
use aws_sdk_dynamodb::types::AttributeValue;
use crate::{ AppError, DynamoDbEntity };
// DynamoDbEntity implementation for Task
use async_graphql::Enum;
use chrono::{ DateTime, Utc };
use serde::{ Deserialize, Serialize };

#[derive(Enum, Copy, Clone, PartialEq, Eq, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TaskType {
    WorkOrder,
    Cleaning,
    MaintenanceRequest,
}

impl TaskType {
    pub fn to_str(&self) -> &str {
        match self {
            TaskType::WorkOrder => "workOrder",
            TaskType::Cleaning => "cleaning",
            TaskType::MaintenanceRequest => "maintenanceRequest",
        }
    }

    pub fn to_string(&self) -> String {
        self.to_str().to_string()
    }

    pub fn from_string(s: &str) -> Result<TaskType, AppError> {
        match s {
            "workOrder" => Ok(Self::WorkOrder),
            "cleaning" => Ok(Self::Cleaning),
            "maintenanceRequest" => Ok(Self::MaintenanceRequest),
            _ => Err(AppError::ValidationError("Invalid task type".to_string())),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub task_number: String,
    pub title: String,
    pub description: String,
    pub work_order_id: Option<String>,
    pub task_type: TaskType,
    pub private: bool,
    pub completed: bool,
    pub assigned_to: Option<String>,
    pub completed_by: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
// Task methods and boilerplate
impl Task {
    /// Create a new Task instance
    pub fn new(
        id: String,
        task_number: String,
        title: String,
        description: String,
        work_order_id: Option<String>,
        task_type: String,
        private: bool,
        assigned_to: Option<String>
    ) -> Result<Self, AppError> {
        let now = Utc::now();
        if task_number.trim().is_empty() {
            return Err(AppError::ValidationError("Task number cannot be empty".to_string()));
        }
        if title.trim().is_empty() {
            return Err(AppError::ValidationError("Title cannot be empty".to_string()));
        }
        if description.trim().is_empty() {
            return Err(AppError::ValidationError("Description cannot be empty".to_string()));
        }

        if task_type == TaskType::WorkOrder.to_string() && work_order_id.is_none() {
            return Err(
                AppError::ValidationError(
                    "If task is of type work order, work_order_id is required".to_string()
                )
            );
        }

        if task_type != TaskType::WorkOrder.to_string() && work_order_id.is_some() {
            return Err(
                AppError::ValidationError(
                    "If task is not of type work order, work_order_id must be empty".to_string()
                )
            );
        }

        if task_type.trim().is_empty() {
            return Err(AppError::ValidationError("Task type cannot be empty".to_string()));
        }

        let task_type_enum = TaskType::from_string(&task_type)?;

        Ok(Self {
            id,
            task_number,
            title,
            description,
            work_order_id,
            task_type: task_type_enum,
            private,
            completed: false,
            assigned_to,
            completed_by: None,
            created_at: now,
            updated_at: now,
        })
    }

    /// Returns true if the task is completed
    pub fn is_completed(&self) -> bool {
        self.completed
    }

    /// Mark the task as completed
    pub fn complete_task(&mut self, completed_by: Option<String>) {
        self.completed = true;
        self.completed_by = completed_by;
        self.updated_at = Utc::now();
    }

    /// Assign the task to a user
    pub fn assign_to(&mut self, user_id: String) {
        self.assigned_to = Some(user_id);
        self.updated_at = Utc::now();
    }

    /// Unassign the task
    pub fn unassign(&mut self) {
        self.assigned_to = None;
        self.updated_at = Utc::now();
    }

    /// Update the task's title and description
    pub fn update_details(&mut self, title: Option<String>, description: Option<String>) {
        if let Some(t) = title {
            self.title = t;
        }
        if let Some(d) = description {
            self.description = d;
        }
        self.updated_at = Utc::now();
    }

    pub fn update_work_order_id(&mut self, work_order_id: Option<String>) {
        self.work_order_id = work_order_id;
        self.updated_at = Utc::now();
    }
}

impl DynamoDbEntity for Task {
    fn table_name() -> &'static str {
        "Tasks"
    }

    fn primary_key(&self) -> String {
        self.id.clone()
    }

    fn from_item(item: &HashMap<String, AttributeValue>) -> Option<Self> {
        let id = item.get("id")?.as_s().ok()?.to_string();

        let task_number = item.get("task_number")?.as_s().ok()?.to_string();

        let title = item.get("title")?.as_s().ok()?.to_string();

        let description = item.get("description")?.as_s().ok()?.to_string();

        let work_order_id = item
            .get("work_order_id")
            .and_then(|v| v.as_s().ok())
            .map(|s| s.to_string());

        let task_type_str = item.get("task_type")?.as_s().ok()?;

        let task_type = TaskType::from_string(&task_type_str).ok()?;

        let private = item.get("private")?.as_bool().ok().unwrap_or(&false);

        let completed = item.get("completed")?.as_bool().ok().unwrap_or(&false);

        let assigned_to = item
            .get("assigned_to")
            .and_then(|v| v.as_s().ok())
            .map(|s| s.to_string());
        let completed_by = item
            .get("completed_by")
            .and_then(|v| v.as_s().ok())
            .map(|s| s.to_string());
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
            task_number,
            title,
            description,
            work_order_id,
            task_type,
            private: *private,
            completed: *completed,
            assigned_to,
            completed_by,
            created_at,
            updated_at,
        })
    }

    fn to_item(&self) -> HashMap<String, AttributeValue> {
        let mut item = HashMap::new();

        item.insert("id".to_string(), AttributeValue::S(self.id.clone()));

        item.insert("task_number".to_string(), AttributeValue::S(self.task_number.clone()));

        item.insert("title".to_string(), AttributeValue::S(self.title.clone()));

        item.insert("description".to_string(), AttributeValue::S(self.description.clone()));

        if let Some(work_order_id) = &self.work_order_id {
            item.insert("work_order_id".to_string(), AttributeValue::S(work_order_id.clone()));
        }

        item.insert("task_type".to_string(), AttributeValue::S(self.task_type.to_string()));

        item.insert("private".to_string(), AttributeValue::Bool(self.private));

        item.insert("completed".to_string(), AttributeValue::Bool(self.completed));

        if let Some(assigned_to) = &self.assigned_to {
            item.insert("assigned_to".to_string(), AttributeValue::S(assigned_to.clone()));
        }

        if let Some(completed_by) = &self.completed_by {
            item.insert("completed_by".to_string(), AttributeValue::S(completed_by.clone()));
        }

        item.insert("created_at".to_string(), AttributeValue::S(self.created_at.to_string()));

        item.insert("updated_at".to_string(), AttributeValue::S(self.updated_at.to_string()));

        item
    }
}
