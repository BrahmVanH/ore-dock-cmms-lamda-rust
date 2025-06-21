use std::collections::HashMap;

use aws_sdk_dynamodb::types::AttributeValue;
use chrono::{ DateTime, Utc };
use serde::{ Deserialize, Serialize };
use tracing::info;

use crate::error::AppError;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RoleType {
    System, // Built-in system roles
    Custom, // User-defined roles
    Group, // Group-based roles
    Temporary, // Temporary roles with expiration
}

impl RoleType {
    pub(crate) fn to_str(&self) -> &str {
        match self {
            RoleType::System => "system",
            RoleType::Custom => "custom",
            RoleType::Group => "group",
            RoleType::Temporary => "temporary",
        }
    }

    pub(crate) fn to_string(&self) -> String {
        self.to_str().to_string()
    }

    pub(crate) fn from_string(s: &str) -> Result<RoleType, AppError> {
        match s {
            "system" => Ok(Self::System),
            "custom" => Ok(Self::Custom),
            "group" => Ok(Self::Group),
            "temporary" => Ok(Self::Temporary),
            _ => Err(AppError::ValidationError("Invalid role type".to_string())),
        }
    }
}

/// Represents a Role in the system
///
/// # Fields
///
/// * `id` - Unique identifier for the role
/// * `name` - Name of the role
/// * `description` - Optional description of the role
/// * `role_type` - Type of role (system, custom, group, temporary)
/// * `is_system_role` - Whether this is a built-in system role
/// * `permission_ids` - List of permission IDs assigned to this role
/// * `parent_role_id` - Optional parent role for role inheritance
/// * `priority` - Priority level for role hierarchy (higher number = higher priority)
/// * `active` - Whether this role is currently active
/// * `expires_at` - Optional expiration date for temporary roles
/// * `max_users` - Optional maximum number of users that can have this role
/// * `created_by` - User who created this role
/// * `created_at` - Date and time of creation
/// * `updated_at` - Date and time of last update
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Role {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub role_type: RoleType,
    pub is_system_role: bool,
    pub permission_ids: Vec<String>,
    pub parent_role_id: Option<String>,
    pub priority: i32,
    pub active: bool,
    pub expires_at: Option<DateTime<Utc>>,
    pub max_users: Option<i32>,
    pub created_by: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Defines methods for Role
impl Role {
    /// Creates new Role instance
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier
    /// * `name` - Name of the role
    /// * `description` - Optional description
    /// * `role_type` - Type of role as string
    /// * `is_system_role` - Whether this is a system role
    /// * `permission_ids` - List of permission IDs
    /// * `parent_role_id` - Optional parent role ID
    /// * `priority` - Priority level
    /// * `active` - Whether role is active
    /// * `expires_at` - Optional expiration time
    /// * `max_users` - Optional maximum users
    /// * `created_by` - Optional creator user ID
    ///
    /// # Returns
    ///
    /// New Role instance
    pub fn new(
        id: String,
        name: String,
        description: Option<String>,
        role_type: String,
        is_system_role: bool,
        permission_ids: Vec<String>,
        parent_role_id: Option<String>,
        priority: i32,
        active: bool,
        expires_at: Option<DateTime<Utc>>,
        max_users: Option<i32>,
        created_by: Option<String>
    ) -> Result<Self, AppError> {
        let now = Utc::now();

        if name.trim().is_empty() {
            return Err(AppError::ValidationError("Role name cannot be empty".to_string()));
        }

        let role_type_enum = RoleType::from_string(&role_type)?;

        // Validate system role restrictions
        if is_system_role && role_type_enum != RoleType::System {
            return Err(AppError::ValidationError("System roles must have System type".to_string()));
        }

        Ok(Self {
            id,
            name,
            description,
            role_type: role_type_enum,
            is_system_role,
            permission_ids,
            parent_role_id,
            priority,
            active,
            expires_at,
            max_users,
            created_by,
            created_at: now,
            updated_at: now,
        })
    }

    /// Creates Role instance from DynamoDB item
    ///
    /// # Arguments
    ///
    /// * `item` - The dynamo db item
    ///
    /// # Returns
    ///
    /// 'Some' Role if item fields match, 'None' otherwise
    pub(crate) fn from_item(item: &HashMap<String, AttributeValue>) -> Option<Self> {
        info!("calling from_item with: {:?}", &item);

        let id = item.get("id")?.as_s().ok()?.to_string();
        let name = item.get("name")?.as_s().ok()?.to_string();

        let description = item
            .get("description")
            .and_then(|v| v.as_s().ok())
            .map(|s| s.to_string());

        let role_type_str = item.get("role_type")?.as_s().ok()?;
        let role_type = RoleType::from_string(&role_type_str)
            .map_err(|e| e)
            .ok()?;

        let is_system_role = item
            .get("is_system_role")
            .and_then(|v| v.as_bool().ok())
            .unwrap_or(&false);

        let permission_ids = item
            .get("permission_ids")
            .and_then(|v| v.as_ss().ok())
            .map(|ids|
                ids
                    .iter()
                    .map(|s| s.to_string())
                    .collect()
            )
            .unwrap_or_default();

        let parent_role_id = item
            .get("parent_role_id")
            .and_then(|v| v.as_s().ok())
            .map(|s| s.to_string());

        let priority = item
            .get("priority")
            .and_then(|v| v.as_n().ok())
            .and_then(|s| s.parse::<i32>().ok())
            .unwrap_or(0);

        let active = item
            .get("active")
            .and_then(|v| v.as_bool().ok())
            .unwrap_or(&true);

        let expires_at = item
            .get("expires_at")
            .and_then(|v| v.as_s().ok())
            .and_then(|s| s.parse::<DateTime<Utc>>().ok());

        let max_users = item
            .get("max_users")
            .and_then(|v| v.as_n().ok())
            .and_then(|s| s.parse::<i32>().ok());

        let created_by = item
            .get("created_by")
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
            name,
            description,
            role_type,
            is_system_role,
            permission_ids,
            parent_role_id,
            priority,
            active,
            expires_at,
            max_users,
            created_by,
            created_at,
            updated_at,
        });

        info!("result of from_item on role: {:?}", res);
        res
    }

    /// Creates DynamoDB item from Role instance
    ///
    /// # Arguments
    ///
    /// * `self` - borrowed instance of self
    ///
    /// # Returns
    ///
    /// HashMap representing DB item for Role instance
    pub(crate) fn to_item(&self) -> HashMap<String, AttributeValue> {
        let mut item = HashMap::new();

        item.insert("id".to_string(), AttributeValue::S(self.id.clone()));
        item.insert("name".to_string(), AttributeValue::S(self.name.clone()));

        if let Some(desc) = &self.description {
            item.insert("description".to_string(), AttributeValue::S(desc.clone()));
        }

        item.insert(
            "role_type".to_string(),
            AttributeValue::S(self.role_type.to_str().to_string())
        );
        item.insert("is_system_role".to_string(), AttributeValue::Bool(self.is_system_role));

        // Store permission IDs as string set
        if !self.permission_ids.is_empty() {
            item.insert(
                "permission_ids".to_string(),
                AttributeValue::Ss(self.permission_ids.clone())
            );
        }

        if let Some(parent_id) = &self.parent_role_id {
            item.insert("parent_role_id".to_string(), AttributeValue::S(parent_id.clone()));
        }

        item.insert("priority".to_string(), AttributeValue::N(self.priority.to_string()));
        item.insert("active".to_string(), AttributeValue::Bool(self.active));

        if let Some(expires) = &self.expires_at {
            item.insert("expires_at".to_string(), AttributeValue::S(expires.to_string()));
        }

        if let Some(max) = &self.max_users {
            item.insert("max_users".to_string(), AttributeValue::N(max.to_string()));
        }

        if let Some(creator) = &self.created_by {
            item.insert("created_by".to_string(), AttributeValue::S(creator.clone()));
        }

        item.insert("created_at".to_string(), AttributeValue::S(self.created_at.to_string()));
        item.insert("updated_at".to_string(), AttributeValue::S(self.updated_at.to_string()));

        item
    }

    /// Checks if the role has expired
    fn is_expired(&self) -> bool {
        if let Some(expires_at) = &self.expires_at { Utc::now() > *expires_at } else { false }
    }

    /// Checks if the role is currently usable
    fn is_usable(&self) -> bool {
        self.active && !self.is_expired()
    }

    /// Adds a permission to this role
    fn add_permission(&mut self, permission_id: String) {
        if !self.permission_ids.contains(&permission_id) {
            self.permission_ids.push(permission_id);
            self.updated_at = Utc::now();
        }
    }

    /// Removes a permission from this role
    fn remove_permission(&mut self, permission_id: &str) {
        if let Some(pos) = self.permission_ids.iter().position(|x| x == permission_id) {
            self.permission_ids.remove(pos);
            self.updated_at = Utc::now();
        }
    }

    /// Checks if this role has a specific permission
    fn has_permission(&self, permission_id: &str) -> bool {
        self.permission_ids.contains(&permission_id.to_string())
    }
}
