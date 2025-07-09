use async_graphql::*;
use chrono::{DateTime, Utc};
use tracing::warn;

use crate::{
    error::AppError,
    models::{
        user_role::{UserRole, RoleAssignmentStatus, AssignmentSource},
        user::User,
        role::Role,
    },
    DbClient,
    Repository,
};

#[derive(Debug, Default)]
pub(crate) struct UserRoleQuery;

#[Object]
impl UserRoleQuery {
    /// Get user role assignment by ID
    async fn user_role_by_id(
        &self,
        ctx: &Context<'_>,
        id: String,
    ) -> Result<Option<UserRole>, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        repo.get::<UserRole>(id)
            .await
            .map_err(|e| e.to_graphql_error())
    }

    /// Get all user role assignments with filtering
    async fn user_roles(
        &self,
        ctx: &Context<'_>,
        limit: Option<i32>,
        status_filter: Option<String>,
        assignment_source_filter: Option<String>,
        primary_only: Option<bool>,
        effective_only: Option<bool>,
    ) -> Result<Vec<UserRole>, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        let mut user_roles = repo
            .list::<UserRole>(limit)
            .await
            .map_err(|e| e.to_graphql_error())?;

        // Apply status filter
        if let Some(status) = status_filter {
            let status_enum = RoleAssignmentStatus::from_string(&status)
                .map_err(|e| e.to_graphql_error())?;
            user_roles = user_roles
                .into_iter()
                .filter(|ur| ur.status.to_string() == status_enum.to_string())
                .collect();
        }

        // Apply assignment source filter
        if let Some(source) = assignment_source_filter {
            let source_enum = AssignmentSource::from_string(&source)
                .map_err(|e| e.to_graphql_error())?;
            user_roles = user_roles
                .into_iter()
                .filter(|ur| ur.assignment_source.to_string() == source_enum.to_string())
                .collect();
        }

        // Apply primary role filter
        if let Some(true) = primary_only {
            user_roles = user_roles
                .into_iter()
                .filter(|ur| ur.is_primary_role)
                .collect();
        }

        // Apply effective only filter
        if let Some(true) = effective_only {
            user_roles = user_roles
                .into_iter()
                .filter(|ur| ur.is_effective())
                .collect();
        }

        Ok(user_roles)
    }

    /// Get role assignments for a specific user
    async fn user_roles_for_user(
        &self,
        ctx: &Context<'_>,
        user_id: String,
        effective_only: Option<bool>,
        limit: Option<i32>,
    ) -> Result<Vec<UserRole>, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        // Verify user exists
        let _user = repo
            .get::<User>(user_id.clone())
            .await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| {
                AppError::NotFound(format!("User {} not found", user_id))
                    .to_graphql_error()
            })?;

        let mut user_roles = repo
            .list::<UserRole>(None)
            .await
            .map_err(|e| e.to_graphql_error())?;

        // Filter by user ID
        user_roles = user_roles
            .into_iter()
            .filter(|ur| ur.user_id == user_id)
            .collect();

        // Apply effective only filter
        if let Some(true) = effective_only {
            user_roles = user_roles
                .into_iter()
                .filter(|ur| ur.is_effective())
                .collect();
        }

        // Sort by priority (primary role first, then by assigned date)
        user_roles.sort_by(|a, b| {
            if a.is_primary_role && !b.is_primary_role {
                std::cmp::Ordering::Less
            } else if !a.is_primary_role && b.is_primary_role {
                std::cmp::Ordering::Greater
            } else {
                b.assigned_at.cmp(&a.assigned_at)
            }
        });

        // Apply limit if provided
        if let Some(limit_val) = limit {
            user_roles.truncate(limit_val as usize);
        }

        Ok(user_roles)
    }

    /// Get user assignments for a specific role
    async fn user_roles_for_role(
        &self,
        ctx: &Context<'_>,
        role_id: String,
        effective_only: Option<bool>,
        limit: Option<i32>,
    ) -> Result<Vec<UserRole>, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        // Verify role exists
        let _role = repo
            .get::<Role>(role_id.clone())
            .await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| {
                AppError::NotFound(format!("Role {} not found", role_id))
                    .to_graphql_error()
            })?;

        let mut user_roles = repo
            .list::<UserRole>(None)
            .await
            .map_err(|e| e.to_graphql_error())?;

        // Filter by role ID
        user_roles = user_roles
            .into_iter()
            .filter(|ur| ur.role_id == role_id)
            .collect();

        // Apply effective only filter
        if let Some(true) = effective_only {
            user_roles = user_roles
                .into_iter()
                .filter(|ur| ur.is_effective())
                .collect();
        }

        // Sort by assignment date (most recent first)
        user_roles.sort_by(|a, b| b.assigned_at.cmp(&a.assigned_at));

        // Apply limit if provided
        if let Some(limit_val) = limit {
            user_roles.truncate(limit_val as usize);
        }

        Ok(user_roles)
    }

    /// Get primary role assignments
    async fn primary_user_roles(
        &self,
        ctx: &Context<'_>,
        effective_only: Option<bool>,
        limit: Option<i32>,
    ) -> Result<Vec<UserRole>, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        let mut user_roles = repo
            .list::<UserRole>(None)
            .await
            .map_err(|e| e.to_graphql_error())?;

        // Filter for primary roles only
        user_roles = user_roles
            .into_iter()
            .filter(|ur| ur.is_primary_role)
            .collect();

        // Apply effective only filter
        if let Some(true) = effective_only {
            user_roles = user_roles
                .into_iter()
                .filter(|ur| ur.is_effective())
                .collect();
        }

        // Apply limit if provided
        if let Some(limit_val) = limit {
            user_roles.truncate(limit_val as usize);
        }

        Ok(user_roles)
    }

    /// Get role assignments by status
    async fn user_roles_by_status(
        &self,
        ctx: &Context<'_>,
        status: String,
        limit: Option<i32>,
    ) -> Result<Vec<UserRole>, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        let status_enum = RoleAssignmentStatus::from_string(&status)
            .map_err(|e| e.to_graphql_error())?;

        let mut user_roles = repo
            .list::<UserRole>(None)
            .await
            .map_err(|e| e.to_graphql_error())?;

        user_roles = user_roles
            .into_iter()
            .filter(|ur| ur.status.to_string() == status_enum.to_string())
            .collect();

        // Apply limit if provided
        if let Some(limit_val) = limit {
            user_roles.truncate(limit_val as usize);
        }

        Ok(user_roles)
    }

    /// Get role assignments by assignment source
    async fn user_roles_by_assignment_source(
        &self,
        ctx: &Context<'_>,
        assignment_source: String,
        limit: Option<i32>,
    ) -> Result<Vec<UserRole>, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        let source_enum = AssignmentSource::from_string(&assignment_source)
            .map_err(|e| e.to_graphql_error())?;

        let mut user_roles = repo
            .list::<UserRole>(None)
            .await
            .map_err(|e| e.to_graphql_error())?;

        user_roles = user_roles
            .into_iter()
            .filter(|ur| ur.assignment_source.to_string() == source_enum.to_string())
            .collect();

        // Apply limit if provided
        if let Some(limit_val) = limit {
            user_roles.truncate(limit_val as usize);
        }

        Ok(user_roles)
    }

    /// Get expired role assignments
    async fn expired_user_roles(
        &self,
        ctx: &Context<'_>,
        limit: Option<i32>,
    ) -> Result<Vec<UserRole>, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        let mut user_roles = repo
            .list::<UserRole>(None)
            .await
            .map_err(|e| e.to_graphql_error())?;

        let now = Utc::now();

        // Filter for expired assignments
        user_roles = user_roles
            .into_iter()
            .filter(|ur| {
                if let Some(expires_at) = ur.expires_at {
                    expires_at < now
                } else {
                    false
                }
            })
            .collect();

        // Sort by expiration date (most recently expired first)
        user_roles.sort_by(|a, b| {
            b.expires_at.unwrap_or_else(|| Utc::now())
                .cmp(&a.expires_at.unwrap_or_else(|| Utc::now()))
        });

        // Apply limit if provided
        if let Some(limit_val) = limit {
            user_roles.truncate(limit_val as usize);
        }

        Ok(user_roles)
    }

    /// Get role assignments expiring soon
    async fn user_roles_expiring_soon(
        &self,
        ctx: &Context<'_>,
        days_ahead: Option<i32>,
        limit: Option<i32>,
    ) -> Result<Vec<UserRole>, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        let mut user_roles = repo
            .list::<UserRole>(None)
            .await
            .map_err(|e| e.to_graphql_error())?;

        let days_ahead = days_ahead.unwrap_or(30);
        let cutoff_date = Utc::now() + chrono::Duration::days(days_ahead as i64);
        let now = Utc::now();

        // Filter for assignments expiring soon
        user_roles = user_roles
            .into_iter()
            .filter(|ur| {
                if let Some(expires_at) = ur.expires_at {
                    expires_at > now && expires_at <= cutoff_date
                } else {
                    false
                }
            })
            .collect();

        // Sort by expiration date (soonest first)
        user_roles.sort_by(|a, b| {
            a.expires_at.unwrap_or_else(|| Utc::now())
                .cmp(&b.expires_at.unwrap_or_else(|| Utc::now()))
        });

        // Apply limit if provided
        if let Some(limit_val) = limit {
            user_roles.truncate(limit_val as usize);
        }

        Ok(user_roles)
    }

    /// Get temporarily elevated role assignments
    async fn elevated_user_roles(
        &self,
        ctx: &Context<'_>,
        active_only: Option<bool>,
        limit: Option<i32>,
    ) -> Result<Vec<UserRole>, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        let mut user_roles = repo
            .list::<UserRole>(None)
            .await
            .map_err(|e| e.to_graphql_error())?;

        // Filter for elevation assignments
        user_roles = user_roles
            .into_iter()
            .filter(|ur| {
                matches!(ur.assignment_source, AssignmentSource::Elevation) ||
                ur.elevation_request_id.is_some()
            })
            .collect();

        // Apply active only filter
        if let Some(true) = active_only {
            user_roles = user_roles
                .into_iter()
                .filter(|ur| ur.is_effective())
                .collect();
        }

        // Sort by assignment date (most recent first)
        user_roles.sort_by(|a, b| b.assigned_at.cmp(&a.assigned_at));

        // Apply limit if provided
        if let Some(limit_val) = limit {
            user_roles.truncate(limit_val as usize);
        }

        Ok(user_roles)
    }

    /// Get suspended role assignments
    async fn suspended_user_roles(
        &self,
        ctx: &Context<'_>,
        limit: Option<i32>,
    ) -> Result<Vec<UserRole>, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        let mut user_roles = repo
            .list::<UserRole>(None)
            .await
            .map_err(|e| e.to_graphql_error())?;

        // Filter for suspended assignments
        user_roles = user_roles
            .into_iter()
            .filter(|ur| matches!(ur.status, RoleAssignmentStatus::Suspended))
            .collect();

        // Apply limit if provided
        if let Some(limit_val) = limit {
            user_roles.truncate(limit_val as usize);
        }

        Ok(user_roles)
    }

    /// Get role assignment statistics
    async fn user_role_statistics(
        &self,
        ctx: &Context<'_>,
    ) -> Result<UserRoleStatistics, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        let user_roles = repo
            .list::<UserRole>(None)
            .await
            .map_err(|e| e.to_graphql_error())?;

        let total_assignments = user_roles.len() as i32;
        let active_assignments = user_roles.iter().filter(|ur| matches!(ur.status, RoleAssignmentStatus::Active)).count() as i32;
        let suspended_assignments = user_roles.iter().filter(|ur| matches!(ur.status, RoleAssignmentStatus::Suspended)).count() as i32;
        let expired_assignments = user_roles.iter().filter(|ur| matches!(ur.status, RoleAssignmentStatus::Expired)).count() as i32;
        let revoked_assignments = user_roles.iter().filter(|ur| matches!(ur.status, RoleAssignmentStatus::Revoked)).count() as i32;
        let pending_assignments = user_roles.iter().filter(|ur| matches!(ur.status, RoleAssignmentStatus::Pending)).count() as i32;
        let primary_assignments = user_roles.iter().filter(|ur| ur.is_primary_role).count() as i32;
        let effective_assignments = user_roles.iter().filter(|ur| ur.is_effective()).count() as i32;

        Ok(UserRoleStatistics {
            total_assignments,
            active_assignments,
            suspended_assignments,
            expired_assignments,
            revoked_assignments,
            pending_assignments,
            primary_assignments,
            effective_assignments,
        })
    }

    /// Get role assignment history for a user
    async fn user_role_history(
        &self,
        ctx: &Context<'_>,
        user_id: String,
        limit: Option<i32>,
    ) -> Result<Vec<UserRole>, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        // Verify user exists
        let _user = repo
            .get::<User>(user_id.clone())
            .await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| {
                AppError::NotFound(format!("User {} not found", user_id))
                    .to_graphql_error()
            })?;

        let mut user_roles = repo
            .list::<UserRole>(None)
            .await
            .map_err(|e| e.to_graphql_error())?;

        // Filter by user ID
        user_roles = user_roles
            .into_iter()
            .filter(|ur| ur.user_id == user_id)
            .collect();

        // Sort by assignment date (most recent first)
        user_roles.sort_by(|a, b| b.assigned_at.cmp(&a.assigned_at));

        // Apply limit if provided
        if let Some(limit_val) = limit {
            user_roles.truncate(limit_val as usize);
        }

        Ok(user_roles)
    }

    /// Check if a user has a specific role (effective assignments only)
    async fn user_has_role(
        &self,
        ctx: &Context<'_>,
        user_id: String,
        role_id: String,
    ) -> Result<bool, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        let user_roles = repo
            .list::<UserRole>(None)
            .await
            .map_err(|e| e.to_graphql_error())?;

        let has_role = user_roles
            .iter()
            .any(|ur| ur.user_id == user_id && ur.role_id == role_id && ur.is_effective());

        Ok(has_role)
    }
}

