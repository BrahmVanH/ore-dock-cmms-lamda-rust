use async_graphql::*;
use chrono::{ DateTime, Utc };
use tracing::warn;

use crate::{
    error::AppError,
    models::{ user::{ User, UserStatus, UserType }, role::Role, work_order::WorkOrder },
    DbClient,
    Repository,
};

#[derive(Debug, Default)]
pub(crate) struct UserQuery;

#[Object]
impl UserQuery {
    /// Get user by ID
    async fn user_by_id(&self, ctx: &Context<'_>, id: String) -> Result<Option<User>, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        repo.get::<User>(id).await.map_err(|e| e.to_graphql_error())
    }

    /// Get user by username
    async fn user_by_username(
        &self,
        ctx: &Context<'_>,
        username: String
    ) -> Result<Option<User>, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        let users = repo.list::<User>(None).await.map_err(|e| e.to_graphql_error())?;

        let user = users.into_iter().find(|u| u.username.to_lowercase() == username.to_lowercase());

        Ok(user)
    }

    /// Get user by email
    async fn user_by_email(&self, ctx: &Context<'_>, email: String) -> Result<Option<User>, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        let users = repo.list::<User>(None).await.map_err(|e| e.to_graphql_error())?;

        let user = users.into_iter().find(|u| u.email.to_lowercase() == email.to_lowercase());

        Ok(user)
    }

    /// Get user by employee ID
    async fn user_by_employee_id(
        &self,
        ctx: &Context<'_>,
        employee_id: String
    ) -> Result<Option<User>, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        let users = repo.list::<User>(None).await.map_err(|e| e.to_graphql_error())?;

        let user = users
            .into_iter()
            .find(|u| u.employee_id.as_ref().map_or(false, |id| *id == employee_id));

        Ok(user)
    }

    /// Get all users with filtering
    async fn users(
        &self,
        ctx: &Context<'_>,
        limit: Option<i32>,
        status_filter: Option<String>,
        user_type_filter: Option<String>,
        department_filter: Option<String>,
        active_only: Option<bool>
    ) -> Result<Vec<User>, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        let mut users = repo.list::<User>(limit).await.map_err(|e| e.to_graphql_error())?;

        // Apply status filter
        if let Some(status) = status_filter {
            let status_enum = UserStatus::from_string(&status).map_err(|e| e.to_graphql_error())?;
            users = users
                .into_iter()
                .filter(|user| user.status.to_string() == status_enum.to_string())
                .collect();
        }

        // Apply user type filter
        if let Some(user_type) = user_type_filter {
            let type_enum = UserType::from_string(&user_type).map_err(|e| e.to_graphql_error())?;
            users = users
                .into_iter()
                .filter(|user| user.user_type.to_string() == type_enum.to_string())
                .collect();
        }

        // Apply department filter
        if let Some(department) = department_filter {
            users = users
                .into_iter()
                .filter(|user| {
                    user.department.as_ref().map_or(false, |dept| *dept == department)
                })
                .collect();
        }

        // Apply active only filter
        if let Some(true) = active_only {
            users = users
                .into_iter()
                .filter(|user| user.is_active())
                .collect();
        }

        Ok(users)
    }

    /// Get users by status
    async fn users_by_status(
        &self,
        ctx: &Context<'_>,
        status: String,
        limit: Option<i32>
    ) -> Result<Vec<User>, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        let status_enum = UserStatus::from_string(&status).map_err(|e| e.to_graphql_error())?;

        let mut users = repo.list::<User>(None).await.map_err(|e| e.to_graphql_error())?;

        users = users
            .into_iter()
            .filter(|user| user.status.to_string() == status_enum.to_string())
            .collect();

        // Apply limit if provided
        if let Some(limit_val) = limit {
            users.truncate(limit_val as usize);
        }

        Ok(users)
    }

    /// Get users by department
    async fn users_by_department(
        &self,
        ctx: &Context<'_>,
        department: String,
        active_only: Option<bool>,
        limit: Option<i32>
    ) -> Result<Vec<User>, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        let mut users = repo.list::<User>(None).await.map_err(|e| e.to_graphql_error())?;

        users = users
            .into_iter()
            .filter(|user| { user.department.as_ref().map_or(false, |dept| *dept == department) })
            .collect();

        // Apply active only filter
        if let Some(true) = active_only {
            users = users
                .into_iter()
                .filter(|user| user.is_active())
                .collect();
        }

        // Apply limit if provided
        if let Some(limit_val) = limit {
            users.truncate(limit_val as usize);
        }

        Ok(users)
    }

    /// Get users by manager
    async fn users_by_manager(
        &self,
        ctx: &Context<'_>,
        manager_id: String,
        active_only: Option<bool>,
        limit: Option<i32>
    ) -> Result<Vec<User>, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        // Verify manager exists
        let _manager = repo
            .get::<User>(manager_id.clone()).await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| {
                AppError::NotFound(format!("Manager {} not found", manager_id)).to_graphql_error()
            })?;

        let mut users = repo.list::<User>(None).await.map_err(|e| e.to_graphql_error())?;

        users = users
            .into_iter()
            .filter(|user| {
                user.manager_id.as_ref().map_or(false, |mgr_id| *mgr_id == manager_id)
            })
            .collect();

        // Apply active only filter
        if let Some(true) = active_only {
            users = users
                .into_iter()
                .filter(|user| user.is_active())
                .collect();
        }

        // Apply limit if provided
        if let Some(limit_val) = limit {
            users.truncate(limit_val as usize);
        }

        Ok(users)
    }

    /// Get users by role
    async fn users_by_role(
        &self,
        ctx: &Context<'_>,
        role_id: String,
        active_only: Option<bool>,
        limit: Option<i32>
    ) -> Result<Vec<User>, Error> {
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
                AppError::NotFound(format!("Role {} not found", role_id)).to_graphql_error()
            })?;

        let mut users = repo.list::<User>(None).await.map_err(|e| e.to_graphql_error())?;

        users = users
            .into_iter()
            .filter(|user| { user.primary_role_id.as_ref().map_or(false, |r_id| *r_id == role_id) })
            .collect();

        // Apply active only filter
        if let Some(true) = active_only {
            users = users
                .into_iter()
                .filter(|user| user.is_active())
                .collect();
        }

        // Apply limit if provided
        if let Some(limit_val) = limit {
            users.truncate(limit_val as usize);
        }

        Ok(users)
    }

    /// Get recently active users
    async fn recently_active_users(
        &self,
        ctx: &Context<'_>,
        days: Option<i32>,
        limit: Option<i32>
    ) -> Result<Vec<User>, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        let days_back = days.unwrap_or(30);
        let cutoff_date = Utc::now() - chrono::Duration::days(days_back as i64);

        let mut users = repo.list::<User>(None).await.map_err(|e| e.to_graphql_error())?;

        users = users
            .into_iter()
            .filter(|user| {
                if let Some(last_login) = user.last_login_at {
                    last_login >= cutoff_date
                } else {
                    false
                }
            })
            .collect();

        // Sort by last login (most recent first)
        users.sort_by(|a, b| {
            b.last_login_at
                .unwrap_or_else(|| Utc::now())
                .cmp(&a.last_login_at.unwrap_or_else(|| Utc::now()))
        });

        // Apply limit if provided
        if let Some(limit_val) = limit {
            users.truncate(limit_val as usize);
        }

        Ok(users)
    }

    /// Get locked user accounts
    async fn locked_users(
        &self,
        ctx: &Context<'_>,
        limit: Option<i32>
    ) -> Result<Vec<User>, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        let mut users = repo.list::<User>(None).await.map_err(|e| e.to_graphql_error())?;

        users = users
            .into_iter()
            .filter(|user| user.is_account_locked())
            .collect();

        // Apply limit if provided
        if let Some(limit_val) = limit {
            users.truncate(limit_val as usize);
        }

        Ok(users)
    }

    /// Get terminated users
    async fn terminated_users(
        &self,
        ctx: &Context<'_>,
        limit: Option<i32>
    ) -> Result<Vec<User>, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        let mut users = repo.list::<User>(None).await.map_err(|e| e.to_graphql_error())?;

        users = users
            .into_iter()
            .filter(|user| user.is_terminated())
            .collect();

        // Sort by termination date (most recent first)
        users.sort_by(|a, b| {
            b.termination_date
                .unwrap_or_else(|| Utc::now())
                .cmp(&a.termination_date.unwrap_or_else(|| Utc::now()))
        });

        // Apply limit if provided
        if let Some(limit_val) = limit {
            users.truncate(limit_val as usize);
        }

        Ok(users)
    }

    /// Search users by name
    async fn users_by_name(
        &self,
        ctx: &Context<'_>,
        search_term: String,
        exact_match: Option<bool>,
        limit: Option<i32>
    ) -> Result<Vec<User>, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        let users = repo.list::<User>(None).await.map_err(|e| e.to_graphql_error())?;

        let exact_match = exact_match.unwrap_or(false);
        let search_lower = search_term.to_lowercase();

        let mut filtered_users: Vec<User> = if exact_match {
            users
                .into_iter()
                .filter(|user| {
                    user.full_name().to_lowercase() == search_lower ||
                        user.first_name.to_lowercase() == search_lower ||
                        user.last_name.to_lowercase() == search_lower ||
                        user.display_name
                            .as_ref()
                            .map_or(false, |dn| dn.to_lowercase() == search_lower)
                })
                .collect()
        } else {
            users
                .into_iter()
                .filter(|user| {
                    user.full_name().to_lowercase().contains(&search_lower) ||
                        user.first_name.to_lowercase().contains(&search_lower) ||
                        user.last_name.to_lowercase().contains(&search_lower) ||
                        user.display_name
                            .as_ref()
                            .map_or(false, |dn| dn.to_lowercase().contains(&search_lower))
                })
                .collect()
        };

        // Apply limit if provided
        if let Some(limit_val) = limit {
            filtered_users.truncate(limit_val as usize);
        }

        Ok(filtered_users)
    }

    /// Get work orders assigned to a user
    async fn work_orders_for_user(
        &self,
        ctx: &Context<'_>,
        user_id: String,
        limit: Option<i32>
    ) -> Result<Vec<WorkOrder>, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        // Verify user exists
        let _user = repo
            .get::<User>(user_id.clone()).await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| {
                AppError::NotFound(format!("User {} not found", user_id)).to_graphql_error()
            })?;

        // Get all work orders and filter by assigned technician
        let mut work_orders = repo.list::<WorkOrder>(None).await.map_err(|e| e.to_graphql_error())?;

        work_orders = work_orders
            .into_iter()
            .filter(|wo| {
                wo.assigned_technician_id.as_ref().map_or(false, |tech_id| *tech_id == user_id)
            })
            .collect();

        // Sort by creation date (most recent first)
        work_orders.sort_by(|a, b| b.created_at.cmp(&a.created_at));

        // Apply limit if provided
        if let Some(limit_val) = limit {
            work_orders.truncate(limit_val as usize);
        }

        Ok(work_orders)
    }

    /// Get user statistics
    async fn user_statistics(&self, ctx: &Context<'_>) -> Result<UserStatistics, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        let users = repo.list::<User>(None).await.map_err(|e| e.to_graphql_error())?;

        let total_users = users.len() as i32;
        let active_users = users
            .iter()
            .filter(|u| u.is_active())
            .count() as i32;
        let suspended_users = users
            .iter()
            .filter(|u| matches!(u.status, UserStatus::Suspended))
            .count() as i32;
        let locked_users = users
            .iter()
            .filter(|u| u.is_account_locked())
            .count() as i32;
        let terminated_users = users
            .iter()
            .filter(|u| u.is_terminated())
            .count() as i32;

        Ok(UserStatistics {
            total_users,
            active_users,
            suspended_users,
            locked_users,
            terminated_users,
        })
    }
}

/// User statistics summary
#[derive(Debug)]
pub struct UserStatistics {
    pub total_users: i32,
    pub active_users: i32,
    pub suspended_users: i32,
    pub locked_users: i32,
    pub terminated_users: i32,
}

#[Object]
impl UserStatistics {
    async fn total_users(&self) -> i32 {
        self.total_users
    }

    async fn active_users(&self) -> i32 {
        self.active_users
    }

    async fn suspended_users(&self) -> i32 {
        self.suspended_users
    }

    async fn locked_users(&self) -> i32 {
        self.locked_users
    }

    async fn terminated_users(&self) -> i32 {
        self.terminated_users
    }

    async fn inactive_users(&self) -> i32 {
        self.total_users - self.active_users
    }

    async fn active_percentage(&self) -> f64 {
        if self.total_users == 0 {
            0.0
        } else {
            ((self.active_users as f64) / (self.total_users as f64)) * 100.0
        }
    }
}
