use std::collections::HashMap;

use async_graphql::{Enum, Object};
use aws_sdk_dynamodb::types::AttributeValue;
use chrono::{ DateTime, Utc };
use serde::{ Deserialize, Serialize };
use tracing::info;

use crate::{ error::AppError, DynamoDbEntity };

#[derive(Enum, Copy, Clone, PartialEq, Eq, Debug, Deserialize, Serialize)]

#[serde(rename_all = "snake_case")]
pub enum RoleAssignmentStatus {
    Active, // Currently active assignment
    Suspended, // Temporarily suspended
    Expired, // Assignment has expired
    Revoked, // Manually revoked before expiration
    Pending, // Pending activation
}

impl RoleAssignmentStatus {
    pub(crate) fn to_str(&self) -> &str {
        match self {
            RoleAssignmentStatus::Active => "active",
            RoleAssignmentStatus::Suspended => "suspended",
            RoleAssignmentStatus::Expired => "expired",
            RoleAssignmentStatus::Revoked => "revoked",
            RoleAssignmentStatus::Pending => "pending",
        }
    }

    pub(crate) fn to_string(&self) -> String {
        self.to_str().to_string()
    }

    pub(crate) fn from_string(s: &str) -> Result<RoleAssignmentStatus, AppError> {
        match s {
            "active" => Ok(Self::Active),
            "suspended" => Ok(Self::Suspended),
            "expired" => Ok(Self::Expired),
            "revoked" => Ok(Self::Revoked),
            "pending" => Ok(Self::Pending),
            _ => Err(AppError::ValidationError("Invalid role assignment status".to_string())),
        }
    }
}

#[derive(Enum, Copy, Clone, PartialEq, Eq, Debug, Deserialize, Serialize)]

#[serde(rename_all = "snake_case")]
pub enum AssignmentSource {
    Manual, // Manually assigned by administrator
    Automatic, // Automatically assigned by system rules
    Elevation, // Temporary elevation from temp_role_elevation
    Inheritance, // Inherited from group or hierarchy
    Import, // Imported from external system
}

impl AssignmentSource {
    pub(crate) fn to_str(&self) -> &str {
        match self {
            AssignmentSource::Manual => "manual",
            AssignmentSource::Automatic => "automatic",
            AssignmentSource::Elevation => "elevation",
            AssignmentSource::Inheritance => "inheritance",
            AssignmentSource::Import => "import",
        }
    }

    pub(crate) fn to_string(&self) -> String {
        self.to_str().to_string()
    }

    pub(crate) fn from_string(s: &str) -> Result<AssignmentSource, AppError> {
        match s {
            "manual" => Ok(Self::Manual),
            "automatic" => Ok(Self::Automatic),
            "elevation" => Ok(Self::Elevation),
            "inheritance" => Ok(Self::Inheritance),
            "import" => Ok(Self::Import),
            _ => Err(AppError::ValidationError("Invalid assignment source".to_string())),
        }
    }
}

