/*
Maintenance Request form fields

pub struct MaintenanceRequest

submitted_by: String,
manager_on_site: String,

created_at: String,
work_order_created: Boolean,
status: MaintenanceRequestStatusOptions
read_by_id: String
description: String,
reported_location: String,
troubleshooting_performed: String,
severity: WorkOrderSeverity,


MaintenanceRequestStatusOptions:

Submitted,
Read,
Accepted,
Denied,
Archived



*/

// FEATURES:

//  - when maint request has been viewed in client, trigger status update and updated
// read-by
//  - If request has been read but not updated, send notification to reader about
// changing Accepting or denying
//  - When user selects "create work order from request" update status to Accepted

use std::collections::HashMap;

use async_graphql::Enum;
use aws_sdk_dynamodb::types::AttributeValue;
use chrono::{ DateTime, Utc };
use serde::{ Deserialize, Serialize };
use tracing::info;

use crate::{ models::work_order::WorkOrderSeverity, AppError, DynamoDbEntity };

// declare MaintenanceRequestStatus Enum
#[derive(Enum, Copy, Clone, PartialEq, Eq, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum MaintenanceRequestStatus {
    Submitted,
    Read,
    Accepted,
    Denied,
    Archived,
}

// create MaintenanceRequestStatus impl
impl MaintenanceRequestStatus {
    pub fn to_str(&self) -> &str {
        match self {
            MaintenanceRequestStatus::Submitted => "Submitted",
            MaintenanceRequestStatus::Read => "Read",
            MaintenanceRequestStatus::Accepted => "Accepted",
            MaintenanceRequestStatus::Denied => "Denied",
            MaintenanceRequestStatus::Archived => "Archived",
        }
    }

    pub fn to_string(&self) -> String {
        self.to_str().to_string()
    }

