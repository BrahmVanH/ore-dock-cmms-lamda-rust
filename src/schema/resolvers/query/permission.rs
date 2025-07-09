use async_graphql::*;
use chrono::{ DateTime, Utc };
use tracing::warn;

use crate::{
    error::AppError,
    models::{
        permission::{ Permission, PermissionScope },
        permission_log::{ PermissionAction, ResourceType },
        role::Role,
    },
    DbClient,
    Repository,
};

#[derive(Debug, Default)]
pub(crate) struct PermissionQuery;

#[Object]
impl PermissionQuery {
    async fn permission_by_id(
        &self,
        ctx: &Context<'_>,
        id: String
    ) -> Result<Option<Permission>, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        repo.get::<Permission>(id).await.map_err(|e| e.to_graphql_error())
    }

    async fn permissions(
        &self,
        ctx: &Context<'_>,
        limit: Option<i32>,
        active_only: Option<bool>,
        scope_filter: Option<String>,
        resource_type_filter: Option<String>
    ) -> Result<Vec<Permission>, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        let mut permissions = repo
            .list::<Permission>(limit).await
            .map_err(|e| e.to_graphql_error())?;

        if let Some(true) = active_only {
            permissions = permissions
                .into_iter()
                .filter(|p| p.active && !p.is_expired())
                .collect();
        }

        if let Some(scope) = scope_filter {
            let scope_enum = PermissionScope::from_string(&scope).map_err(|e|
                e.to_graphql_error()
            )?;
            permissions = permissions
                .into_iter()
                .filter(|p| p.scope.to_string() == scope_enum.to_string())
                .collect();
        }

        if let Some(resource_type) = resource_type_filter {
            let resource_type_enum = ResourceType::from_string(&resource_type).map_err(|e|
                e.to_graphql_error()
            )?;
            permissions = permissions
                .into_iter()
                .filter(|p| p.resource_type == resource_type_enum)
                .collect();
        }

        Ok(permissions)
    }

    async fn permissions_for_role(
        &self,
        ctx: &Context<'_>,
        role_id: String,
        active_only: Option<bool>,
        limit: Option<i32>
    ) -> Result<Vec<Permission>, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        let _role = repo
            .get::<Role>(role_id.clone()).await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| {
                AppError::NotFound(format!("Role {} not found", role_id)).to_graphql_error()
            })?;

        let mut permissions = repo
            .list::<Permission>(None).await
            .map_err(|e| e.to_graphql_error())?;

        permissions = permissions
            .into_iter()
            .filter(|p| p.role_id == role_id)
            .collect();

        if let Some(true) = active_only {
            permissions = permissions
                .into_iter()
                .filter(|p| p.active && !p.is_expired())
                .collect();
        }

        if let Some(limit_val) = limit {
            permissions.truncate(limit_val as usize);
        }

        Ok(permissions)
    }

    async fn permissions_by_scope(
        &self,
        ctx: &Context<'_>,
        scope: String,
        active_only: Option<bool>,
        limit: Option<i32>
    ) -> Result<Vec<Permission>, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        let scope_enum = PermissionScope::from_string(&scope).map_err(|e| e.to_graphql_error())?;

        let mut permissions = repo
            .list::<Permission>(None).await
            .map_err(|e| e.to_graphql_error())?;

        permissions = permissions
            .into_iter()
            .filter(|p| p.scope.to_string() == scope_enum.to_string())
            .collect();

        if let Some(true) = active_only {
            permissions = permissions
                .into_iter()
                .filter(|p| p.active && !p.is_expired())
                .collect();
        }

        if let Some(limit_val) = limit {
            permissions.truncate(limit_val as usize);
        }

        Ok(permissions)
    }

    async fn permissions_by_resource_type(
        &self,
        ctx: &Context<'_>,
        resource_type: String,
        active_only: Option<bool>,
        limit: Option<i32>
    ) -> Result<Vec<Permission>, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        let resource_type_enum = ResourceType::from_string(&resource_type).map_err(|e|
            e.to_graphql_error()
        )?;

        let mut permissions = repo
            .list::<Permission>(None).await
            .map_err(|e| e.to_graphql_error())?;

        permissions = permissions
            .into_iter()
            .filter(|p| p.resource_type == resource_type_enum)
            .collect();

        if let Some(true) = active_only {
            permissions = permissions
                .into_iter()
                .filter(|p| p.active && !p.is_expired())
                .collect();
        }

        if let Some(limit_val) = limit {
            permissions.truncate(limit_val as usize);
        }

        Ok(permissions)
    }

    async fn permissions_by_action(
        &self,
        ctx: &Context<'_>,
        action: String,
        active_only: Option<bool>,
        limit: Option<i32>
    ) -> Result<Vec<Permission>, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        let action_enum = PermissionAction::from_string(&action).map_err(|e| e.to_graphql_error())?;

        let mut permissions = repo
            .list::<Permission>(None).await
            .map_err(|e| e.to_graphql_error())?;

        permissions = permissions
            .into_iter()
            .filter(|p| p.actions.contains(&action_enum))
            .collect();

        if let Some(true) = active_only {
            permissions = permissions
                .into_iter()
                .filter(|p| p.active && !p.is_expired())
                .collect();
        }

        if let Some(limit_val) = limit {
            permissions.truncate(limit_val as usize);
        }

        Ok(permissions)
    }

    async fn expired_permissions(
        &self,
        ctx: &Context<'_>,
        limit: Option<i32>
    ) -> Result<Vec<Permission>, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        let mut permissions = repo
            .list::<Permission>(None).await
            .map_err(|e| e.to_graphql_error())?;

        permissions = permissions
            .into_iter()
            .filter(|p| p.is_expired())
            .collect();

        permissions.sort_by(|a, b| {
            b.expires_at
                .unwrap_or_else(|| Utc::now())
                .cmp(&a.expires_at.unwrap_or_else(|| Utc::now()))
        });

        if let Some(limit_val) = limit {
            permissions.truncate(limit_val as usize);
        }

        Ok(permissions)
    }

    async fn permissions_expiring_soon(
        &self,
        ctx: &Context<'_>,
        days_ahead: Option<i32>,
        limit: Option<i32>
    ) -> Result<Vec<Permission>, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        let mut permissions = repo
            .list::<Permission>(None).await
            .map_err(|e| e.to_graphql_error())?;

        let days_ahead = days_ahead.unwrap_or(30);
        let cutoff_date = Utc::now() + chrono::Duration::days(days_ahead as i64);
        let now = Utc::now();

        permissions = permissions
            .into_iter()
            .filter(|p| {
                if let Some(expires_at) = p.expires_at {
                    expires_at > now && expires_at <= cutoff_date
                } else {
                    false
                }
            })
            .collect();

        permissions.sort_by(|a, b| {
            a.expires_at
                .unwrap_or_else(|| Utc::now())
                .cmp(&b.expires_at.unwrap_or_else(|| Utc::now()))
        });

        if let Some(limit_val) = limit {
            permissions.truncate(limit_val as usize);
        }

        Ok(permissions)
    }

    async fn permissions_by_creator(
        &self,
        ctx: &Context<'_>,
        creator_id: String,
        active_only: Option<bool>,
        limit: Option<i32>
    ) -> Result<Vec<Permission>, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        let mut permissions = repo
            .list::<Permission>(None).await
            .map_err(|e| e.to_graphql_error())?;

        permissions = permissions
            .into_iter()
            .filter(|p| p.created_by == creator_id)
            .collect();

        if let Some(true) = active_only {
            permissions = permissions
                .into_iter()
                .filter(|p| p.active && !p.is_expired())
                .collect();
        }

        permissions.sort_by(|a, b| b.created_at.cmp(&a.created_at));

        if let Some(limit_val) = limit {
            permissions.truncate(limit_val as usize);
        }

        Ok(permissions)
    }

    async fn global_permissions(
        &self,
        ctx: &Context<'_>,
        active_only: Option<bool>,
        limit: Option<i32>
    ) -> Result<Vec<Permission>, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        let mut permissions = repo
            .list::<Permission>(None).await
            .map_err(|e| e.to_graphql_error())?;

        permissions = permissions
            .into_iter()
            .filter(|p| matches!(p.scope, PermissionScope::Global))
            .collect();

        if let Some(true) = active_only {
            permissions = permissions
                .into_iter()
                .filter(|p| p.active && !p.is_expired())
                .collect();
        }

        if let Some(limit_val) = limit {
            permissions.truncate(limit_val as usize);
        }

        Ok(permissions)
    }

    async fn permission_statistics(
        &self,
        ctx: &Context<'_>
    ) -> Result<PermissionStatistics, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        let permissions = repo.list::<Permission>(None).await.map_err(|e| e.to_graphql_error())?;

        let total_permissions = permissions.len() as i32;
        let active_permissions = permissions
            .iter()
            .filter(|p| p.active)
            .count() as i32;
        let expired_permissions = permissions
            .iter()
            .filter(|p| p.is_expired())
            .count() as i32;
        let global_permissions = permissions
            .iter()
            .filter(|p| matches!(p.scope, PermissionScope::Global))
            .count() as i32;
        let org_permissions = permissions
            .iter()
            .filter(|p| matches!(p.scope, PermissionScope::Organization))
            .count() as i32;
        let location_permissions = permissions
            .iter()
            .filter(|p| matches!(p.scope, PermissionScope::Location))
            .count() as i32;
        let asset_permissions = permissions
            .iter()
            .filter(|p| matches!(p.scope, PermissionScope::Asset))
            .count() as i32;
        let own_permissions = permissions
            .iter()
            .filter(|p| matches!(p.scope, PermissionScope::Own))
            .count() as i32;

        Ok(PermissionStatistics {
            total_permissions,
            active_permissions,
            expired_permissions,
            global_permissions,
            org_permissions,
            location_permissions,
            asset_permissions,
            own_permissions,
        })
    }

    async fn check_permission(
        &self,
        ctx: &Context<'_>,
        role_id: String,
        resource_type: String,
        action: String,
        resource_id: Option<String>
    ) -> Result<bool, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        let resource_type_enum = ResourceType::from_string(&resource_type).map_err(|e|
            e.to_graphql_error()
        )?;
        let action_enum = PermissionAction::from_string(&action).map_err(|e| e.to_graphql_error())?;

        let permissions = repo.list::<Permission>(None).await.map_err(|e| e.to_graphql_error())?;

        let has_permission = permissions
            .iter()
            .any(|p| {
                p.role_id == role_id &&
                    p.allows_action(&action_enum) &&
                    p.applies_to_resource(
                        &resource_type_enum,
                        &resource_id.clone().unwrap_or_default()
                    )
            });

        Ok(has_permission)
    }
}

