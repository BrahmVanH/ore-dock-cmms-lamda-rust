use std::collections::HashMap;

use async_graphql::Object;
use aws_sdk_dynamodb::types::AttributeValue;
use chrono::{ DateTime, Utc };
use serde::{ Deserialize, Serialize };
use tracing::info;

use crate::error::AppError;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PermissionAction {
    Create,
    Read,
    Update,
    Delete,
    Execute,
    Approve,
    Assign,
    View,
    Export,
    Import,
}

impl PermissionAction {
    pub fn to_str(&self) -> &str {
        match self {
            PermissionAction::Create => "create",
            PermissionAction::Read => "read",
            PermissionAction::Update => "update",
            PermissionAction::Delete => "delete",
            PermissionAction::Execute => "execute",
            PermissionAction::Approve => "approve",
            PermissionAction::Assign => "assign",
            PermissionAction::View => "view",
            PermissionAction::Export => "export",
            PermissionAction::Import => "import",
        }
    }

    pub fn to_string(&self) -> String {
        self.to_str().to_string()
    }

    pub fn from_string(s: &str) -> Result<PermissionAction, AppError> {
        match s {
            "create" => Ok(Self::Create),
            "read" => Ok(Self::Read),
            "update" => Ok(Self::Update),
            "delete" => Ok(Self::Delete),
            "execute" => Ok(Self::Execute),
            "approve" => Ok(Self::Approve),
            "assign" => Ok(Self::Assign),
            "view" => Ok(Self::View),
            "export" => Ok(Self::Export),
            "import" => Ok(Self::Import),
            _ => Err(AppError::ValidationError("Invalid permission action".to_string())),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PermissionStatus {
    Granted,
    Denied,
    Pending,
    Expired,
    Revoked,
}

impl PermissionStatus {
    fn to_str(&self) -> &str {
        match self {
            PermissionStatus::Granted => "granted",
            PermissionStatus::Denied => "denied",
            PermissionStatus::Pending => "pending",
            PermissionStatus::Expired => "expired",
            PermissionStatus::Revoked => "revoked",
        }
    }

    fn to_string(&self) -> String {
        self.to_str().to_string()
    }

    fn from_string(s: &str) -> Result<PermissionStatus, AppError> {
        match s {
            "granted" => Ok(Self::Granted),
            "denied" => Ok(Self::Denied),
            "pending" => Ok(Self::Pending),
            "expired" => Ok(Self::Expired),
            "revoked" => Ok(Self::Revoked),
            _ => Err(AppError::ValidationError("Invalid permission status".to_string())),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ResourceType {
    Asset,
    Location,
    MaintenanceSchedule,
    WorkOrder,
    User,
    Role,
    Notification,
    Report,
    System,
}

impl ResourceType {
    pub fn to_str(&self) -> &str {
        match self {
            ResourceType::Asset => "asset",
            ResourceType::Location => "location",
            ResourceType::MaintenanceSchedule => "maintenance_schedule",
            ResourceType::WorkOrder => "work_order",
            ResourceType::User => "user",
            ResourceType::Role => "role",
            ResourceType::Notification => "notification",
            ResourceType::Report => "report",
            ResourceType::System => "system",
        }
    }

    pub fn to_string(&self) -> String {
        self.to_str().to_string()
    }

    pub fn from_string(s: &str) -> Result<ResourceType, AppError> {
        match s {
            "asset" => Ok(Self::Asset),
            "location" => Ok(Self::Location),
            "maintenance_schedule" => Ok(Self::MaintenanceSchedule),
            "work_order" => Ok(Self::WorkOrder),
            "user" => Ok(Self::User),
            "role" => Ok(Self::Role),
            "notification" => Ok(Self::Notification),
            "report" => Ok(Self::Report),
            "system" => Ok(Self::System),
            _ => Err(AppError::ValidationError("Invalid resource type".to_string())),
        }
    }
}

/// Represents a Permission Log in the system
///
/// # Fields
///
/// * `id` - Unique identifier for the permission log entry
/// * `user_id` - ID of the user attempting the action
/// * `resource_type` - Type of resource being accessed
/// * `resource_id` - ID of the specific resource
/// * `action` - The action being attempted
/// * `status` - Result of the permission check
/// * `attempted_at` - When the permission check was attempted
/// * `granted_at` - When permission was granted (if applicable)
/// * `denied_reason` - Reason for denial (if applicable)
/// * `ip_address` - IP address of the request
/// * `user_agent` - User agent string from the request
/// * `session_id` - Session ID of the user
/// * `role_at_time` - User's role at the time of the request
/// * `created_at` - Date and time of creation
/// * `updated_at` - Date and time of last update
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PermissionLog {
    pub id: String,
    pub user_id: String,
    pub resource_type: ResourceType,
    pub resource_id: String,
    pub action: PermissionAction,
    pub status: PermissionStatus,
    pub attempted_at: DateTime<Utc>,
    pub granted_at: Option<DateTime<Utc>>,
    pub denied_reason: Option<String>,
    pub ip_address: String,
    pub user_agent: String,
    pub session_id: Option<String>,
    pub role_at_time: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Defines methods for PermissionLog
impl PermissionLog {
    /// Creates new PermissionLog instance
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier
    /// * `user_id` - User ID attempting the action
    /// * `resource_type` - Resource type as string
    /// * `resource_id` - Resource ID
    /// * `action` - Action as string
    /// * `status` - Status as string
    /// * `attempted_at` - When the attempt was made
    /// * `granted_at` - Optional grant time
    /// * `denied_reason` - Optional denial reason
    /// * `ip_address` - IP address
    /// * `user_agent` - User agent string
    /// * `session_id` - Optional session ID
    /// * `role_at_time` - Optional role at time of request
    ///
    /// # Returns
    ///
    /// New PermissionLog instance
    pub fn new(
        id: String,
        user_id: String,
        resource_type: String,
        resource_id: String,
        action: String,
        status: String,
        attempted_at: DateTime<Utc>,
        granted_at: Option<DateTime<Utc>>,
        denied_reason: Option<String>,
        ip_address: String,
        user_agent: String,
        session_id: Option<String>,
        role_at_time: Option<String>
    ) -> Result<Self, AppError> {
        let now = Utc::now();

        if user_id.trim().is_empty() {
            return Err(AppError::ValidationError("User ID cannot be empty".to_string()));
        }

        if resource_id.trim().is_empty() {
            return Err(AppError::ValidationError("Resource ID cannot be empty".to_string()));
        }

        let resource_type_enum = ResourceType::from_string(&resource_type)?;
        let action_enum = PermissionAction::from_string(&action)?;
        let status_enum = PermissionStatus::from_string(&status)?;

        Ok(Self {
            id,
            user_id,
            resource_type: resource_type_enum,
            resource_id,
            action: action_enum,
            status: status_enum,
            attempted_at,
            granted_at,
            denied_reason,
            ip_address,
            user_agent,
            session_id,
            role_at_time,
            created_at: now,
            updated_at: now,
        })
    }

    /// Creates PermissionLog instance from DynamoDB item
    ///
    /// # Arguments
    ///
    /// * `item` - The dynamo db item
    ///
    /// # Returns
    ///
    /// 'Some' PermissionLog if item fields match, 'None' otherwise
    pub fn from_item(item: &HashMap<String, AttributeValue>) -> Option<Self> {
        info!("calling from_item with: {:?}", &item);

        let id = item.get("id")?.as_s().ok()?.to_string();
        let user_id = item.get("user_id")?.as_s().ok()?.to_string();

        let resource_type_str = item.get("resource_type")?.as_s().ok()?;
        let resource_type = ResourceType::from_string(&resource_type_str)
            .map_err(|e| e)
            .ok()?;

        let resource_id = item.get("resource_id")?.as_s().ok()?.to_string();

        let action_str = item.get("action")?.as_s().ok()?;
        let action = PermissionAction::from_string(&action_str)
            .map_err(|e| e)
            .ok()?;

        let status_str = item.get("status")?.as_s().ok()?;
        let status = PermissionStatus::from_string(&status_str)
            .map_err(|e| e)
            .ok()?;

        let attempted_at = item
            .get("attempted_at")
            .and_then(|v| v.as_s().ok())
            .and_then(|s| s.parse::<DateTime<Utc>>().ok())
            .unwrap_or_else(|| Utc::now());

        let granted_at = item
            .get("granted_at")
            .and_then(|v| v.as_s().ok())
            .and_then(|s| s.parse::<DateTime<Utc>>().ok());

        let denied_reason = item
            .get("denied_reason")
            .and_then(|v| v.as_s().ok())
            .map(|s| s.to_string());

        let ip_address = item.get("ip_address")?.as_s().ok()?.to_string();
        let user_agent = item.get("user_agent")?.as_s().ok()?.to_string();

        let session_id = item
            .get("session_id")
            .and_then(|v| v.as_s().ok())
            .map(|s| s.to_string());

        let role_at_time = item
            .get("role_at_time")
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

        let res = Some(Self {
            id,
            user_id,
            resource_type,
            resource_id,
            action,
            status,
            attempted_at,
            granted_at,
            denied_reason,
            ip_address,
            user_agent,
            session_id,
            role_at_time,
            created_at,
            updated_at,
        });

        info!("result of from_item on permission_log: {:?}", res);
        res
    }

    /// Creates DynamoDB item from PermissionLog instance
    ///
    /// # Arguments
    ///
    /// * `self` - borrowed instance of self
    ///
    /// # Returns
    ///
    /// HashMap representing DB item for PermissionLog instance
    pub fn to_item(&self) -> HashMap<String, AttributeValue> {
        let mut item = HashMap::new();

        item.insert("id".to_string(), AttributeValue::S(self.id.clone()));
        item.insert("user_id".to_string(), AttributeValue::S(self.user_id.clone()));
        item.insert(
            "resource_type".to_string(),
            AttributeValue::S(self.resource_type.to_str().to_string())
        );
        item.insert("resource_id".to_string(), AttributeValue::S(self.resource_id.clone()));
        item.insert("action".to_string(), AttributeValue::S(self.action.to_str().to_string()));
        item.insert("status".to_string(), AttributeValue::S(self.status.to_str().to_string()));
        item.insert("attempted_at".to_string(), AttributeValue::S(self.attempted_at.to_string()));

        if let Some(granted) = &self.granted_at {
            item.insert("granted_at".to_string(), AttributeValue::S(granted.to_string()));
        }

        if let Some(reason) = &self.denied_reason {
            item.insert("denied_reason".to_string(), AttributeValue::S(reason.clone()));
        }

        item.insert("ip_address".to_string(), AttributeValue::S(self.ip_address.clone()));
        item.insert("user_agent".to_string(), AttributeValue::S(self.user_agent.clone()));

        if let Some(session) = &self.session_id {
            item.insert("session_id".to_string(), AttributeValue::S(session.clone()));
        }

        if let Some(role) = &self.role_at_time {
            item.insert("role_at_time".to_string(), AttributeValue::S(role.clone()));
        }

        item.insert("created_at".to_string(), AttributeValue::S(self.created_at.to_string()));
        item.insert("updated_at".to_string(), AttributeValue::S(self.updated_at.to_string()));

        item
    }
}

#[Object]
impl PermissionLog {
    async fn id(&self) -> &str {
        &self.id
    }

    async fn user_id(&self) -> &str {
        &self.user_id
    }

    async fn resource_type(&self) -> &str {
        self.resource_type.to_str()
    }

    async fn resource_id(&self) -> &str {
        &self.resource_id
    }

    async fn action(&self) -> &str {
        self.action.to_str()
    }

    async fn status(&self) -> &str {
        self.status.to_str()
    }

    async fn attempted_at(&self) -> &DateTime<Utc> {
        &self.attempted_at
    }

    async fn granted_at(&self) -> Option<&DateTime<Utc>> {
        self.granted_at.as_ref()
    }

    async fn denied_reason(&self) -> Option<&str> {
        self.denied_reason.as_deref()
    }

    async fn ip_address(&self) -> &str {
        &self.ip_address
    }

    async fn user_agent(&self) -> &str {
        &self.user_agent
    }

    async fn session_id(&self) -> Option<&str> {
        self.session_id.as_deref()
    }

    async fn role_at_time(&self) -> Option<&str> {
        self.role_at_time.as_deref()
    }

    async fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }

    async fn updated_at(&self) -> &DateTime<Utc> {
        &self.updated_at
    }
}
