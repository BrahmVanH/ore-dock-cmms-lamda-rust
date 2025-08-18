use std::collections::HashMap;

use async_graphql::Enum;
use aws_sdk_dynamodb::types::AttributeValue;
use chrono::{ DateTime, Utc };
use serde::{ Deserialize, Serialize };
use tracing::info;

use crate::error::AppError;

#[derive(Enum, Copy, Clone, PartialEq, Eq, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ElevationStatus {
    Pending, // Waiting for approval
    Approved, // Approved and active
    Active, // Currently in effect
    Expired, // Time period has ended
    Revoked, // Manually revoked before expiration
    Denied, // Request was denied
    Cancelled, // Cancelled by requester
}

impl ElevationStatus {
    pub(crate) fn to_str(&self) -> &str {
        match self {
            ElevationStatus::Pending => "pending",
            ElevationStatus::Approved => "approved",
            ElevationStatus::Active => "active",
            ElevationStatus::Expired => "expired",
            ElevationStatus::Revoked => "revoked",
            ElevationStatus::Denied => "denied",
            ElevationStatus::Cancelled => "cancelled",
        }
    }

    pub(crate) fn to_string(&self) -> String {
        self.to_str().to_string()
    }

    pub(crate) fn from_string(s: &str) -> Result<ElevationStatus, AppError> {
        match s {
            "pending" => Ok(Self::Pending),
            "approved" => Ok(Self::Approved),
            "active" => Ok(Self::Active),
            "expired" => Ok(Self::Expired),
            "revoked" => Ok(Self::Revoked),
            "denied" => Ok(Self::Denied),
            "cancelled" => Ok(Self::Cancelled),
            _ => Err(AppError::ValidationError("Invalid elevation status".to_string())),
        }
    }
}

#[derive(Enum, Copy, Clone, PartialEq, Eq, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ElevationPriority {
    Low,
    Normal,
    High,
    Emergency,
}

impl ElevationPriority {
    pub(crate) fn to_str(&self) -> &str {
        match self {
            ElevationPriority::Low => "low",
            ElevationPriority::Normal => "normal",
            ElevationPriority::High => "high",
            ElevationPriority::Emergency => "emergency",
        }
    }

    pub(crate) fn to_string(&self) -> String {
        self.to_str().to_string()
    }

    pub(crate) fn from_string(s: &str) -> Result<ElevationPriority, AppError> {
        match s {
            "low" => Ok(Self::Low),
            "normal" => Ok(Self::Normal),
            "high" => Ok(Self::High),
            "emergency" => Ok(Self::Emergency),
            _ => Err(AppError::ValidationError("Invalid elevation priority".to_string())),
        }
    }
}

