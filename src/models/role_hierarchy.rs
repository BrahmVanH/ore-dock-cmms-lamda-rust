use std::collections::HashMap;

use aws_sdk_dynamodb::types::AttributeValue;
use chrono::{ DateTime, Utc };
use serde::{ Deserialize, Serialize };
use tracing::info;

use crate::error::AppError;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum HierarchyType {
    Direct, // Direct parent-child relationship
    Inherited, // Permissions inherited through hierarchy
    Delegated, // Temporary delegation of permissions
    Conditional, // Conditional inheritance based on context
}

impl HierarchyType {
    pub(crate) fn to_str(&self) -> &str {
        match self {
            HierarchyType::Direct => "direct",
            HierarchyType::Inherited => "inherited",
            HierarchyType::Delegated => "delegated",
            HierarchyType::Conditional => "conditional",
        }
    }

    pub(crate) fn to_string(&self) -> String {
        self.to_str().to_string()
    }

    pub(crate) fn from_string(s: &str) -> Result<HierarchyType, AppError> {
        match s {
            "direct" => Ok(Self::Direct),
            "inherited" => Ok(Self::Inherited),
            "delegated" => Ok(Self::Delegated),
            "conditional" => Ok(Self::Conditional),
            _ => Err(AppError::ValidationError("Invalid hierarchy type".to_string())),
        }
    }
}

