use crate::{
    DbClient,
    models::{
        prelude::*,
        permission::{ Permission, PermissionScope },
        permission_log::PermissionAction,
        role::Role,
    },
    AppError,
    Repository,
};

#[derive(Debug, Default)]
pub struct PermissionMutation;

#[Object]
impl PermissionMutation {
    async fn create_permission(
        &self,
        ctx: &Context<'_>,
        role_id: String,
        resource_type: String,
        actions: Vec<String>,
        scope: String,
        conditions: Option<String>,
        resource_filters: Option<String>,
        active: Option<bool>,
        expires_at: Option<DateTime<Utc>>,
        created_by: String
    ) -> Result<Permission, Error> {
        info!("Creating new permission for role: {}", role_id);

        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());
        let id = format!("permission-{}", Uuid::new_v4());

        let _role = repo
            .get::<Role>(role_id.clone()).await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| {
                AppError::ValidationError(format!("Role {} not found", role_id)).to_graphql_error()
            })?;

        let conditions_json = if let Some(ref cond_str) = conditions {
            Some(
                serde_json
                    ::from_str::<Json>(cond_str)
                    .map_err(|_| {
                        AppError::ValidationError(
                            "Invalid conditions JSON".to_string()
                        ).to_graphql_error()
                    })?
            )
        } else {
            None
        };

        let resource_filters_json = if let Some(ref filter_str) = resource_filters {
            Some(
                serde_json
                    ::from_str::<Json>(filter_str)
                    .map_err(|_| {
                        AppError::ValidationError(
                            "Invalid resource filters JSON".to_string()
                        ).to_graphql_error()
                    })?
            )
        } else {
            None
        };

        let permission = Permission::new(
            id,
            role_id,
            resource_type,
            actions,
            scope,
            conditions_json,
            resource_filters_json,
            active.unwrap_or(true),
            expires_at,
            created_by
        ).map_err(|e| e.to_graphql_error())?;

        repo.create(permission).await.map_err(|e| e.to_graphql_error())
    }

    async fn update_permission(
        &self,
        ctx: &Context<'_>,
        id: String,
        actions: Option<Vec<String>>,
        scope: Option<String>,
        conditions: Option<String>,
        resource_filters: Option<String>,
        active: Option<bool>,
        expires_at: Option<DateTime<Utc>>
    ) -> Result<Permission, Error> {
        info!("Updating permission: {}", id);

        let db_client = ctx
            .data::<DbClient>()
            .map_err(|_| {
                AppError::InternalServerError("Database client not available".to_string())
            })?;

        let repo = Repository::new(db_client.clone());

        let mut permission = repo
            .get::<Permission>(id.clone()).await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| AppError::NotFound(format!("Permission {} not found", id)))?;

        if let Some(action_strings) = actions {
            let action_enums: Result<Vec<PermissionAction>, AppError> = action_strings
                .iter()
                .map(|a| PermissionAction::from_string(a))
                .collect();
            permission.actions = action_enums.map_err(|e| e.to_graphql_error())?;
        }

        if let Some(scope_str) = scope {
            permission.scope = PermissionScope::from_string(&scope_str).map_err(|e|
                e.to_graphql_error()
            )?;
        }

        if let Some(cond_str) = conditions {
            permission.conditions = if cond_str.is_empty() {
                None
            } else {
                Some(
                    serde_json
                        ::from_str::<Json>(&cond_str)
                        .map_err(|_| {
                            AppError::ValidationError(
                                "Invalid conditions JSON".to_string()
                            ).to_graphql_error()
                        })?
                )
            };
        }

        if let Some(filter_str) = resource_filters {
            permission.resource_filters = if filter_str.is_empty() {
                None
            } else {
                Some(
                    serde_json
                        ::from_str::<Json>(&filter_str)
                        .map_err(|_| {
                            AppError::ValidationError(
                                "Invalid resource filters JSON".to_string()
                            ).to_graphql_error()
                        })?
                )
            };
        }

        if let Some(is_active) = active {
            permission.active = is_active;
        }

        if let Some(expires) = expires_at {
            permission.expires_at = Some(expires);
        }

        permission.updated_at = Utc::now();

        repo.update(permission).await.map_err(|e| e.to_graphql_error())
    }

    async fn add_action_to_permission(
        &self,
        ctx: &Context<'_>,
        permission_id: String,
        action: String
    ) -> Result<Permission, Error> {
        info!("Adding action {} to permission {}", action, permission_id);

        let db_client = ctx
            .data::<DbClient>()
            .map_err(|_| {
                AppError::InternalServerError("Database client not available".to_string())
            })?;

        let repo = Repository::new(db_client.clone());

        let mut permission = repo
            .get::<Permission>(permission_id.clone()).await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| AppError::NotFound(format!("Permission {} not found", permission_id)))?;

        let action_enum = PermissionAction::from_string(&action).map_err(|e| e.to_graphql_error())?;

        if !permission.actions.contains(&action_enum) {
            permission.actions.push(action_enum);
            permission.updated_at = Utc::now();
        }

        repo.update(permission).await.map_err(|e| e.to_graphql_error())
    }

    async fn remove_action_from_permission(
        &self,
        ctx: &Context<'_>,
        permission_id: String,
        action: String
    ) -> Result<Permission, Error> {
        info!("Removing action {} from permission {}", action, permission_id);

        let db_client = ctx
            .data::<DbClient>()
            .map_err(|_| {
                AppError::InternalServerError("Database client not available".to_string())
            })?;

        let repo = Repository::new(db_client.clone());

        let mut permission = repo
            .get::<Permission>(permission_id.clone()).await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| AppError::NotFound(format!("Permission {} not found", permission_id)))?;

        let action_enum = PermissionAction::from_string(&action).map_err(|e| e.to_graphql_error())?;

        permission.actions.retain(|a| *a != action_enum);

        if permission.actions.is_empty() {
            return Err(
                AppError::ValidationError(
                    "Permission must have at least one action".to_string()
                ).to_graphql_error()
            );
        }

        permission.updated_at = Utc::now();

        repo.update(permission).await.map_err(|e| e.to_graphql_error())
    }

    async fn activate_permission(
        &self,
        ctx: &Context<'_>,
        id: String
    ) -> Result<Permission, Error> {
        info!("Activating permission: {}", id);

        let db_client = ctx
            .data::<DbClient>()
            .map_err(|_| {
                AppError::InternalServerError("Database client not available".to_string())
            })?;

        let repo = Repository::new(db_client.clone());

        let mut permission = repo
            .get::<Permission>(id.clone()).await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| AppError::NotFound(format!("Permission {} not found", id)))?;

        permission.active = true;
        permission.updated_at = Utc::now();

        repo.update(permission).await.map_err(|e| e.to_graphql_error())
    }

    async fn deactivate_permission(
        &self,
        ctx: &Context<'_>,
        id: String
    ) -> Result<Permission, Error> {
        info!("Deactivating permission: {}", id);

        let db_client = ctx
            .data::<DbClient>()
            .map_err(|_| {
                AppError::InternalServerError("Database client not available".to_string())
            })?;

        let repo = Repository::new(db_client.clone());

        let mut permission = repo
            .get::<Permission>(id.clone()).await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| AppError::NotFound(format!("Permission {} not found", id)))?;

        permission.active = false;
        permission.updated_at = Utc::now();

        repo.update(permission).await.map_err(|e| e.to_graphql_error())
    }

    async fn extend_permission_expiration(
        &self,
        ctx: &Context<'_>,
        id: String,
        new_expiration: DateTime<Utc>
    ) -> Result<Permission, Error> {
        info!("Extending permission expiration: {}", id);

        let db_client = ctx
            .data::<DbClient>()
            .map_err(|_| {
                AppError::InternalServerError("Database client not available".to_string())
            })?;

        let repo = Repository::new(db_client.clone());

        let mut permission = repo
            .get::<Permission>(id.clone()).await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| AppError::NotFound(format!("Permission {} not found", id)))?;

        if new_expiration <= Utc::now() {
            return Err(
                AppError::ValidationError(
                    "New expiration must be in the future".to_string()
                ).to_graphql_error()
            );
        }

        permission.expires_at = Some(new_expiration);
        permission.updated_at = Utc::now();

        repo.update(permission).await.map_err(|e| e.to_graphql_error())
    }

    async fn clone_permission(
        &self,
        ctx: &Context<'_>,
        source_permission_id: String,
        new_role_id: String,
        created_by: String
    ) -> Result<Permission, Error> {
        info!("Cloning permission {} to role {}", source_permission_id, new_role_id);

        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        let source_permission = repo
            .get::<Permission>(source_permission_id.clone()).await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(||
                AppError::NotFound(format!("Source permission {} not found", source_permission_id))
            )?;

        let _role = repo
            .get::<Role>(new_role_id.clone()).await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| {
                AppError::ValidationError(
                    format!("Target role {} not found", new_role_id)
                ).to_graphql_error()
            })?;

        let new_id = format!("permission-{}", Uuid::new_v4());

        let cloned_permission = Permission::new(
            new_id,
            new_role_id,
            source_permission.resource_type.to_string(),
            source_permission.actions
                .iter()
                .map(|a| a.to_string())
                .collect(),
            source_permission.scope.to_string(),
            source_permission.conditions.clone(),
            source_permission.resource_filters.clone(),
            source_permission.active,
            source_permission.expires_at,
            created_by
        ).map_err(|e| e.to_graphql_error())?;

        repo.create(cloned_permission).await.map_err(|e| e.to_graphql_error())
    }

    async fn bulk_update_permissions(
        &self,
        ctx: &Context<'_>,
        permission_ids: Vec<String>,
        active: Option<bool>,
        expires_at: Option<DateTime<Utc>>
    ) -> Result<Vec<Permission>, Error> {
        info!("Bulk updating {} permissions", permission_ids.len());

        let db_client = ctx
            .data::<DbClient>()
            .map_err(|_| {
                AppError::InternalServerError("Database client not available".to_string())
            })?;

        let repo = Repository::new(db_client.clone());

        let mut results = Vec::new();

        for permission_id in permission_ids {
            let permission_result = repo.get::<Permission>(permission_id.clone()).await;

            if let Ok(Some(mut permission)) = permission_result {
                let mut updated = false;

                if let Some(is_active) = active {
                    permission.active = is_active;
                    updated = true;
                }

                if let Some(expires) = expires_at {
                    permission.expires_at = Some(expires);
                    updated = true;
                }

                if updated {
                    permission.updated_at = Utc::now();
                    if let Ok(updated_permission) = repo.update(permission).await {
                        results.push(updated_permission);
                    }
                }
            }
        }

        Ok(results)
    }

    async fn copy_permissions_to_role(
        &self,
        ctx: &Context<'_>,
        source_role_id: String,
        target_role_id: String,
        created_by: String,
        exclude_expired: Option<bool>
    ) -> Result<Vec<Permission>, Error> {
        info!("Copying permissions from role {} to role {}", source_role_id, target_role_id);

        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        let _source_role = repo
            .get::<Role>(source_role_id.clone()).await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| {
                AppError::ValidationError(
                    format!("Source role {} not found", source_role_id)
                ).to_graphql_error()
            })?;

        let _target_role = repo
            .get::<Role>(target_role_id.clone()).await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| {
                AppError::ValidationError(
                    format!("Target role {} not found", target_role_id)
                ).to_graphql_error()
            })?;

        let all_permissions = repo
            .list::<Permission>(None).await
            .map_err(|e| e.to_graphql_error())?;

        let mut source_permissions: Vec<Permission> = all_permissions
            .into_iter()
            .filter(|p| p.role_id == source_role_id)
            .collect();

        if let Some(true) = exclude_expired {
            source_permissions = source_permissions
                .into_iter()
                .filter(|p| !p.is_expired())
                .collect();
        }

        let mut results = Vec::new();

        for source_permission in source_permissions {
            let new_id = format!("permission-{}", Uuid::new_v4());

            let cloned_permission = Permission::new(
                new_id,
                target_role_id.clone(),
                source_permission.resource_type.to_string(),
                source_permission.actions
                    .iter()
                    .map(|a| a.to_string())
                    .collect(),
                source_permission.scope.to_string(),
                source_permission.conditions,
                source_permission.resource_filters,
                source_permission.active,
                source_permission.expires_at,
                created_by.clone()
            ).map_err(|e| e.to_graphql_error())?;

            if let Ok(created_permission) = repo.create(cloned_permission).await {
                results.push(created_permission);
            }
        }

        Ok(results)
    }

    async fn delete_permission(&self, ctx: &Context<'_>, id: String) -> Result<bool, Error> {
        info!("Deleting permission: {}", id);

        let db_client = ctx
            .data::<DbClient>()
            .map_err(|_| {
                AppError::InternalServerError("Database client not available".to_string())
            })?;

        let repo = Repository::new(db_client.clone());

        let _permission = repo
            .get::<Permission>(id.clone()).await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| AppError::NotFound(format!("Permission {} not found", id)))?;

        repo.delete::<Permission>(id).await.map_err(|e| e.to_graphql_error())
    }

    async fn delete_permissions_for_role(
        &self,
        ctx: &Context<'_>,
        role_id: String
    ) -> Result<i32, Error> {
        info!("Deleting all permissions for role: {}", role_id);

        let db_client = ctx
            .data::<DbClient>()
            .map_err(|_| {
                AppError::InternalServerError("Database client not available".to_string())
            })?;

        let repo = Repository::new(db_client.clone());

        let all_permissions = repo
            .list::<Permission>(None).await
            .map_err(|e| e.to_graphql_error())?;

        let role_permissions: Vec<Permission> = all_permissions
            .into_iter()
            .filter(|p| p.role_id == role_id)
            .collect();

        let mut deleted_count = 0;

        for permission in role_permissions {
            if repo.delete::<Permission>(permission.id).await.is_ok() {
                deleted_count += 1;
            }
        }

        Ok(deleted_count)
    }
}
