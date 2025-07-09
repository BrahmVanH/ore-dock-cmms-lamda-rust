use async_graphql::*;
use chrono::{DateTime, Utc};
use tracing::{info, warn};
use uuid::Uuid;

use crate::{
    DbClient,
    models::{
        role::{Role, RoleType},
        permission::Permission,
        user_role::UserRole,
    },
    AppError,
    Repository,
};

#[derive(Debug, Default)]
pub struct RoleMutation;

#[Object]
impl RoleMutation {
    async fn create_role(
        &self,
        ctx: &Context<'_>,
        name: String,
        description: Option<String>,
        role_type: String,
        is_system_role: Option<bool>,
        permission_ids: Option<Vec<String>>,
        parent_role_id: Option<String>,
        priority: Option<i32>,
        active: Option<bool>,
        expires_at: Option<DateTime<Utc>>,
        max_users: Option<i32>,
        created_by: Option<String>,
    ) -> Result<Role, Error> {
        info!("Creating new role: {}", name);

        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());
        let id = Uuid::new_v4().to_string();

        let existing_roles = repo.list::<Role>(None).await.map_err(|e| e.to_graphql_error())?;
        if existing_roles.iter().any(|r| r.name.to_lowercase() == name.to_lowercase()) {
            return Err(AppError::ValidationError("Role name already exists".to_string()).to_graphql_error());
        }

        if let Some(ref parent_id) = parent_role_id {
            let _parent_role = repo.get::<Role>(parent_id.clone())
                .await
                .map_err(|e| e.to_graphql_error())?
                .ok_or_else(|| {
                    AppError::ValidationError(format!("Parent role {} not found", parent_id))
                        .to_graphql_error()
                })?;
        }

        let permission_ids = permission_ids.unwrap_or_default();
        for permission_id in &permission_ids {
            let _permission = repo.get::<Permission>(permission_id.clone())
                .await
                .map_err(|e| e.to_graphql_error())?
                .ok_or_else(|| {
                    AppError::ValidationError(format!("Permission {} not found", permission_id))
                        .to_graphql_error()
                })?;
        }

        let role = Role::new(
            id,
            name,
            description,
            role_type,
            is_system_role.unwrap_or(false),
            permission_ids,
            parent_role_id,
            priority.unwrap_or(0),
            active.unwrap_or(true),
            expires_at,
            max_users,
            created_by,
        ).map_err(|e| e.to_graphql_error())?;