/// User role assignment statistics
#[derive(Debug)]
pub struct UserRoleStatistics {
    pub total_assignments: i32,
    pub active_assignments: i32,
    pub suspended_assignments: i32,
    pub expired_assignments: i32,
    pub revoked_assignments: i32,
    pub pending_assignments: i32,
    pub primary_assignments: i32,
    pub effective_assignments: i32,
}

#[Object]
impl UserRoleStatistics {
    async fn total_assignments(&self) -> i32 {
        self.total_assignments
    }

    async fn active_assignments(&self) -> i32 {
        self.active_assignments
    }

    async fn suspended_assignments(&self) -> i32 {
        self.suspended_assignments
    }

    async fn expired_assignments(&self) -> i32 {
        self.expired_assignments
    }

    async fn revoked_assignments(&self) -> i32 {
        self.revoked_assignments
    }

    async fn pending_assignments(&self) -> i32 {
        self.pending_assignments
    }

    async fn primary_assignments(&self) -> i32 {
        self.primary_assignments
    }

    async fn effective_assignments(&self) -> i32 {
        self.effective_assignments
    }

    async fn inactive_assignments(&self) -> i32 {
        self.total_assignments - self.active_assignments
    }

    async fn active_percentage(&self) -> f64 {
        if self.total_assignments == 0 {
            0.0
        } else {
            (self.active_assignments as f64 / self.total_assignments as f64) * 100.0
        }
    }
}