    pub fn from_string(s: &str) -> Result<MaintenanceRequestStatus, AppError> {
        match s {
            "Submitted" => Ok(Self::Submitted),
            "Read" => Ok(Self::Read),
            "Accepted" => Ok(Self::Accepted),
            "Denied" => Ok(Self::Denied),
            "Archived" => Ok(Self::Archived),
            _ => Err(AppError::ValidationError("Invalid maintenance request status".to_string())),
        }
    }
}
// declare MaintenanceRequest struct
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MaintenanceRequest {
    pub id: String,
    pub submitted_by: String,
    pub manager_on_site: String,
    pub work_order_created: bool,
    pub status: MaintenanceRequestStatus,
    pub read_by_id: Option<String>,
    pub description: String,
    pub reported_location: String,
    pub troubleshooting_performed: String,
    pub severity: WorkOrderSeverity,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
// create MaintenanceRequest impl
impl MaintenanceRequest {
    pub fn new(
        id: String,
        submitted_by: String,
        manager_on_site: String,
        description: String,
        reported_location: String,
        troubleshooting_performed: String,
        severity: WorkOrderSeverity
    ) -> Result<Self, AppError> {
        let now = Utc::now();

        if id.trim().is_empty() {
            return Err(AppError::ValidationError("id cannot be empty".to_string()));
        }

        if submitted_by.trim().is_empty() {
            return Err(AppError::ValidationError("submitted_by cannot be empty".to_string()));
        }

        if manager_on_site.trim().is_empty() {
            return Err(AppError::ValidationError("manager_on_site cannot be empty".to_string()));
        }

        if description.trim().is_empty() {
            return Err(AppError::ValidationError("description cannot be empty".to_string()));
        }

        if reported_location.trim().is_empty() {
            return Err(AppError::ValidationError("reported_location cannot be empty".to_string()));
        }

        if troubleshooting_performed.trim().is_empty() {
            return Err(
                AppError::ValidationError("troubleshooting_performed cannot be empty".to_string())
            );
        }

        let work_order_created = false;
        let status = MaintenanceRequestStatus::Submitted;
        let read_by_id = None;
        let created_at = now;
        let updated_at = now;

        return Ok(MaintenanceRequest {
            id,
            submitted_by,
            manager_on_site,
            work_order_created,
            status,
            read_by_id,
            description,
            reported_location,
            troubleshooting_performed,
            severity,
            created_at,
            updated_at,
        });
    }

    pub fn is_accepted(&self) -> bool {
        matches!(self.status, MaintenanceRequestStatus::Accepted)
    }

    pub fn is_denied(&self) -> bool {
        matches!(self.status, MaintenanceRequestStatus::Denied)
    }

    pub fn is_archived(&self) -> bool {
        matches!(self.status, MaintenanceRequestStatus::Archived)
    }

    pub fn archive(&mut self) -> Result<(), AppError> {
        if !matches!(self.status, MaintenanceRequestStatus::Read) {
            return Err(
                AppError::ValidationError(
                    "Only read maintenance requests can be archived".to_string()
                )
            );
        }
        self.status = MaintenanceRequestStatus::Archived;
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn has_been_read(&self) -> bool {
        matches!(self.status, MaintenanceRequestStatus::Read)
    }

    pub fn mark_as_read(&mut self, user_id: String) -> Result<(), AppError> {
        if user_id.trim().is_empty() {
            return Err(AppError::ValidationError("UserId cannot be empty".to_string()));
        }
        self.read_by_id = Some(user_id);
        self.status = MaintenanceRequestStatus::Read;
        Ok(())
    }
}

// create DynamoDbEntity impl for MaintenanceRequest
impl DynamoDbEntity for MaintenanceRequest {
    fn table_name() -> &'static str {
        "MaintenanceRequests"
    }
    fn primary_key(&self) -> String {
        self.id.clone()
    }
    fn from_item(item: &HashMap<String, AttributeValue>) -> Option<Self> {
        info!("calling from_item with: {:?}", &item);

        let id = item.get("id")?.as_s().ok()?.to_string();
        let submitted_by = item.get("submitted_by")?.as_s().ok()?.to_string();
        let manager_on_site = item.get("manager_on_site")?.as_s().ok()?.to_string();
        let work_order_created = item.get("work_order_created")?.as_bool().ok()?;

        let status_str = item.get("status")?.as_s().ok()?;

        let status = MaintenanceRequestStatus::from_string(&status_str).ok()?;

        let read_by_id = item
            .get("read_by_id")
            .and_then(|v| v.as_s().ok())
            .map(|s| s.to_string());

        let description = item.get("description")?.as_s().ok()?.to_string();

        let reported_location = item.get("reported_location")?.as_s().ok()?.to_string();

        let troubleshooting_performed = item
            .get("troubleshooting_performed")?
            .as_s()
            .ok()?
            .to_string();

        let severity_str = item.get("severity")?.as_s().ok()?;

        let severity = WorkOrderSeverity::from_string(&severity_str).ok()?;

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
            submitted_by,
            manager_on_site,
            work_order_created: *work_order_created,
            status,
            read_by_id,
            description,
            reported_location,
            troubleshooting_performed,
            severity,
            created_at,
            updated_at,
        })
    }

    fn to_item(&self) -> HashMap<String, AttributeValue> {
        let mut item = HashMap::new();

        item.insert("id".to_string(), AttributeValue::S(self.id.clone()));
        item.insert("submitted_by".to_string(), AttributeValue::S(self.submitted_by.clone()));
        item.insert("manager_on_site".to_string(), AttributeValue::S(self.manager_on_site.clone()));

        item.insert(
            "work_order_created".to_string(),
            AttributeValue::Bool(self.work_order_created.clone())
        );
        item.insert("status".to_string(), AttributeValue::S(self.status.to_str().to_string()));

        if let Some(read_by_id) = &self.read_by_id {
            item.insert("read_by_id".to_string(), AttributeValue::S(read_by_id.clone()));
        }

        item.insert("description".to_string(), AttributeValue::S(self.description.clone()));

        item.insert(
            "reported_location".to_string(),
            AttributeValue::S(self.reported_location.to_string())
        );

        item.insert(
            "troubleshooting_performed".to_string(),
            AttributeValue::S(self.troubleshooting_performed.to_string())
        );

        item.insert("severity".to_string(), AttributeValue::S(self.severity.to_string()));

        item.insert("created_at".to_string(), AttributeValue::S(self.created_at.to_string()));
        item.insert("updated_at".to_string(), AttributeValue::S(self.updated_at.to_string()));

        item
    }
}