/// Represents a Role Hierarchy in the system
///
/// # Fields
///
/// * `id` - Unique identifier for the hierarchy relationship
/// * `parent_role_id` - ID of the parent role
/// * `child_role_id` - ID of the child role
/// * `hierarchy_type` - Type of hierarchy relationship
/// * `inherited_permissions` - Whether child inherits parent permissions
/// * `permission_overrides` - List of permission IDs that override inherited ones
/// * `depth_level` - Depth level in the hierarchy (0 = root, 1 = first level, etc.)
/// * `active` - Whether this hierarchy relationship is active
/// * `priority` - Priority order when multiple parents exist
/// * `conditions` - Optional conditions for when hierarchy applies
/// * `delegation_expires_at` - Expiration for delegated relationships
/// * `created_by` - User who created this hierarchy
/// * `created_at` - Date and time of creation
/// * `updated_at` - Date and time of last update
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RoleHierarchy {
    pub id: String,
    pub parent_role_id: String,
    pub child_role_id: String,
    pub hierarchy_type: HierarchyType,
    pub inherited_permissions: bool,
    pub permission_overrides: Vec<String>,
    pub depth_level: i32,
    pub active: bool,
    pub priority: i32,
    pub conditions: Option<String>,
    pub delegation_expires_at: Option<DateTime<Utc>>,
    pub created_by: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Defines methods for RoleHierarchy
impl RoleHierarchy {
    /// Creates new RoleHierarchy instance
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier
    /// * `parent_role_id` - Parent role ID
    /// * `child_role_id` - Child role ID
    /// * `hierarchy_type` - Type of hierarchy as string
    /// * `inherited_permissions` - Whether permissions are inherited
    /// * `permission_overrides` - List of permission override IDs
    /// * `depth_level` - Depth level in hierarchy
    /// * `active` - Whether hierarchy is active
    /// * `priority` - Priority order
    /// * `conditions` - Optional conditions
    /// * `delegation_expires_at` - Optional delegation expiration
    /// * `created_by` - Creator user ID
    ///
    /// # Returns
    ///
    /// New RoleHierarchy instance
    pub fn new(
        id: String,
        parent_role_id: String,
        child_role_id: String,
        hierarchy_type: String,
        inherited_permissions: bool,
        permission_overrides: Vec<String>,
        depth_level: i32,
        active: bool,
        priority: i32,
        conditions: Option<String>,
        delegation_expires_at: Option<DateTime<Utc>>,
        created_by: String
    ) -> Result<Self, AppError> {
        let now = Utc::now();

        if parent_role_id.trim().is_empty() {
            return Err(AppError::ValidationError("Parent role ID cannot be empty".to_string()));
        }

        if child_role_id.trim().is_empty() {
            return Err(AppError::ValidationError("Child role ID cannot be empty".to_string()));
        }

        if parent_role_id == child_role_id {
            return Err(
                AppError::ValidationError("Parent and child role cannot be the same".to_string())
            );
        }

        if created_by.trim().is_empty() {
            return Err(AppError::ValidationError("Created by cannot be empty".to_string()));
        }

        let hierarchy_type_enum = HierarchyType::from_string(&hierarchy_type)?;

        Ok(Self {
            id,
            parent_role_id,
            child_role_id,
            hierarchy_type: hierarchy_type_enum,
            inherited_permissions,
            permission_overrides,
            depth_level,
            active,
            priority,
            conditions,
            delegation_expires_at,
            created_by,
            created_at: now,
            updated_at: now,
        })
    }

    /// Creates RoleHierarchy instance from DynamoDB item
    ///
    /// # Arguments
    ///
    /// * `item` - The dynamo db item
    ///
    /// # Returns
    ///
    /// 'Some' RoleHierarchy if item fields match, 'None' otherwise
    pub(crate)  fn from_item(item: &HashMap<String, AttributeValue>) -> Option<Self> {
        info!("calling from_item with: {:?}", &item);

        let id = item.get("id")?.as_s().ok()?.to_string();
        let parent_role_id = item.get("parent_role_id")?.as_s().ok()?.to_string();
        let child_role_id = item.get("child_role_id")?.as_s().ok()?.to_string();

        let hierarchy_type_str = item.get("hierarchy_type")?.as_s().ok()?;
        let hierarchy_type = HierarchyType::from_string(&hierarchy_type_str)
            .map_err(|e| e)
            .ok()?;

        let inherited_permissions = item
            .get("inherited_permissions")
            .and_then(|v| v.as_bool().ok())
            .unwrap_or(&true);

        let permission_overrides = item
            .get("permission_overrides")
            .and_then(|v| v.as_ss().ok())
            .map(|ids|
                ids
                    .iter()
                    .map(|s| s.to_string())
                    .collect()
            )
            .unwrap_or_default();

        let depth_level = item
            .get("depth_level")
            .and_then(|v| v.as_n().ok())
            .and_then(|s| s.parse::<i32>().ok())
            .unwrap_or(0);

        let active = item
            .get("active")
            .and_then(|v| v.as_bool().ok())
            .unwrap_or(&true);

        let priority = item
            .get("priority")
            .and_then(|v| v.as_n().ok())
            .and_then(|s| s.parse::<i32>().ok())
            .unwrap_or(0);

        let conditions = item
            .get("conditions")
            .and_then(|v| v.as_s().ok())
            .map(|s| s.to_string());

        let delegation_expires_at = item
            .get("delegation_expires_at")
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
            parent_role_id,
            child_role_id,
            hierarchy_type,
            inherited_permissions,
            permission_overrides,
            depth_level,
            active,
            priority,
            conditions,
            delegation_expires_at,
            created_by,
            created_at,
            updated_at,
        });

        info!("result of from_item on role_hierarchy: {:?}", res);
        res
    }

    /// Creates DynamoDB item from RoleHierarchy instance
    ///
    /// # Arguments
    ///
    /// * `self` - borrowed instance of self
    ///
    /// # Returns
    ///
    /// HashMap representing DB item for RoleHierarchy instance
    pub(crate)  fn to_item(&self) -> HashMap<String, AttributeValue> {
        let mut item = HashMap::new();

        item.insert("id".to_string(), AttributeValue::S(self.id.clone()));
        item.insert("parent_role_id".to_string(), AttributeValue::S(self.parent_role_id.clone()));
        item.insert("child_role_id".to_string(), AttributeValue::S(self.child_role_id.clone()));
        item.insert(
            "hierarchy_type".to_string(),
            AttributeValue::S(self.hierarchy_type.to_str().to_string())
        );
        item.insert(
            "inherited_permissions".to_string(),
            AttributeValue::Bool(self.inherited_permissions)
        );

        // Store permission overrides as string set
        if !self.permission_overrides.is_empty() {
            item.insert(
                "permission_overrides".to_string(),
                AttributeValue::Ss(self.permission_overrides.clone())
            );
        }

        item.insert("depth_level".to_string(), AttributeValue::N(self.depth_level.to_string()));
        item.insert("active".to_string(), AttributeValue::Bool(self.active));
        item.insert("priority".to_string(), AttributeValue::N(self.priority.to_string()));

        if let Some(cond) = &self.conditions {
            item.insert("conditions".to_string(), AttributeValue::S(cond.clone()));
        }

        if let Some(expires) = &self.delegation_expires_at {
            item.insert(
                "delegation_expires_at".to_string(),
                AttributeValue::S(expires.to_string())
            );
        }

        item.insert("created_by".to_string(), AttributeValue::S(self.created_by.clone()));
        item.insert("created_at".to_string(), AttributeValue::S(self.created_at.to_string()));
        item.insert("updated_at".to_string(), AttributeValue::S(self.updated_at.to_string()));

        item
    }

    /// Checks if this hierarchy relationship has expired (for delegated relationships)
     fn is_expired(&self) -> bool {
        if let Some(expires_at) = &self.delegation_expires_at {
            Utc::now() > *expires_at
        } else {
            false
        }
    }

    /// Checks if this hierarchy relationship is currently effective
     fn is_effective(&self) -> bool {
        self.active && !self.is_expired()
    }

    /// Adds a permission override to this hierarchy
     fn add_permission_override(&mut self, permission_id: String) {
        if !self.permission_overrides.contains(&permission_id) {
            self.permission_overrides.push(permission_id);
            self.updated_at = Utc::now();
        }
    }

    /// Removes a permission override from this hierarchy
    fn remove_permission_override(&mut self, permission_id: &str) {
        if let Some(pos) = self.permission_overrides.iter().position(|x| x == permission_id) {
            self.permission_overrides.remove(pos);
            self.updated_at = Utc::now();
        }
    }

    /// Checks if a specific permission is overridden in this hierarchy
     fn has_permission_override(&self, permission_id: &str) -> bool {
        self.permission_overrides.contains(&permission_id.to_string())
    }
}
