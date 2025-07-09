use async_graphql::*;
use chrono::{ DateTime, Utc };
use tracing::warn;

use crate::{
    error::AppError,
    models::{ role::{ Role, RoleType }, user_role::UserRole, permission::Permission },
    DbClient,
    Repository,
};

#[derive(Debug, Default)]
pub(crate) struct RoleQuery;

#[Object]
impl RoleQuery {
    async fn role_by_id(&self, ctx: &Context<'_>, id: String) -> Result<Option<Role>, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        repo.get::<Role>(id).await.map_err(|e| e.to_graphql_error())
    }

    async fn roles(
        &self,
        ctx: &Context<'_>,
        limit: Option<i32>,
        role_type_filter: Option<String>,
        active_only: Option<bool>,
        system_roles_only: Option<bool>
    ) -> Result<Vec<Role>, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        let mut roles = repo.list::<Role>(limit).await.map_err(|e| e.to_graphql_error())?;

        if let Some(role_type) = role_type_filter {
            let type_enum = RoleType::from_string(&role_type).map_err(|e| e.to_graphql_error())?;
            roles = roles
                .into_iter()
                .filter(|role| role.role_type == type_enum)
                .collect();
        }

        if let Some(true) = active_only {
            roles = roles
                .into_iter()
                .filter(|role| role.is_usable())
                .collect();
        }

        if let Some(true) = system_roles_only {
            roles = roles
                .into_iter()
                .filter(|role| role.is_system_role)
                .collect();
        } else if let Some(false) = system_roles_only {
            roles = roles
                .into_iter()
                .filter(|role| !role.is_system_role)
                .collect();
        }