        repo.create(role)
            .await
            .map_err(|e| e.to_graphql_error())
    }

    async fn update_role(
        &self,
        ctx: &Context<'_>,
        id: String,
        name: Option<String>,
        description: Option<String>,
        role_type: Option<String>,
        parent_role_id: Option<String>,
        priority: Option<i32>,
        active: Option<bool>,
        expires_at: Option<DateTime<Utc>>,
        max_users: Option<i32>,
    ) -> Result<Role, Error> {
        info!("Updating role: {}", id);

        let db_client = ctx.data::<DbClient>().map_err(|_| {
            AppError::InternalServerError("Database client not available".to_string())
        })?;

        let repo = Repository::new(db_client.clone());

        let mut role = repo
            .get::<Role>(id.clone())
            .await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| AppError::NotFound(format!("Role {} not found", id)))?;

        if role.is_system_role {
            return Err(AppError::ValidationError("Cannot modify system roles".to_string()).to_graphql_error());
        }

        if let Some(new_name) = name {
            let existing_roles = repo.list::<Role>(None).await.map_err(|e| e.to_graphql_error())?;
            if existing_roles.iter().any(|r| r.id != role.id && r.name.to_lowercase() == new_name.to_lowercase()) {
                return Err(AppError::ValidationError("Role name already exists".to_string()).to_graphql_error());
            }
            role.name = new_name;
        }

        if let Some(desc) = description {
            role.description = if desc.is_empty() { None } else { Some(desc) };
        }

        if let Some(r_type) = role_type {
            role.role_type = RoleType::from_string(&r_type).map_err(|e| e.to_graphql_error())?;
        }

        if let Some(parent_id) = parent_role_id {
            if !parent_id.is_empty() {
                if parent_id == role.id {
                    return Err(AppError::ValidationError("Role cannot be its own parent".to_string()).to_graphql_error());
                }
                let _parent_role = repo.get::<Role>(parent_id.clone())
                    .await
                    .map_err(|e| e.to_graphql_error())?
                    .ok_or_else(|| {
                        AppError::ValidationError(format!("Parent role {} not found", parent_id))
                            .to_graphql_error()
                    })?;
                role.parent_role_id = Some(parent_id);
            } else {
                role.parent_role_id = None;
            }
        }

        if let Some(prio) = priority {
            role.priority = prio;
        }

        if let Some(is_active) = active {
            role.active = is_active;
        }

        if let Some(expires) = expires_at {
            role.expires_at = Some(expires);
        }

        if let Some(max) = max_users {
            role.max_users = if max <= 0 { None } else { Some(max) };
        }

        role.updated_at = Utc::now();

        repo.update(role).await.map_err(|e| e.to_graphql_error())
    }

    async fn add_permission_to_role(
        &self,
        ctx: &Context<'_>,
        role_id: String,
        permission_id: String,
    ) -> Result<Role, Error> {
        info!("Adding permission {} to role {}", permission_id, role_id);

        let db_client = ctx.data::<DbClient>().map_err(|_| {
            AppError::InternalServerError("Database client not available".to_string())
        })?;

        let repo = Repository::new(db_client.clone());

        let mut role = repo
            .get::<Role>(role_id.clone())
            .await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| AppError::NotFound(format!("Role {} not found", role_id)))?;

        if role.is_system_role {
            return Err(AppError::ValidationError("Cannot modify permissions for system roles".to_string()).to_graphql_error());
        }

        let _permission = repo.get::<Permission>(permission_id.clone())
            .await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| {
                AppError::ValidationError(format!("Permission {} not found", permission_id))
                    .to_graphql_error()
            })?;

        role.add_permission(permission_id);

        repo.update(role).await.map_err(|e| e.to_graphql_error())
    }

    async fn remove_permission_from_role(
        &self,
        ctx: &Context<'_>,
        role_id: String,
        permission_id: String,
    ) -> Result<Role, Error> {
        info!("Removing permission {} from role {}", permission_id, role_id);

        let db_client = ctx.data::<DbClient>().map_err(|_| {
            AppError::InternalServerError("Database client not available".to_string())
        })?;

        let repo = Repository::new(db_client.clone());

        let mut role = repo
            .get::<Role>(role_id.clone())
            .await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| AppError::NotFound(format!("Role {} not found", role_id)))?;

        if role.is_system_role {
            return Err(AppError::ValidationError("Cannot modify permissions for system roles".to_string()).to_graphql_error());
        }

        role.remove_permission(&permission_id);

        repo.update(role).await.map_err(|e| e.to_graphql_error())
    }

    async fn set_role_permissions(
        &self,
        ctx: &Context<'_>,
        role_id: String,
        permission_ids: Vec<String>,
    ) -> Result<Role, Error> {
        info!("Setting permissions for role {}", role_id);

        let db_client = ctx.data::<DbClient>().map_err(|_| {
            AppError::InternalServerError("Database client not available".to_string())
        })?;

        let repo = Repository::new(db_client.clone());

        let mut role = repo
            .get::<Role>(role_id.clone())
            .await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| AppError::NotFound(format!("Role {} not found", role_id)))?;

        if role.is_system_role {
            return Err(AppError::ValidationError("Cannot modify permissions for system roles".to_string()).to_graphql_error());
        }

        for permission_id in &permission_ids {
            let _permission = repo.get::<Permission>(permission_id.clone())
                .await
                .map_err(|e| e.to_graphql_error())?
                .ok_or_else(|| {
                    AppError::ValidationError(format!("Permission {} not found", permission_id))
                        .to_graphql_error()
                })?;
        }

        role.permission_ids = permission_ids;
        role.updated_at = Utc::now();

        repo.update(role).await.map_err(|e| e.to_graphql_error())
    }

    async fn activate_role(
        &self,
        ctx: &Context<'_>,
        id: String,
    ) -> Result<Role, Error> {
        info!("Activating role: {}", id);

        let db_client = ctx.data::<DbClient>().map_err(|_| {
            AppError::InternalServerError("Database client not available".to_string())
        })?;

        let repo = Repository::new(db_client.clone());

        let mut role = repo
            .get::<Role>(id.clone())
            .await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| AppError::NotFound(format!("Role {} not found", id)))?;

        role.active = true;
        role.updated_at = Utc::now();

        repo.update(role).await.map_err(|e| e.to_graphql_error())
    }

    async fn deactivate_role(
        &self,
        ctx: &Context<'_>,
        id: String,
    ) -> Result<Role, Error> {
        info!("Deactivating role: {}", id);

        let db_client = ctx.data::<DbClient>().map_err(|_| {
            AppError::InternalServerError("Database client not available".to_string())
        })?;

        let repo = Repository::new(db_client.clone());

        let mut role = repo
            .get::<Role>(id.clone())
            .await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| AppError::NotFound(format!("Role {} not found", id)))?;

        if role.is_system_role {
            return Err(AppError::ValidationError("Cannot deactivate system roles".to_string()).to_graphql_error());
        }

        role.active = false;
        role.updated_at = Utc::now();

        repo.update(role).await.map_err(|e| e.to_graphql_error())
    }

    async fn extend_role_expiration(
        &self,
        ctx: &Context<'_>,
        id: String,
        new_expiration: DateTime<Utc>,
    ) -> Result<Role, Error> {
        info!("Extending role expiration: {}", id);

        let db_client = ctx.data::<DbClient>().map_err(|_| {
            AppError::InternalServerError("Database client not available".to_string())
        })?;

        let repo = Repository::new(db_client.clone());

        let mut role = repo
            .get::<Role>(id.clone())
            .await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| AppError::NotFound(format!("Role {} not found", id)))?;

        if new_expiration <= Utc::now() {
            return Err(AppError::ValidationError("New expiration must be in the future".to_string()).to_graphql_error());
        }

        role.expires_at = Some(new_expiration);
        role.updated_at = Utc::now();

        repo.update(role).await.map_err(|e| e.to_graphql_error())
    }

    async fn clone_role(
        &self,
        ctx: &Context<'_>,
        source_role_id: String,
        new_name: String,
        new_description: Option<String>,
        created_by: Option<String>,
    ) -> Result<Role, Error> {
        info!("Cloning role {} to {}", source_role_id, new_name);

        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        let source_role = repo
            .get::<Role>(source_role_id.clone())
            .await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| AppError::NotFound(format!("Source role {} not found", source_role_id)))?;

        let existing_roles = repo.list::<Role>(None).await.map_err(|e| e.to_graphql_error())?;
        if existing_roles.iter().any(|r| r.name.to_lowercase() == new_name.to_lowercase()) {
            return Err(AppError::ValidationError("Role name already exists".to_string()).to_graphql_error());
        }

        let new_id = Uuid::new_v4().to_string();

        let cloned_role = Role::new(
            new_id,
            new_name,
            new_description.or(source_role.description),
            source_role.role_type.to_string(),
            false,
            source_role.permission_ids.clone(),
            source_role.parent_role_id.clone(),
            source_role.priority,
            true,
            source_role.expires_at,
            source_role.max_users,
            created_by,
        ).map_err(|e| e.to_graphql_error())?;

        repo.create(cloned_role)
            .await
            .map_err(|e| e.to_graphql_error())
    }

    async fn bulk_update_role_permissions(
        &self,
        ctx: &Context<'_>,
        role_ids: Vec<String>,
        permission_ids: Vec<String>,
        operation: String,
    ) -> Result<Vec<Role>, Error> {
        info!("Bulk updating permissions for {} roles", role_ids.len());

        let db_client = ctx.data::<DbClient>().map_err(|_| {
            AppError::InternalServerError("Database client not available".to_string())
        })?;

        let repo = Repository::new(db_client.clone());

        if !["add", "remove", "set"].contains(&operation.as_str()) {
            return Err(AppError::ValidationError("Operation must be 'add', 'remove', or 'set'".to_string()).to_graphql_error());
        }

        for permission_id in &permission_ids {
            let _permission = repo.get::<Permission>(permission_id.clone())
                .await
                .map_err(|e| e.to_graphql_error())?
                .ok_or_else(|| {
                    AppError::ValidationError(format!("Permission {} not found", permission_id))
                        .to_graphql_error()
                })?;
        }

        let mut results = Vec::new();

        for role_id in role_ids {
            let role_result = repo.get::<Role>(role_id.clone()).await;
            
            if let Ok(Some(mut role)) = role_result {
                if !role.is_system_role {
                    match operation.as_str() {
                        "add" => {
                            for permission_id in &permission_ids {
                                role.add_permission(permission_id.clone());
                            }
                        },
                        "remove" => {
                            for permission_id in &permission_ids {
                                role.remove_permission(permission_id);
                            }
                        },
                        "set" => {
                            role.permission_ids = permission_ids.clone();
                            role.updated_at = Utc::now();
                        },
                        _ => continue,
                    }

                    if let Ok(updated_role) = repo.update(role).await {
                        results.push(updated_role);
                    }
                }
            }
        }

        Ok(results)
    }

    async fn delete_role(
        &self,
        ctx: &Context<'_>,
        id: String,
        force: Option<bool>,
    ) -> Result<bool, Error> {
        info!("Deleting role: {}", id);

        let db_client = ctx.data::<DbClient>().map_err(|_| {
            AppError::InternalServerError("Database client not available".to_string())
        })?;

        let repo = Repository::new(db_client.clone());

        let role = repo
            .get::<Role>(id.clone())
            .await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| AppError::NotFound(format!("Role {} not found", id)))?;

        if role.is_system_role {
            return Err(AppError::ValidationError("Cannot delete system roles".to_string()).to_graphql_error());
        }

        let user_roles = repo.list::<UserRole>(None).await.map_err(|e| e.to_graphql_error())?;
        let has_assignments = user_roles.iter().any(|ur| ur.role_id == id);

        if has_assignments && !force.unwrap_or(false) {
            return Err(AppError::ValidationError("Cannot delete role with active assignments. Use force=true to override.".to_string()).to_graphql_error());
        }

        if has_assignments {
            for user_role in user_roles.iter().filter(|ur| ur.role_id == id) {
                let _ = repo.delete::<UserRole>(user_role.id.clone()).await;
            }
        }

        let child_roles = repo.list::<Role>(None).await.map_err(|e| e.to_graphql_error())?;
        for mut child_role in child_roles.into_iter().filter(|r| r.parent_role_id.as_ref().map_or(false, |p| *p == id)) {
            child_role.parent_role_id = None;
            child_role.updated_at = Utc::now();
            let _ = repo.update(child_role).await;
        }

        repo.delete::<Role>(id).await.map_err(|e| e.to_graphql_error())
    }
}