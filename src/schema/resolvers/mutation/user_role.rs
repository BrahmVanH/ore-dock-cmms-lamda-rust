use crate::{
    DbClient,
    models::{ prelude::*, user_role::{ UserRole, RoleAssignmentStatus }, user::User, role::Role },
    AppError,
    Repository,
};

#[derive(Debug, Default)]
pub struct UserRoleMutation;

#[Object]
impl UserRoleMutation {
    /// Assign a role to a user
    async fn assign_user_role(
        &self,
        ctx: &Context<'_>,
        user_id: String,
        role_id: String,
        assignment_source: String,
        is_primary_role: Option<bool>,
        assigned_by_user_id: String,
        effective_from: Option<DateTime<Utc>>,
        expires_at: Option<DateTime<Utc>>,
        conditions: Option<String>,
        elevation_request_id: Option<String>,
        metadata: Option<String>
    ) -> Result<UserRole, Error> {
        info!("Assigning role {} to user {}", role_id, user_id);

        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());
        let id = format!("user_role-{}", Uuid::new_v4());

        // Verify user exists
        let _user = repo
            .get::<User>(user_id.clone()).await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| {
                AppError::ValidationError(format!("User {} not found", user_id)).to_graphql_error()
            })?;

        // Verify role exists
        let _role = repo
            .get::<Role>(role_id.clone()).await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| {
                AppError::ValidationError(format!("Role {} not found", role_id)).to_graphql_error()
            })?;

        // Verify assigning user exists
        let _assigning_user = repo
            .get::<User>(assigned_by_user_id.clone()).await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| {
                AppError::ValidationError(
                    format!("Assigning user {} not found", assigned_by_user_id)
                ).to_graphql_error()
            })?;

        // Check if assignment already exists and is active
        let existing_assignments = repo
            .list::<UserRole>(None).await
            .map_err(|e| e.to_graphql_error())?;

        let has_active_assignment = existing_assignments
            .iter()
            .any(|ur| { ur.user_id == user_id && ur.role_id == role_id && ur.is_effective() });

        if has_active_assignment {
            return Err(
                AppError::ValidationError(
                    "User already has an active assignment for this role".to_string()
                ).to_graphql_error()
            );
        }

        let is_primary = is_primary_role.unwrap_or(false);

        // If this is a primary role, ensure user doesn't already have one
        if is_primary {
            let has_primary_role = existing_assignments
                .iter()
                .any(|ur| { ur.user_id == user_id && ur.is_primary_role && ur.is_effective() });

            if has_primary_role {
                return Err(
                    AppError::ValidationError(
                        "User already has a primary role assigned".to_string()
                    ).to_graphql_error()
                );
            }
        }

        let effective_from = effective_from.unwrap_or_else(|| Utc::now());

        let user_role = UserRole::new(
            id,
            user_id,
            role_id,
            assignment_source,
            is_primary,
            Some(assigned_by_user_id),
            effective_from,
            expires_at,
            conditions,
            elevation_request_id,
            metadata
        ).map_err(|e| e.to_graphql_error())?;

        repo.create(user_role).await.map_err(|e| e.to_graphql_error())
    }

    /// Update an existing user role assignment
    async fn update_user_role(
        &self,
        ctx: &Context<'_>,
        id: String,
        is_primary_role: Option<bool>,
        effective_from: Option<DateTime<Utc>>,
        expires_at: Option<DateTime<Utc>>,
        conditions: Option<String>,
        metadata: Option<String>
    ) -> Result<UserRole, Error> {
        info!("Updating user role assignment: {}", id);

        let db_client = ctx
            .data::<DbClient>()
            .map_err(|_| {
                AppError::InternalServerError("Database client not available".to_string())
            })?;

        let repo = Repository::new(db_client.clone());

        let mut user_role = repo
            .get::<UserRole>(id.clone()).await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| AppError::NotFound(format!("User role assignment {} not found", id)))?;

        // Don't allow updates to revoked assignments
        if matches!(user_role.status, RoleAssignmentStatus::Revoked) {
            return Err(
                AppError::ValidationError(
                    "Cannot update revoked role assignment".to_string()
                ).to_graphql_error()
            );
        }

        // Handle primary role changes
        if let Some(is_primary) = is_primary_role {
            if is_primary && !user_role.is_primary_role {
                // Check if user already has a primary role
                let existing_assignments = repo
                    .list::<UserRole>(None).await
                    .map_err(|e| e.to_graphql_error())?;

                let has_primary_role = existing_assignments
                    .iter()
                    .any(|ur| {
                        ur.user_id == user_role.user_id &&
                            ur.id != user_role.id &&
                            ur.is_primary_role &&
                            ur.is_effective()
                    });

                if has_primary_role {
                    return Err(
                        AppError::ValidationError(
                            "User already has a primary role assigned".to_string()
                        ).to_graphql_error()
                    );
                }
            }
            user_role.is_primary_role = is_primary;
        }

        if let Some(effective) = effective_from {
            user_role.effective_from = effective;
        }
        if let Some(expires) = expires_at {
            user_role.expires_at = Some(expires);
        }
        if let Some(cond) = conditions {
            user_role.conditions = if cond.is_empty() { None } else { Some(cond) };
        }
        if let Some(meta) = metadata {
            user_role.metadata = if meta.is_empty() { None } else { Some(meta) };
        }

        user_role.updated_at = Utc::now();

        repo.update(user_role).await.map_err(|e| e.to_graphql_error())
    }

    /// Revoke a user role assignment
    async fn revoke_user_role(
        &self,
        ctx: &Context<'_>,
        id: String,
        revoked_by_user_id: String,
        revocation_reason: Option<String>
    ) -> Result<UserRole, Error> {
        info!("Revoking user role assignment: {}", id);

        let db_client = ctx
            .data::<DbClient>()
            .map_err(|_| {
                AppError::InternalServerError("Database client not available".to_string())
            })?;

        let repo = Repository::new(db_client.clone());

        let mut user_role = repo
            .get::<UserRole>(id.clone()).await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| AppError::NotFound(format!("User role assignment {} not found", id)))?;

        // Verify revoking user exists
        let _revoking_user = repo
            .get::<User>(revoked_by_user_id.clone()).await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| {
                AppError::ValidationError(
                    format!("Revoking user {} not found", revoked_by_user_id)
                ).to_graphql_error()
            })?;

        if matches!(user_role.status, RoleAssignmentStatus::Revoked) {
            return Err(
                AppError::ValidationError(
                    "Role assignment is already revoked".to_string()
                ).to_graphql_error()
            );
        }

        user_role.status = RoleAssignmentStatus::Revoked;
        user_role.revoked_at = Some(Utc::now());
        user_role.revoked_by_user_id = Some(revoked_by_user_id);
        user_role.revocation_reason = revocation_reason;
        user_role.updated_at = Utc::now();

        repo.update(user_role).await.map_err(|e| e.to_graphql_error())
    }

    /// Suspend a user role assignment
    async fn suspend_user_role(
        &self,
        ctx: &Context<'_>,
        id: String,
        suspended_by_user_id: String,
        suspension_reason: Option<String>
    ) -> Result<UserRole, Error> {
        info!("Suspending user role assignment: {}", id);

        let db_client = ctx
            .data::<DbClient>()
            .map_err(|_| {
                AppError::InternalServerError("Database client not available".to_string())
            })?;

        let repo = Repository::new(db_client.clone());

        let mut user_role = repo
            .get::<UserRole>(id.clone()).await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| AppError::NotFound(format!("User role assignment {} not found", id)))?;

        // Verify suspending user exists
        let _suspending_user = repo
            .get::<User>(suspended_by_user_id.clone()).await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| {
                AppError::ValidationError(
                    format!("Suspending user {} not found", suspended_by_user_id)
                ).to_graphql_error()
            })?;

        if !matches!(user_role.status, RoleAssignmentStatus::Active) {
            return Err(
                AppError::ValidationError(
                    "Only active role assignments can be suspended".to_string()
                ).to_graphql_error()
            );
        }

        user_role.status = RoleAssignmentStatus::Suspended;
        user_role.revoked_by_user_id = Some(suspended_by_user_id);
        user_role.revocation_reason = suspension_reason;
        user_role.updated_at = Utc::now();

        repo.update(user_role).await.map_err(|e| e.to_graphql_error())
    }

    /// Reactivate a suspended user role assignment
    async fn reactivate_user_role(&self, ctx: &Context<'_>, id: String) -> Result<UserRole, Error> {
        info!("Reactivating user role assignment: {}", id);

        let db_client = ctx
            .data::<DbClient>()
            .map_err(|_| {
                AppError::InternalServerError("Database client not available".to_string())
            })?;

        let repo = Repository::new(db_client.clone());

        let mut user_role = repo
            .get::<UserRole>(id.clone()).await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| AppError::NotFound(format!("User role assignment {} not found", id)))?;

        if !matches!(user_role.status, RoleAssignmentStatus::Suspended) {
            return Err(
                AppError::ValidationError(
                    "Only suspended role assignments can be reactivated".to_string()
                ).to_graphql_error()
            );
        }

        user_role.status = RoleAssignmentStatus::Active;
        user_role.revoked_by_user_id = None;
        user_role.revocation_reason = None;
        user_role.updated_at = Utc::now();

        repo.update(user_role).await.map_err(|e| e.to_graphql_error())
    }

    /// Extend the expiration date of a user role assignment
    async fn extend_user_role_expiration(
        &self,
        ctx: &Context<'_>,
        id: String,
        new_expiration: DateTime<Utc>
    ) -> Result<UserRole, Error> {
        info!("Extending user role assignment expiration: {}", id);

        let db_client = ctx
            .data::<DbClient>()
            .map_err(|_| {
                AppError::InternalServerError("Database client not available".to_string())
            })?;

        let repo = Repository::new(db_client.clone());

        let mut user_role = repo
            .get::<UserRole>(id.clone()).await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| AppError::NotFound(format!("User role assignment {} not found", id)))?;

        if new_expiration <= Utc::now() {
            return Err(
                AppError::ValidationError(
                    "New expiration must be in the future".to_string()
                ).to_graphql_error()
            );
        }

        user_role.expires_at = Some(new_expiration);
        user_role.updated_at = Utc::now();

        repo.update(user_role).await.map_err(|e| e.to_graphql_error())
    }

    /// Mark a user role as used (updates last_used_at)
    async fn mark_user_role_as_used(
        &self,
        ctx: &Context<'_>,
        id: String
    ) -> Result<UserRole, Error> {
        info!("Marking user role assignment as used: {}", id);

        let db_client = ctx
            .data::<DbClient>()
            .map_err(|_| {
                AppError::InternalServerError("Database client not available".to_string())
            })?;

        let repo = Repository::new(db_client.clone());

        let mut user_role = repo
            .get::<UserRole>(id.clone()).await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| AppError::NotFound(format!("User role assignment {} not found", id)))?;

        user_role.mark_as_used();

        repo.update(user_role).await.map_err(|e| e.to_graphql_error())
    }

    /// Set a role assignment as primary (and unset others for the user)
    async fn set_primary_user_role(
        &self,
        ctx: &Context<'_>,
        id: String
    ) -> Result<UserRole, Error> {
        info!("Setting user role assignment as primary: {}", id);

        let db_client = ctx
            .data::<DbClient>()
            .map_err(|_| {
                AppError::InternalServerError("Database client not available".to_string())
            })?;

        let repo = Repository::new(db_client.clone());

        let mut target_role = repo
            .get::<UserRole>(id.clone()).await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| AppError::NotFound(format!("User role assignment {} not found", id)))?;

        if !target_role.is_effective() {
            return Err(
                AppError::ValidationError(
                    "Only effective role assignments can be set as primary".to_string()
                ).to_graphql_error()
            );
        }

        // Get all role assignments for this user
        let all_user_roles = repo.list::<UserRole>(None).await.map_err(|e| e.to_graphql_error())?;

        let user_roles: Vec<UserRole> = all_user_roles
            .into_iter()
            .filter(|ur| ur.user_id == target_role.user_id)
            .collect();

        // Update all existing primary roles for this user to non-primary
        for mut user_role in user_roles {
            if user_role.id != target_role.id && user_role.is_primary_role {
                user_role.is_primary_role = false;
                user_role.updated_at = Utc::now();
                repo.update(user_role).await.map_err(|e| e.to_graphql_error())?;
            }
        }

        // Set target role as primary
        target_role.is_primary_role = true;
        target_role.updated_at = Utc::now();

        repo.update(target_role).await.map_err(|e| e.to_graphql_error())
    }

    /// Bulk assign roles to multiple users
    async fn bulk_assign_user_roles(
        &self,
        ctx: &Context<'_>,
        user_ids: Vec<String>,
        role_id: String,
        assignment_source: String,
        assigned_by_user_id: String,
        effective_from: Option<DateTime<Utc>>,
        expires_at: Option<DateTime<Utc>>
    ) -> Result<Vec<UserRole>, Error> {
        info!("Bulk assigning role {} to {} users", role_id, user_ids.len());

        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        // Verify role exists
        let _role = repo
            .get::<Role>(role_id.clone()).await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| {
                AppError::ValidationError(format!("Role {} not found", role_id)).to_graphql_error()
            })?;

        // Verify assigning user exists
        let _assigning_user = repo
            .get::<User>(assigned_by_user_id.clone()).await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| {
                AppError::ValidationError(
                    format!("Assigning user {} not found", assigned_by_user_id)
                ).to_graphql_error()
            })?;

        let mut results = Vec::new();
        let effective_from = effective_from.unwrap_or_else(|| Utc::now());

        for user_id in user_ids {
            // Verify user exists
            let user_exists = repo
                .get::<User>(user_id.clone()).await
                .map_err(|e| e.to_graphql_error())?
                .is_some();

            if !user_exists {
                continue; // Skip non-existent users
            }

            // Check if assignment already exists
            let existing_assignments = repo
                .list::<UserRole>(None).await
                .map_err(|e| e.to_graphql_error())?;

            let has_active_assignment = existing_assignments
                .iter()
                .any(|ur| { ur.user_id == user_id && ur.role_id == role_id && ur.is_effective() });

            if has_active_assignment {
                continue; // Skip users who already have this role
            }

            let id = format!("user_role-{}", Uuid::new_v4());

            let user_role = UserRole::new(
                id,
                user_id,
                role_id.clone(),
                assignment_source.clone(),
                false, // Not primary role in bulk assignment
                Some(assigned_by_user_id.clone()),
                effective_from,
                expires_at,
                None, // No conditions
                None, // No elevation request
                None // No metadata
            ).map_err(|e| e.to_graphql_error())?;

            let created_role = repo.create(user_role).await.map_err(|e| e.to_graphql_error())?;

            results.push(created_role);
        }

        Ok(results)
    }

    /// Bulk revoke roles from multiple users
    async fn bulk_revoke_user_roles(
        &self,
        ctx: &Context<'_>,
        user_role_ids: Vec<String>,
        revoked_by_user_id: String,
        revocation_reason: Option<String>
    ) -> Result<Vec<UserRole>, Error> {
        info!("Bulk revoking {} user role assignments", user_role_ids.len());

        let db_client = ctx
            .data::<DbClient>()
            .map_err(|_| {
                AppError::InternalServerError("Database client not available".to_string())
            })?;

        let repo = Repository::new(db_client.clone());

        // Verify revoking user exists
        let _revoking_user = repo
            .get::<User>(revoked_by_user_id.clone()).await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| {
                AppError::ValidationError(
                    format!("Revoking user {} not found", revoked_by_user_id)
                ).to_graphql_error()
            })?;

        let mut results = Vec::new();

        for user_role_id in user_role_ids {
            let user_role_result = repo
                .get::<UserRole>(user_role_id.clone()).await
                .map_err(|e| e.to_graphql_error());

            if let Ok(Some(mut user_role)) = user_role_result {
                if !matches!(user_role.status, RoleAssignmentStatus::Revoked) {
                    user_role.status = RoleAssignmentStatus::Revoked;
                    user_role.revoked_at = Some(Utc::now());
                    user_role.revoked_by_user_id = Some(revoked_by_user_id.clone());
                    user_role.revocation_reason = revocation_reason.clone();
                    user_role.updated_at = Utc::now();

                    if let Ok(updated_role) = repo.update(user_role).await {
                        results.push(updated_role);
                    }
                }
            }
        }

        Ok(results)
    }

    /// Delete a user role assignment (hard delete)
    async fn delete_user_role(&self, ctx: &Context<'_>, id: String) -> Result<bool, Error> {
        info!("Deleting user role assignment: {}", id);

        let db_client = ctx
            .data::<DbClient>()
            .map_err(|_| {
                AppError::InternalServerError("Database client not available".to_string())
            })?;

        let repo = Repository::new(db_client.clone());

        // Verify user role exists
        let user_role = repo
            .get::<UserRole>(id.clone()).await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| AppError::NotFound(format!("User role assignment {} not found", id)))?;

        // Business rule: Only allow deletion of revoked or expired assignments
        if user_role.is_effective() {
            return Err(
                AppError::ValidationError(
                    "Cannot delete effective role assignments. Revoke first.".to_string()
                ).to_graphql_error()
            );
        }

        repo.delete::<UserRole>(id).await.map_err(|e| e.to_graphql_error())
    }
}