        Ok(roles)
    }

    async fn roles_by_type(
        &self,
        ctx: &Context<'_>,
        role_type: String,
        active_only: Option<bool>,
        limit: Option<i32>
    ) -> Result<Vec<Role>, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        let type_enum = RoleType::from_string(&role_type).map_err(|e| e.to_graphql_error())?;

        let mut roles = repo.list::<Role>(None).await.map_err(|e| e.to_graphql_error())?;

        roles = roles
            .into_iter()
            .filter(|role| role.role_type == type_enum)
            .collect();

        if let Some(true) = active_only {
            roles = roles
                .into_iter()
                .filter(|role| role.is_usable())
                .collect();
        }

        roles.sort_by(|a, b| b.priority.cmp(&a.priority));

        if let Some(limit_val) = limit {
            roles.truncate(limit_val as usize);
        }

        Ok(roles)
    }

    async fn roles_by_name(
        &self,
        ctx: &Context<'_>,
        name: String,
        exact_match: Option<bool>,
        limit: Option<i32>
    ) -> Result<Vec<Role>, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        let roles = repo.list::<Role>(None).await.map_err(|e| e.to_graphql_error())?;

        let exact_match = exact_match.unwrap_or(false);

        let mut filtered_roles: Vec<Role> = if exact_match {
            roles
                .into_iter()
                .filter(|role| role.name.to_lowercase() == name.to_lowercase())
                .collect()
        } else {
            roles
                .into_iter()
                .filter(|role| role.name.to_lowercase().contains(&name.to_lowercase()))
                .collect()
        };

        if let Some(limit_val) = limit {
            filtered_roles.truncate(limit_val as usize);
        }

        Ok(filtered_roles)
    }

    async fn system_roles(
        &self,
        ctx: &Context<'_>,
        active_only: Option<bool>,
        limit: Option<i32>
    ) -> Result<Vec<Role>, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        let mut roles = repo.list::<Role>(None).await.map_err(|e| e.to_graphql_error())?;

        roles = roles
            .into_iter()
            .filter(|role| role.is_system_role)
            .collect();

        if let Some(true) = active_only {
            roles = roles
                .into_iter()
                .filter(|role| role.is_usable())
                .collect();
        }

        roles.sort_by(|a, b| b.priority.cmp(&a.priority));

        if let Some(limit_val) = limit {
            roles.truncate(limit_val as usize);
        }

        Ok(roles)
    }

    async fn custom_roles(
        &self,
        ctx: &Context<'_>,
        active_only: Option<bool>,
        limit: Option<i32>
    ) -> Result<Vec<Role>, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        let mut roles = repo.list::<Role>(None).await.map_err(|e| e.to_graphql_error())?;

        roles = roles
            .into_iter()
            .filter(|role| !role.is_system_role)
            .collect();

        if let Some(true) = active_only {
            roles = roles
                .into_iter()
                .filter(|role| role.is_usable())
                .collect();
        }

        roles.sort_by(|a, b| b.priority.cmp(&a.priority));

        if let Some(limit_val) = limit {
            roles.truncate(limit_val as usize);
        }

        Ok(roles)
    }

    async fn roles_by_priority(
        &self,
        ctx: &Context<'_>,
        min_priority: Option<i32>,
        max_priority: Option<i32>,
        limit: Option<i32>
    ) -> Result<Vec<Role>, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        let mut roles = repo.list::<Role>(None).await.map_err(|e| e.to_graphql_error())?;

        if let Some(min) = min_priority {
            roles = roles
                .into_iter()
                .filter(|role| role.priority >= min)
                .collect();
        }

        if let Some(max) = max_priority {
            roles = roles
                .into_iter()
                .filter(|role| role.priority <= max)
                .collect();
        }

        roles.sort_by(|a, b| b.priority.cmp(&a.priority));

        if let Some(limit_val) = limit {
            roles.truncate(limit_val as usize);
        }

        Ok(roles)
    }

    async fn expired_roles(
        &self,
        ctx: &Context<'_>,
        limit: Option<i32>
    ) -> Result<Vec<Role>, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        let mut roles = repo.list::<Role>(None).await.map_err(|e| e.to_graphql_error())?;

        roles = roles
            .into_iter()
            .filter(|role| role.is_expired())
            .collect();

        roles.sort_by(|a, b| {
            b.expires_at
                .unwrap_or_else(|| Utc::now())
                .cmp(&a.expires_at.unwrap_or_else(|| Utc::now()))
        });

        if let Some(limit_val) = limit {
            roles.truncate(limit_val as usize);
        }

        Ok(roles)
    }

    async fn roles_expiring_soon(
        &self,
        ctx: &Context<'_>,
        days_ahead: Option<i32>,
        limit: Option<i32>
    ) -> Result<Vec<Role>, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        let mut roles = repo.list::<Role>(None).await.map_err(|e| e.to_graphql_error())?;

        let days_ahead = days_ahead.unwrap_or(30);
        let cutoff_date = Utc::now() + chrono::Duration::days(days_ahead as i64);
        let now = Utc::now();

        roles = roles
            .into_iter()
            .filter(|role| {
                if let Some(expires_at) = role.expires_at {
                    expires_at > now && expires_at <= cutoff_date
                } else {
                    false
                }
            })
            .collect();

        roles.sort_by(|a, b| {
            a.expires_at
                .unwrap_or_else(|| Utc::now())
                .cmp(&b.expires_at.unwrap_or_else(|| Utc::now()))
        });

        if let Some(limit_val) = limit {
            roles.truncate(limit_val as usize);
        }

        Ok(roles)
    }

    async fn roles_with_permission(
        &self,
        ctx: &Context<'_>,
        permission_id: String,
        active_only: Option<bool>,
        limit: Option<i32>
    ) -> Result<Vec<Role>, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        let _permission = repo
            .get::<Permission>(permission_id.clone()).await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| {
                AppError::NotFound(
                    format!("Permission {} not found", permission_id)
                ).to_graphql_error()
            })?;

        let mut roles = repo.list::<Role>(None).await.map_err(|e| e.to_graphql_error())?;

        roles = roles
            .into_iter()
            .filter(|role| role.has_permission(&permission_id))
            .collect();

        if let Some(true) = active_only {
            roles = roles
                .into_iter()
                .filter(|role| role.is_usable())
                .collect();
        }

        roles.sort_by(|a, b| b.priority.cmp(&a.priority));

        if let Some(limit_val) = limit {
            roles.truncate(limit_val as usize);
        }

        Ok(roles)
    }

    async fn child_roles(
        &self,
        ctx: &Context<'_>,
        parent_role_id: String,
        active_only: Option<bool>,
        limit: Option<i32>
    ) -> Result<Vec<Role>, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        let _parent_role = repo
            .get::<Role>(parent_role_id.clone()).await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| {
                AppError::NotFound(
                    format!("Parent role {} not found", parent_role_id)
                ).to_graphql_error()
            })?;

        let mut roles = repo.list::<Role>(None).await.map_err(|e| e.to_graphql_error())?;

        roles = roles
            .into_iter()
            .filter(|role| {
                role.parent_role_id.as_ref().map_or(false, |parent_id| *parent_id == parent_role_id)
            })
            .collect();

        if let Some(true) = active_only {
            roles = roles
                .into_iter()
                .filter(|role| role.is_usable())
                .collect();
        }

        roles.sort_by(|a, b| b.priority.cmp(&a.priority));

        if let Some(limit_val) = limit {
            roles.truncate(limit_val as usize);
        }

        Ok(roles)
    }

    async fn roles_by_creator(
        &self,
        ctx: &Context<'_>,
        creator_id: String,
        active_only: Option<bool>,
        limit: Option<i32>
    ) -> Result<Vec<Role>, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        let mut roles = repo.list::<Role>(None).await.map_err(|e| e.to_graphql_error())?;

        roles = roles
            .into_iter()
            .filter(|role| {
                role.created_by.as_ref().map_or(false, |creator| *creator == creator_id)
            })
            .collect();

        if let Some(true) = active_only {
            roles = roles
                .into_iter()
                .filter(|role| role.is_usable())
                .collect();
        }

        roles.sort_by(|a, b| b.created_at.cmp(&a.created_at));

        if let Some(limit_val) = limit {
            roles.truncate(limit_val as usize);
        }

        Ok(roles)
    }

    async fn role_hierarchy(
        &self,
        ctx: &Context<'_>,
        root_role_id: Option<String>,
        active_only: Option<bool>
    ) -> Result<Vec<Role>, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        let mut all_roles = repo.list::<Role>(None).await.map_err(|e| e.to_graphql_error())?;

        if let Some(true) = active_only {
            all_roles = all_roles
                .into_iter()
                .filter(|role| role.is_usable())
                .collect();
        }

        if let Some(root_id) = root_role_id {
            let mut result = Vec::new();
            let mut to_process = vec![root_id];

            while let Some(current_id) = to_process.pop() {
                if let Some(role) = all_roles.iter().find(|r| r.id == current_id) {
                    result.push(role.clone());

                    let children: Vec<String> = all_roles
                        .iter()
                        .filter(|r| r.parent_role_id.as_ref().map_or(false, |p| *p == current_id))
                        .map(|r| r.id.clone())
                        .collect();

                    to_process.extend(children);
                }
            }

            result.sort_by(|a, b| b.priority.cmp(&a.priority));
            Ok(result)
        } else {
            let root_roles: Vec<Role> = all_roles
                .into_iter()
                .filter(|role| role.parent_role_id.is_none())
                .collect();

            Ok(root_roles)
        }
    }

    async fn user_assignments_for_role(
        &self,
        ctx: &Context<'_>,
        role_id: String,
        effective_only: Option<bool>,
        limit: Option<i32>
    ) -> Result<Vec<UserRole>, Error> {
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

        let mut user_roles = repo.list::<UserRole>(None).await.map_err(|e| e.to_graphql_error())?;

        user_roles = user_roles
            .into_iter()
            .filter(|ur| ur.role_id == role_id)
            .collect();

        if let Some(true) = effective_only {
            user_roles = user_roles
                .into_iter()
                .filter(|ur| ur.is_effective())
                .collect();
        }

        user_roles.sort_by(|a, b| b.assigned_at.cmp(&a.assigned_at));

        if let Some(limit_val) = limit {
            user_roles.truncate(limit_val as usize);
        }

        Ok(user_roles)
    }

    async fn role_usage_statistics(
        &self,
        ctx: &Context<'_>,
        role_id: String
    ) -> Result<RoleUsageStatistics, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        let role = repo
            .get::<Role>(role_id.clone()).await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| {
                AppError::NotFound(format!("Role {} not found", role_id)).to_graphql_error()
            })?;

        let user_roles = repo.list::<UserRole>(None).await.map_err(|e| e.to_graphql_error())?;

        let role_assignments: Vec<UserRole> = user_roles
            .into_iter()
            .filter(|ur| ur.role_id == role_id)
            .collect();

        let total_assignments = role_assignments.len() as i32;
        let effective_assignments = role_assignments
            .iter()
            .filter(|ur| ur.is_effective())
            .count() as i32;
        let primary_assignments = role_assignments
            .iter()
            .filter(|ur| ur.is_primary_role)
            .count() as i32;

        Ok(RoleUsageStatistics {
            role,
            total_assignments,
            effective_assignments,
            primary_assignments,
        })
    }

    async fn role_statistics(&self, ctx: &Context<'_>) -> Result<RoleStatistics, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        let roles = repo.list::<Role>(None).await.map_err(|e| e.to_graphql_error())?;

        let total_roles = roles.len() as i32;
        let active_roles = roles
            .iter()
            .filter(|r| r.active)
            .count() as i32;
        let system_roles = roles
            .iter()
            .filter(|r| r.is_system_role)
            .count() as i32;
        let custom_roles = roles
            .iter()
            .filter(|r| !r.is_system_role)
            .count() as i32;
        let expired_roles = roles
            .iter()
            .filter(|r| r.is_expired())
            .count() as i32;
        let usable_roles = roles
            .iter()
            .filter(|r| r.is_usable())
            .count() as i32;

        Ok(RoleStatistics {
            total_roles,
            active_roles,
            system_roles,
            custom_roles,
            expired_roles,
            usable_roles,
        })
    }
}