/// Represents a Temporary Role Elevation in the system
///
/// # Fields
///
/// * `id` - Unique identifier for the elevation request
/// * `user_id` - ID of the user requesting elevation
/// * `original_role_id` - ID of the user's current role
/// * `elevated_role_id` - ID of the role being elevated to
/// * `reason` - Reason for the elevation request
/// * `justification` - Detailed justification for the request
/// * `requested_by_user_id` - ID of the user making the request (may differ from user_id)
/// * `approved_by_user_id` - ID of the user who approved the request
/// * `start_time` - When the elevation should/did start
/// * `end_time` - When the elevation should/will end
/// * `actual_start_time` - When the elevation actually started
/// * `actual_end_time` - When the elevation actually ended
/// * `status` - Current status of the elevation
/// * `priority` - Priority level of the request
/// * `auto_revoke` - Whether to automatically revoke at end_time
/// * `notification_sent` - Whether notifications have been sent
/// * `approval_required` - Whether approval is required for this elevation
/// * `approval_deadline` - Deadline for approval decision
/// * `revoked_by_user_id` - ID of user who revoked the elevation
/// * `revocation_reason` - Reason for revocation
/// * `created_at` - Date and time of creation
/// * `updated_at` - Date and time of last update
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TempRoleElevation {
    pub id: String,
    pub user_id: String,
    pub original_role_id: String,
    pub elevated_role_id: String,
    pub reason: Option<String>,
    pub justification: Option<String>,
    pub requested_by_user_id: String,
    pub approved_by_user_id: Option<String>,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub actual_start_time: Option<DateTime<Utc>>,
    pub actual_end_time: Option<DateTime<Utc>>,
    pub status: ElevationStatus,
    pub priority: ElevationPriority,
    pub auto_revoke: bool,
    pub notification_sent: bool,
    pub approval_required: bool,
    pub approval_deadline: Option<DateTime<Utc>>,
    pub revoked_by_user_id: Option<String>,
    pub revocation_reason: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Defines methods for TempRoleElevation
impl TempRoleElevation {
    /// Creates new TempRoleElevation instance
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier
    /// * `user_id` - User ID requesting elevation
    /// * `original_role_id` - Current role ID
    /// * `elevated_role_id` - Target role ID
    /// * `reason` - Optional reason
    /// * `justification` - Optional detailed justification
    /// * `requested_by_user_id` - Requester user ID
    /// * `start_time` - Elevation start time
    /// * `end_time` - Elevation end time
    /// * `priority` - Priority as string
    /// * `auto_revoke` - Whether to auto-revoke
    /// * `approval_required` - Whether approval is required
    /// * `approval_deadline` - Optional approval deadline
    ///
    /// # Returns
    ///
    /// New TempRoleElevation instance
    pub fn new(
        id: String,
        user_id: String,
        original_role_id: String,
        elevated_role_id: String,
        reason: Option<String>,
        justification: Option<String>,
        requested_by_user_id: String,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
        priority: String,
        auto_revoke: bool,
        approval_required: bool,
        approval_deadline: Option<DateTime<Utc>>
    ) -> Result<Self, AppError> {
        let now = Utc::now();

        if user_id.trim().is_empty() {
            return Err(AppError::ValidationError("User ID cannot be empty".to_string()));
        }

        if original_role_id.trim().is_empty() {
            return Err(AppError::ValidationError("Original role ID cannot be empty".to_string()));
        }

        if elevated_role_id.trim().is_empty() {
            return Err(AppError::ValidationError("Elevated role ID cannot be empty".to_string()));
        }

        if requested_by_user_id.trim().is_empty() {
            return Err(
                AppError::ValidationError("Requested by user ID cannot be empty".to_string())
            );
        }

        if end_time <= start_time {
            return Err(AppError::ValidationError("End time must be after start time".to_string()));
        }

        let priority_enum = ElevationPriority::from_string(&priority)?;

        let status = if approval_required {
            ElevationStatus::Pending
        } else {
            ElevationStatus::Approved
        };

        Ok(Self {
            id,
            user_id,
            original_role_id,
            elevated_role_id,
            reason,
            justification,
            requested_by_user_id,
            approved_by_user_id: None,
            start_time,
            end_time,
            actual_start_time: None,
            actual_end_time: None,
            status,
            priority: priority_enum,
            auto_revoke,
            notification_sent: false,
            approval_required,
            approval_deadline,
            revoked_by_user_id: None,
            revocation_reason: None,
            created_at: now,
            updated_at: now,
        })
    }

    /// Creates TempRoleElevation instance from DynamoDB item
    ///
    /// # Arguments
    ///
    /// * `item` - The dynamo db item
    ///
    /// # Returns
    ///
    /// 'Some' TempRoleElevation if item fields match, 'None' otherwise
    pub(crate) fn from_item(item: &HashMap<String, AttributeValue>) -> Option<Self> {
        info!("calling from_item with: {:?}", &item);

        let id = item.get("id")?.as_s().ok()?.to_string();
        let user_id = item.get("user_id")?.as_s().ok()?.to_string();
        let original_role_id = item.get("original_role_id")?.as_s().ok()?.to_string();
        let elevated_role_id = item.get("elevated_role_id")?.as_s().ok()?.to_string();

        let reason = item
            .get("reason")
            .and_then(|v| v.as_s().ok())
            .map(|s| s.to_string());

        let justification = item
            .get("justification")
            .and_then(|v| v.as_s().ok())
            .map(|s| s.to_string());

        let requested_by_user_id = item.get("requested_by_user_id")?.as_s().ok()?.to_string();

        let approved_by_user_id = item
            .get("approved_by_user_id")
            .and_then(|v| v.as_s().ok())
            .map(|s| s.to_string());

        let start_time = item
            .get("start_time")
            .and_then(|v| v.as_s().ok())
            .and_then(|s| s.parse::<DateTime<Utc>>().ok())
            .unwrap_or_else(|| Utc::now());

        let end_time = item
            .get("end_time")
            .and_then(|v| v.as_s().ok())
            .and_then(|s| s.parse::<DateTime<Utc>>().ok())
            .unwrap_or_else(|| Utc::now());

        let actual_start_time = item
            .get("actual_start_time")
            .and_then(|v| v.as_s().ok())
            .and_then(|s| s.parse::<DateTime<Utc>>().ok());

        let actual_end_time = item
            .get("actual_end_time")
            .and_then(|v| v.as_s().ok())
            .and_then(|s| s.parse::<DateTime<Utc>>().ok());

        let status_str = item.get("status")?.as_s().ok()?;
        let status = ElevationStatus::from_string(&status_str)
            .map_err(|e| e)
            .ok()?;

        let priority_str = item.get("priority")?.as_s().ok()?;
        let priority = ElevationPriority::from_string(&priority_str)
            .map_err(|e| e)
            .ok()?;

        let auto_revoke = item
            .get("auto_revoke")
            .and_then(|v| v.as_bool().ok())
            .unwrap_or(&true);

        let notification_sent = item
            .get("notification_sent")
            .and_then(|v| v.as_bool().ok())
            .unwrap_or(&false);

        let approval_required = item
            .get("approval_required")
            .and_then(|v| v.as_bool().ok())
            .unwrap_or(&true);

        let approval_deadline = item
            .get("approval_deadline")
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
            original_role_id,
            elevated_role_id,
            reason,
            justification,
            requested_by_user_id,
            approved_by_user_id,
            start_time,
            end_time,
            actual_start_time,
            actual_end_time,
            status,
            priority,
            auto_revoke: *auto_revoke,
            notification_sent: *notification_sent,
            approval_required: *approval_required,
            approval_deadline,
            revoked_by_user_id,
            revocation_reason,
            created_at,
            updated_at,
        });

        info!("result of from_item on temp_role_elevation: {:?}", res);
        res
    }

    /// Creates DynamoDB item from TempRoleElevation instance
    ///
    /// # Arguments
    ///
    /// * `self` - borrowed instance of self
    ///
    /// # Returns
    ///
    /// HashMap representing DB item for TempRoleElevation instance
    pub(crate) fn to_item(&self) -> HashMap<String, AttributeValue> {
        let mut item = HashMap::new();

        item.insert("id".to_string(), AttributeValue::S(self.id.clone()));
        item.insert("user_id".to_string(), AttributeValue::S(self.user_id.clone()));
        item.insert(
            "original_role_id".to_string(),
            AttributeValue::S(self.original_role_id.clone())
        );
        item.insert(
            "elevated_role_id".to_string(),
            AttributeValue::S(self.elevated_role_id.clone())
        );

        if let Some(reason) = &self.reason {
            item.insert("reason".to_string(), AttributeValue::S(reason.clone()));
        }

        if let Some(justification) = &self.justification {
            item.insert("justification".to_string(), AttributeValue::S(justification.clone()));
        }

        item.insert(
            "requested_by_user_id".to_string(),
            AttributeValue::S(self.requested_by_user_id.clone())
        );

        if let Some(approved_by) = &self.approved_by_user_id {
            item.insert("approved_by_user_id".to_string(), AttributeValue::S(approved_by.clone()));
        }

        item.insert("start_time".to_string(), AttributeValue::S(self.start_time.to_string()));
        item.insert("end_time".to_string(), AttributeValue::S(self.end_time.to_string()));

        if let Some(actual_start) = &self.actual_start_time {
            item.insert(
                "actual_start_time".to_string(),
                AttributeValue::S(actual_start.to_string())
            );
        }

        if let Some(actual_end) = &self.actual_end_time {
            item.insert("actual_end_time".to_string(), AttributeValue::S(actual_end.to_string()));
        }

        item.insert("status".to_string(), AttributeValue::S(self.status.to_string()));
        item.insert("priority".to_string(), AttributeValue::S(self.priority.to_string()));
        item.insert("auto_revoke".to_string(), AttributeValue::Bool(self.auto_revoke));
        item.insert("notification_sent".to_string(), AttributeValue::Bool(self.notification_sent));
        item.insert("approval_required".to_string(), AttributeValue::Bool(self.approval_required));

        if let Some(deadline) = &self.approval_deadline {
            item.insert("approval_deadline".to_string(), AttributeValue::S(deadline.to_string()));
        }

        if let Some(revoked_by) = &self.revoked_by_user_id {
            item.insert("revoked_by_user_id".to_string(), AttributeValue::S(revoked_by.clone()));
        }

        if let Some(revocation_reason) = &self.revocation_reason {
            item.insert(
                "revocation_reason".to_string(),
                AttributeValue::S(revocation_reason.clone())
            );
        }

        item.insert("created_at".to_string(), AttributeValue::S(self.created_at.to_string()));
        item.insert("updated_at".to_string(), AttributeValue::S(self.updated_at.to_string()));

        item
    }

    /// Checks if the elevation is currently active
    fn is_active(&self) -> bool {
        let now = Utc::now();
        matches!(self.status, ElevationStatus::Active) &&
            now >= self.start_time &&
            now <= self.end_time
    }

    /// Checks if the elevation has expired
    fn is_expired(&self) -> bool {
        Utc::now() > self.end_time
    }

    /// Checks if the elevation is pending approval
    fn is_pending(&self) -> bool {
        matches!(self.status, ElevationStatus::Pending)
    }

    /// Activates the elevation
    fn activate(&mut self) -> Result<(), AppError> {
        if !matches!(self.status, ElevationStatus::Approved) {
            return Err(
                AppError::ValidationError(
                    "Elevation must be approved before activation".to_string()
                )
            );
        }

        self.status = ElevationStatus::Active;
        self.actual_start_time = Some(Utc::now());
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Revokes the elevation
    fn revoke(
        &mut self,
        revoked_by_user_id: String,
        reason: Option<String>
    ) -> Result<(), AppError> {
        if matches!(self.status, ElevationStatus::Expired | ElevationStatus::Revoked) {
            return Err(
                AppError::ValidationError("Elevation is already expired or revoked".to_string())
            );
        }

        self.status = ElevationStatus::Revoked;
        self.actual_end_time = Some(Utc::now());
        self.revoked_by_user_id = Some(revoked_by_user_id);
        self.revocation_reason = reason;
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Approves the elevation
    fn approve(&mut self, approved_by_user_id: String) -> Result<(), AppError> {
        if !matches!(self.status, ElevationStatus::Pending) {
            return Err(
                AppError::ValidationError("Only pending elevations can be approved".to_string())
            );
        }

        self.status = ElevationStatus::Approved;
        self.approved_by_user_id = Some(approved_by_user_id);
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Denies the elevation
    fn deny(&mut self, denied_by_user_id: String, reason: Option<String>) -> Result<(), AppError> {
        if !matches!(self.status, ElevationStatus::Pending) {
            return Err(
                AppError::ValidationError("Only pending elevations can be denied".to_string())
            );
        }

        self.status = ElevationStatus::Denied;
        self.approved_by_user_id = Some(denied_by_user_id);
        self.revocation_reason = reason;
        self.updated_at = Utc::now();
        Ok(())
    }
}