/// Represents a User Role assignment in the system
///
/// # Fields
///
/// * `id` - Unique identifier for this role assignment
/// * `user_id` - ID of the user assigned to the role
/// * `role_id` - ID of the role being assigned
/// * `assignment_source` - How this role was assigned (manual, automatic, etc.)
/// * `status` - Current status of the role assignment
/// * `is_primary_role` - Whether this is the user's primary role
/// * `assigned_at` - Date and time when role was assigned
/// * `assigned_by_user_id` - ID of the user who made the assignment
/// * `effective_from` - When the role assignment becomes effective
/// * `expires_at` - Optional expiration date for the role assignment
/// * `last_used_at` - When this role was last actively used
/// * `conditions` - Optional JSON conditions for when role applies
/// * `elevation_request_id` - ID of temp elevation request if applicable
/// * `revoked_at` - When the role was revoked
/// * `revoked_by_user_id` - ID of user who revoked the role
/// * `revocation_reason` - Reason for revocation
/// * `metadata` - Additional metadata about the assignment
/// * `created_at` - Date and time of creation
/// * `updated_at` - Date and time of last update
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserRole {
    pub id: String,
    pub user_id: String,
    pub role_id: String,
    pub assignment_source: AssignmentSource,
    pub status: RoleAssignmentStatus,
    pub is_primary_role: bool,
    pub assigned_at: DateTime<Utc>,
    pub assigned_by_user_id: Option<String>,
    pub effective_from: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub last_used_at: Option<DateTime<Utc>>,
    pub conditions: Option<String>,
    pub elevation_request_id: Option<String>,
    pub revoked_at: Option<DateTime<Utc>>,
    pub revoked_by_user_id: Option<String>,
    pub revocation_reason: Option<String>,
    pub metadata: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Defines methods for UserRole
impl UserRole {
    /// Creates new UserRole instance
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier
    /// * `user_id` - User ID being assigned the role
    /// * `role_id` - Role ID being assigned
    /// * `assignment_source` - How role was assigned as string
    /// * `is_primary_role` - Whether this is primary role
    /// * `assigned_by_user_id` - Optional assigner user ID
    /// * `effective_from` - When assignment becomes effective
    /// * `expires_at` - Optional expiration time
    /// * `conditions` - Optional conditions
    /// * `elevation_request_id` - Optional elevation request ID
    /// * `metadata` - Optional metadata
    ///
    /// # Returns
    ///
    /// New UserRole instance
    pub fn new(
        id: String,
        user_id: String,
        role_id: String,
        assignment_source: String,
        is_primary_role: bool,
        assigned_by_user_id: Option<String>,
        effective_from: DateTime<Utc>,
        expires_at: Option<DateTime<Utc>>,
        conditions: Option<String>,
        elevation_request_id: Option<String>,
        metadata: Option<String>
    ) -> Result<Self, AppError> {
        let now = Utc::now();

        if user_id.trim().is_empty() {
            return Err(AppError::ValidationError("User ID cannot be empty".to_string()));
        }

        if role_id.trim().is_empty() {
            return Err(AppError::ValidationError("Role ID cannot be empty".to_string()));
        }

        let source_enum = AssignmentSource::from_string(&assignment_source)?;

        // Validate effective_from is not in the future for active assignments
        let status = if effective_from > now {
            RoleAssignmentStatus::Pending
        } else {
            RoleAssignmentStatus::Active
        };

        Ok(Self {
            id,
            user_id,
            role_id,
            assignment_source: source_enum,
            status,
            is_primary_role,
            assigned_at: now,
            assigned_by_user_id,
            effective_from,
            expires_at,
            last_used_at: None,
            conditions,
            elevation_request_id,
            revoked_at: None,
            revoked_by_user_id: None,
            revocation_reason: None,
            metadata,
            created_at: now,
            updated_at: now,
        })
    }

    /// Checks if the role assignment is currently active and effective
    pub(crate) fn is_effective(&self) -> bool {
        let now = Utc::now();

        // Must be active status
        if !matches!(self.status, RoleAssignmentStatus::Active) {
            return false;
        }

        // Must be past effective_from time
        if now < self.effective_from {
            return false;
        }

        // Must not be expired
        if let Some(expires_at) = &self.expires_at {
            if now > *expires_at {
                return false;
            }
        }

        // Must not be revoked
        if self.revoked_at.is_some() {
            return false;
        }

        true
    }

    /// Checks if the role assignment has expired
    fn is_expired(&self) -> bool {
        if let Some(expires_at) = &self.expires_at { Utc::now() > *expires_at } else { false }
    }

    /// Updates the last used timestamp
    pub fn mark_as_used(&mut self) {
        self.last_used_at = Some(Utc::now());
        self.updated_at = Utc::now();
    }

    /// Revokes the role assignment
    fn revoke(
        &mut self,
        revoked_by_user_id: String,
        reason: Option<String>
    ) -> Result<(), AppError> {
        if matches!(self.status, RoleAssignmentStatus::Revoked) {
            return Err(AppError::ValidationError("Role assignment is already revoked".to_string()));
        }

        self.status = RoleAssignmentStatus::Revoked;
        self.revoked_at = Some(Utc::now());
        self.revoked_by_user_id = Some(revoked_by_user_id);
        self.revocation_reason = reason;
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Suspends the role assignment temporarily
    fn suspend(
        &mut self,
        suspended_by_user_id: String,
        reason: Option<String>
    ) -> Result<(), AppError> {
        if !matches!(self.status, RoleAssignmentStatus::Active) {
            return Err(
                AppError::ValidationError(
                    "Only active role assignments can be suspended".to_string()
                )
            );
        }

        self.status = RoleAssignmentStatus::Suspended;
        self.revoked_by_user_id = Some(suspended_by_user_id);
        self.revocation_reason = reason;
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Reactivates a suspended role assignment
    fn reactivate(&mut self) -> Result<(), AppError> {
        if !matches!(self.status, RoleAssignmentStatus::Suspended) {
            return Err(
                AppError::ValidationError(
                    "Only suspended role assignments can be reactivated".to_string()
                )
            );
        }

        self.status = RoleAssignmentStatus::Active;
        self.revoked_by_user_id = None;
        self.revocation_reason = None;
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Extends the expiration date of the role assignment
    fn extend_expiration(&mut self, new_expiration: DateTime<Utc>) -> Result<(), AppError> {
        if new_expiration <= Utc::now() {
            return Err(
                AppError::ValidationError("New expiration must be in the future".to_string())
            );
        }

        self.expires_at = Some(new_expiration);
        self.updated_at = Utc::now();
        Ok(())
    }
}

impl DynamoDbEntity for UserRole {
    fn table_name() -> &'static str {
        "UserRoles"
    }

    fn primary_key(&self) -> String {
        self.id.clone()
    }

    /// Creates UserRole instance from DynamoDB item
    ///
    /// # Arguments
    ///
    /// * `item` - The dynamo db item
    ///
    /// # Returns
    ///
    /// 'Some' UserRole if item fields match, 'None' otherwise
    fn from_item(item: &HashMap<String, AttributeValue>) -> Option<Self> {
        info!("calling from_item with: {:?}", &item);

        let id = item.get("id")?.as_s().ok()?.to_string();
        let user_id = item.get("user_id")?.as_s().ok()?.to_string();
        let role_id = item.get("role_id")?.as_s().ok()?.to_string();

        let assignment_source_str = item.get("assignment_source")?.as_s().ok()?;
        let assignment_source = AssignmentSource::from_string(&assignment_source_str)
            .map_err(|e| e)
            .ok()?;

        let status_str = item.get("status")?.as_s().ok()?;
        let status = RoleAssignmentStatus::from_string(&status_str)
            .map_err(|e| e)
            .ok()?;

        let is_primary_role = item
            .get("is_primary_role")
            .and_then(|v| v.as_bool().ok())
            .unwrap_or(&false);

        let assigned_at = item
            .get("assigned_at")
            .and_then(|v| v.as_s().ok())
            .and_then(|s| s.parse::<DateTime<Utc>>().ok())
            .unwrap_or_else(|| Utc::now());

        let assigned_by_user_id = item
            .get("assigned_by_user_id")
            .and_then(|v| v.as_s().ok())
            .map(|s| s.to_string());

        let effective_from = item
            .get("effective_from")
            .and_then(|v| v.as_s().ok())
            .and_then(|s| s.parse::<DateTime<Utc>>().ok())
            .unwrap_or_else(|| Utc::now());

        let expires_at = item
            .get("expires_at")
            .and_then(|v| v.as_s().ok())
            .and_then(|s| s.parse::<DateTime<Utc>>().ok());

        let last_used_at = item
            .get("last_used_at")
            .and_then(|v| v.as_s().ok())
            .and_then(|s| s.parse::<DateTime<Utc>>().ok());

        let conditions = item
            .get("conditions")
            .and_then(|v| v.as_s().ok())
            .map(|s| s.to_string());

        let elevation_request_id = item
            .get("elevation_request_id")
            .and_then(|v| v.as_s().ok())
            .map(|s| s.to_string());

        let revoked_at = item
            .get("revoked_at")
            .and_then(|v| v.as_s().ok())
            .and_then(|s| s.parse::<DateTime<Utc>>().ok());

        let revoked_by_user_id = item
            .get("revoked_by_user_id")
            .and_then(|v| v.as_s().ok())
            .map(|s| s.to_string());

        let revocation_reason = item
            .get("revocation_reason")
            .and_then(|v| v.as_s().ok())
            .map(|s| s.to_string());

        let metadata = item
            .get("metadata")
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
            role_id,
            assignment_source,
            status,
            is_primary_role: *is_primary_role,
            assigned_at,
            assigned_by_user_id,
            effective_from,
            expires_at,
            last_used_at,
            conditions,
            elevation_request_id,
            revoked_at,
            revoked_by_user_id,
            revocation_reason,
            metadata,
            created_at,
            updated_at,
        });

        info!("result of from_item on user_role: {:?}", res);
        res
    }

    /// Creates DynamoDB item from UserRole instance
    ///
    /// # Arguments
    ///
    /// * `self` - borrowed instance of self
    ///
    /// # Returns
    ///
    /// HashMap representing DB item for UserRole instance
    fn to_item(&self) -> HashMap<String, AttributeValue> {
        let mut item = HashMap::new();

        item.insert("id".to_string(), AttributeValue::S(self.id.clone()));
        item.insert("user_id".to_string(), AttributeValue::S(self.user_id.clone()));
        item.insert("role_id".to_string(), AttributeValue::S(self.role_id.clone()));
        item.insert(
            "assignment_source".to_string(),
            AttributeValue::S(self.assignment_source.to_str().to_string())
        );
        item.insert("status".to_string(), AttributeValue::S(self.status.to_str().to_string()));
        item.insert("is_primary_role".to_string(), AttributeValue::Bool(self.is_primary_role));
        item.insert("assigned_at".to_string(), AttributeValue::S(self.assigned_at.to_string()));

        if let Some(assigned_by) = &self.assigned_by_user_id {
            item.insert("assigned_by_user_id".to_string(), AttributeValue::S(assigned_by.clone()));
        }

        item.insert(
            "effective_from".to_string(),
            AttributeValue::S(self.effective_from.to_string())
        );

        if let Some(expires) = &self.expires_at {
            item.insert("expires_at".to_string(), AttributeValue::S(expires.to_string()));
        }

        if let Some(last_used) = &self.last_used_at {
            item.insert("last_used_at".to_string(), AttributeValue::S(last_used.to_string()));
        }

        if let Some(cond) = &self.conditions {
            item.insert("conditions".to_string(), AttributeValue::S(cond.clone()));
        }

        if let Some(elevation_id) = &self.elevation_request_id {
            item.insert(
                "elevation_request_id".to_string(),
                AttributeValue::S(elevation_id.clone())
            );
        }

        if let Some(revoked) = &self.revoked_at {
            item.insert("revoked_at".to_string(), AttributeValue::S(revoked.to_string()));
        }

        if let Some(revoked_by) = &self.revoked_by_user_id {
            item.insert("revoked_by_user_id".to_string(), AttributeValue::S(revoked_by.clone()));
        }

        if let Some(reason) = &self.revocation_reason {
            item.insert("revocation_reason".to_string(), AttributeValue::S(reason.clone()));
        }

        if let Some(meta) = &self.metadata {
            item.insert("metadata".to_string(), AttributeValue::S(meta.clone()));
        }

        item.insert("created_at".to_string(), AttributeValue::S(self.created_at.to_string()));
        item.insert("updated_at".to_string(), AttributeValue::S(self.updated_at.to_string()));

        item
    }
}