#[derive(Debug)]
pub struct RoleUsageStatistics {
    pub role: Role,
    pub total_assignments: i32,
    pub effective_assignments: i32,
    pub primary_assignments: i32,
}

#[Object]
impl RoleUsageStatistics {
    async fn role(&self) -> &Role {
        &self.role
    }

    async fn total_assignments(&self) -> i32 {
        self.total_assignments
    }

    async fn effective_assignments(&self) -> i32 {
        self.effective_assignments
    }

    async fn primary_assignments(&self) -> i32 {
        self.primary_assignments
    }

    async fn inactive_assignments(&self) -> i32 {
        self.total_assignments - self.effective_assignments
    }

    async fn usage_percentage(&self) -> f64 {
        if let Some(max_users) = self.role.max_users {
            if max_users > 0 {
                ((self.effective_assignments as f64) / (max_users as f64)) * 100.0
            } else {
                0.0
            }
        } else {
            0.0
        }
    }
}

#[derive(Debug)]
pub struct RoleStatistics {
    pub total_roles: i32,
    pub active_roles: i32,
    pub system_roles: i32,
    pub custom_roles: i32,
    pub expired_roles: i32,
    pub usable_roles: i32,
}

#[Object]
impl RoleStatistics {
    async fn total_roles(&self) -> i32 {
        self.total_roles
    }

    async fn active_roles(&self) -> i32 {
        self.active_roles
    }

    async fn system_roles(&self) -> i32 {
        self.system_roles
    }

    async fn custom_roles(&self) -> i32 {
        self.custom_roles
    }

    async fn expired_roles(&self) -> i32 {
        self.expired_roles
    }

    async fn usable_roles(&self) -> i32 {
        self.usable_roles
    }

    async fn inactive_roles(&self) -> i32 {
        self.total_roles - self.active_roles
    }

    async fn active_percentage(&self) -> f64 {
        if self.total_roles == 0 {
            0.0
        } else {
            ((self.active_roles as f64) / (self.total_roles as f64)) * 100.0
        }
    }
}
