use std::collections::HashMap;

use async_graphql::Object;
use aws_sdk_dynamodb::types::AttributeValue;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value as Json;
use tracing::info;

use crate::{error::AppError, models::permission_log::{PermissionAction, ResourceType}};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PermissionScope {
    Global,     // System-wide access
    Organization, // Organization-level access
    Location,   // Location-specific access
    Asset,      // Asset-specific access
    Own,        // Only own resources
}

impl PermissionScope {
    fn to_str(&self) -> &str {
        match self {
            PermissionScope::Global => "global",
            PermissionScope::Organization => "organization",
            PermissionScope::Location => "location",
            PermissionScope::Asset => "asset",
            PermissionScope::Own => "own",
        }
    }

    fn to_string(&self) -> String {
        self.to_str().to_string()
    }

    fn from_string(s: &str) -> Result<PermissionScope, AppError> {
        match s {
            "global" => Ok(Self::Global),
            "organization" => Ok(Self::Organization),
            "location" => Ok(Self::Location),
            "asset" => Ok(Self::Asset),
            "own" => Ok(Self::Own),
            _ => Err(AppError::ValidationError("Invalid permission scope".to_string())),
        }
    }
}

/// Represents a Permission in the system
///
/// # Fields
///
/// * `id` - Unique identifier for the permission
/// * `role_id` - ID of the role this permission belongs to
/// * `resource_type` - Type of resource this permission applies to
/// * `actions` - List of actions allowed on the resource
/// * `scope` - Scope of the permission (global, organization, location, etc.)
/// * `conditions` - Optional JSON conditions for fine-grained access control
/// * `resource_filters` - Optional filters to limit which resources can be accessed
/// * `active` - Whether this permission is currently active
/// * `expires_at` - Optional expiration date for the permission
/// * `created_by` - User who created this permission
/// * `created_at` - Date and time of creation
/// * `updated_at` - Date and time of last update
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Permission {
    pub id: String,
    pub role_id: String,
    pub resource_type: ResourceType,
    pub actions: Vec<PermissionAction>,
    pub scope: PermissionScope,
    pub conditions: Option<Json>,
    pub resource_filters: Option<Json>,
    pub active: bool,
    pub expires_at: Option<DateTime<Utc>>,
    pub created_by: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Defines methods for Permission
impl Permission {
    /// Creates new Permission instance
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier
    /// * `role_id` - Role ID this permission belongs to
    /// * `resource_type` - Resource type as string
    /// * `actions` - List of actions as strings
    /// * `scope` - Permission scope as string
    /// * `conditions` - Optional JSON conditions
    /// * `resource_filters` - Optional JSON resource filters
    /// * `active` - Whether permission is active
    /// * `expires_at` - Optional expiration time
    /// * `created_by` - User who created the permission
    ///
    /// # Returns
    ///
    /// New Permission instance
    pub fn new(
        id: String,
        role_id: String,
        resource_type: String,
        actions: Vec<String>,
        scope: String,
        conditions: Option<Json>,
        resource_filters: Option<Json>,
        active: bool,
        expires_at: Option<DateTime<Utc>>,
        created_by: String,
    ) -> Result<Self, AppError> {
        let now = Utc::now();

        if role_id.trim().is_empty() {
            return Err(AppError::ValidationError("Role ID cannot be empty".to_string()));
        }

        if created_by.trim().is_empty() {
            return Err(AppError::ValidationError("Created by cannot be empty".to_string()));
        }

        if actions.is_empty() {
            return Err(AppError::ValidationError("At least one action is required".to_string()));
        }

        let resource_type_enum = ResourceType::from_string(&resource_type)?;
        let scope_enum = PermissionScope::from_string(&scope)?;

        // Convert action strings to enums
        let action_enums: Result<Vec<PermissionAction>, AppError> = actions
            .iter()
            .map(|a| PermissionAction::from_string(a))
            .collect();
        let action_enums = action_enums?;

        Ok(Self {
            id,
            role_id,
            resource_type: resource_type_enum,
            actions: action_enums,
            scope: scope_enum,
            conditions,
            resource_filters,
            active,
            expires_at,
            created_by,
            created_at: now,
            updated_at: now,
        })
    }

    /// Creates Permission instance from DynamoDB item
    ///
    /// # Arguments
    ///
    /// * `item` - The dynamo db item
    ///
    /// # Returns
    ///
    /// 'Some' Permission if item fields match, 'None' otherwise
    pub fn from_item(item: &HashMap<String, AttributeValue>) -> Option<Self> {
        info!("calling from_item with: {:?}", &item);

        let id = item.get("id")?.as_s().ok()?.to_string();
        let role_id = item.get("role_id")?.as_s().ok()?.to_string();

        let resource_type_str = item.get("resource_type")?.as_s().ok()?;
        let resource_type = ResourceType::from_string(&resource_type_str)
            .map_err(|e| e)
            .ok()?;

        // Handle actions as a string set
        let actions = item
            .get("actions")
            .and_then(|v| v.as_ss().ok())
            .map(|action_strs| {
                action_strs
                    .iter()
                    .filter_map(|a| PermissionAction::from_string(a).ok())
                    .collect::<Vec<PermissionAction>>()
            })
            .unwrap_or_default();

        let scope_str = item.get("scope")?.as_s().ok()?;
        let scope = PermissionScope::from_string(&scope_str)
            .map_err(|e| e)
            .ok()?;

        let conditions = item
            .get("conditions")
            .and_then(|v| v.as_s().ok())
            .and_then(|s| serde_json::from_str::<Json>(s).ok());

        let resource_filters = item
            .get("resource_filters")
            .and_then(|v| v.as_s().ok())
            .and_then(|s| serde_json::from_str::<Json>(s).ok());

        let active = item
            .get("active")
            .and_then(|v| v.as_bool().ok())
            .unwrap_or(&true);

        let expires_at = item
            .get("expires_at")
            .and_then(|v| v.as_s().ok())
            .and_then(|s| s.parse::<DateTime<Utc>>().ok());

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
            role_id,
            resource_type,
            actions,
            scope,
            conditions,
            resource_filters,
            active,
            expires_at,
            created_by,
            created_at,
            updated_at,
        });

        info!("result of from_item on permission: {:?}", res);
        res
    }

    /// Creates DynamoDB item from Permission instance
    ///
    /// # Arguments
    ///
    /// * `self` - borrowed instance of self
    ///
    /// # Returns
    ///
    /// HashMap representing DB item for Permission instance
    pub fn to_item(&self) -> HashMap<String, AttributeValue> {
        let mut item = HashMap::new();

        item.insert("id".to_string(), AttributeValue::S(self.id.clone()));
        item.insert("role_id".to_string(), AttributeValue::S(self.role_id.clone()));
        item.insert("resource_type".to_string(), AttributeValue::S(self.resource_type.to_str().to_string()));

        // Convert actions to string set
        let action_strings: Vec<String> = self
            .actions
            .iter()
            .map(|a| a.to_str().to_string())
            .collect();
        if !action_strings.is_empty() {
            item.insert("actions".to_string(), AttributeValue::Ss(action_strings));
        }

        item.insert("scope".to_string(), AttributeValue::S(self.scope.to_str().to_string()));

        if let Some(conditions) = &self.conditions {
            if let Ok(conditions_json) = serde_json::to_string(conditions) {
                item.insert("conditions".to_string(), AttributeValue::S(conditions_json));
            }
        }

        if let Some(filters) = &self.resource_filters {
            if let Ok(filters_json) = serde_json::to_string(filters) {
                item.insert("resource_filters".to_string(), AttributeValue::S(filters_json));
            }
        }

        item.insert("active".to_string(), AttributeValue::Bool(self.active));

        if let Some(expires) = &self.expires_at {
            item.insert("expires_at".to_string(), AttributeValue::S(expires.to_string()));
        }

        item.insert("created_by".to_string(), AttributeValue::S(self.created_by.clone()));
        item.insert("created_at".to_string(), AttributeValue::S(self.created_at.to_string()));
        item.insert("updated_at".to_string(), AttributeValue::S(self.updated_at.to_string()));

        item
    }

    /// Checks if this permission allows a specific action
    pub fn allows_action(&self, action: &PermissionAction) -> bool {
        self.active && self.actions.contains(action) && !self.is_expired()
    }

    /// Checks if the permission has expired
    pub fn is_expired(&self) -> bool {
        if let Some(expires_at) = &self.expires_at {
            Utc::now() > *expires_at
        } else {
            false
        }
    }

    /// Checks if permission applies to a specific resource
    pub fn applies_to_resource(&self, resource_type: &ResourceType, resource_id: &str) -> bool {
        if &self.resource_type != resource_type {
            return false;
        }

        // TODO: Implement resource filter logic based on conditions and resource_filters
        // For now, return true if the resource type matches
        true
    }
}