#[derive(Debug)]
pub struct PermissionStatistics {
    pub total_permissions: i32,
    pub active_permissions: i32,
    pub expired_permissions: i32,
    pub global_permissions: i32,
    pub org_permissions: i32,
    pub location_permissions: i32,
    pub asset_permissions: i32,
    pub own_permissions: i32,
}

#[Object]
impl PermissionStatistics {
    async fn total_permissions(&self) -> i32 {
        self.total_permissions
    }

    async fn active_permissions(&self) -> i32 {
        self.active_permissions
    }

    async fn expired_permissions(&self) -> i32 {
        self.expired_permissions
    }

    async fn global_permissions(&self) -> i32 {
        self.global_permissions
    }

    async fn org_permissions(&self) -> i32 {
        self.org_permissions
    }

    async fn location_permissions(&self) -> i32 {
        self.location_permissions
    }

    async fn asset_permissions(&self) -> i32 {
        self.asset_permissions
    }

    async fn own_permissions(&self) -> i32 {
        self.own_permissions
    }

    async fn inactive_permissions(&self) -> i32 {
        self.total_permissions - self.active_permissions
    }

    async fn active_percentage(&self) -> f64 {
        if self.total_permissions == 0 {
            0.0
        } else {
            ((self.active_permissions as f64) / (self.total_permissions as f64)) * 100.0
        }
    }
}