#[Object]
impl Permission {
    async fn id(&self) -> &str {
        &self.id
    }

    async fn role_id(&self) -> &str {
        &self.role_id
    }

    async fn resource_type(&self) -> &str {
        self.resource_type.to_str()
    }

    async fn actions(&self) -> Vec<String> {
        self.actions.iter().map(|a| a.to_str().to_string()).collect()
    }

    async fn scope(&self) -> &str {
        self.scope.to_str()
    }

    async fn conditions(&self) -> Option<String> {
        self.conditions.as_ref().and_then(|c| serde_json::to_string(c).ok())
    }

    async fn resource_filters(&self) -> Option<String> {
        self.resource_filters.as_ref().and_then(|f| serde_json::to_string(f).ok())
    }

    async fn active(&self) -> bool {
        self.active
    }

    async fn expires_at(&self) -> Option<&DateTime<Utc>> {
        self.expires_at.as_ref()
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
    #[graphql(name = "is_expired")]
    async fn get_is_expired(&self) -> bool {
        self.is_expired()
    }

    #[graphql(name = "allows_action")]
    async fn check_allows_action(&self, action: String) -> bool {
        if let Ok(action_enum) = PermissionAction::from_string(&action) {
            self.allows_action(&action_enum)
        } else {
            false
        }
    }
}