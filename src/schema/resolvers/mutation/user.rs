use crate::{
    DbClient,
    models::{ prelude::*, user::{ User, UserStatus, UserType }, role::Role },
    AppError,
    Repository,
};

#[derive(Debug, Default)]
pub struct UserMutation;

#[Object]
impl UserMutation {
    /// Create a new user
    async fn create_user(
        &self,
        ctx: &Context<'_>,
        username: String,
        email: String,
        first_name: String,
        last_name: String,
        display_name: Option<String>,
        user_type: String,
        primary_role_id: Option<String>,
        department: Option<String>,
        job_title: Option<String>,
        manager_id: Option<String>,
        contact_number: Option<String>,
        secondary_email: Option<String>,
        hire_date: Option<DateTime<Utc>>,
        certification_levels: Option<String>, // JSON string
        profile_image_url: Option<String>,
        timezone: Option<String>,
        locale: Option<String>,
        emergency_contact: Option<String>, // JSON string
        address: Option<String>, // JSON string
        employee_id: Option<String>,
        cost_center: Option<String>,
        security_clearance: Option<String>,
        notes: Option<String>,
        metadata: Option<String>, // JSON string
        created_by: Option<String>
    ) -> Result<User, Error> {
        // info!("Creating new user: {}", username);

        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());
        let id = format!("user-{}", Uuid::new_v4());

        // Check if username already exists
        let existing_users = repo.list::<User>(None).await.map_err(|e| e.to_graphql_error())?;
        if existing_users.iter().any(|u| u.username.to_lowercase() == username.to_lowercase()) {
            return Err(
                AppError::ValidationError("Username already exists".to_string()).to_graphql_error()
            );
        }

        // Check if email already exists
        if existing_users.iter().any(|u| u.email.to_lowercase() == email.to_lowercase()) {
            return Err(
                AppError::ValidationError("Email already exists".to_string()).to_graphql_error()
            );
        }

        // Validate role exists if provided
        if let Some(ref role_id) = primary_role_id {
            let _role = repo
                .get::<Role>(role_id.clone()).await
                .map_err(|e| e.to_graphql_error())?
                .ok_or_else(|| {
                    AppError::ValidationError(
                        format!("Role {} not found", role_id)
                    ).to_graphql_error()
                })?;
        }

        // Validate manager exists if provided
        if let Some(ref mgr_id) = manager_id {
            let _manager = repo
                .get::<User>(mgr_id.clone()).await
                .map_err(|e| e.to_graphql_error())?
                .ok_or_else(|| {
                    AppError::ValidationError(
                        format!("Manager {} not found", mgr_id)
                    ).to_graphql_error()
                })?;
        }

        // Parse JSON fields
        let cert_levels = if let Some(ref cert_str) = certification_levels {
            serde_json
                ::from_str::<Json>(cert_str)
                .map_err(|_| {
                    AppError::ValidationError(
                        "Invalid certification levels JSON".to_string()
                    ).to_graphql_error()
                })?
        } else {
            Json::Object(serde_json::Map::new())
        };

        let emergency_contact_json = if let Some(ref ec_str) = emergency_contact {
            Some(
                serde_json
                    ::from_str::<Json>(ec_str)
                    .map_err(|_| {
                        AppError::ValidationError(
                            "Invalid emergency contact JSON".to_string()
                        ).to_graphql_error()
                    })?
            )
        } else {
            None
        };

        let address_json = if let Some(ref addr_str) = address {
            Some(
                serde_json
                    ::from_str::<Json>(addr_str)
                    .map_err(|_| {
                        AppError::ValidationError(
                            "Invalid address JSON".to_string()
                        ).to_graphql_error()
                    })?
            )
        } else {
            None
        };

        let metadata_json = if let Some(ref meta_str) = metadata {
            Some(
                serde_json
                    ::from_str::<Json>(meta_str)
                    .map_err(|_| {
                        AppError::ValidationError(
                            "Invalid metadata JSON".to_string()
                        ).to_graphql_error()
                    })?
            )
        } else {
            None
        };

        let user = User::new(
            id,
            username,
            email,
            first_name,
            last_name,
            display_name,
            user_type,
            "active".to_string(), // Default to active status
            primary_role_id,
            department,
            job_title,
            manager_id,
            contact_number,
            secondary_email,
            hire_date,
            cert_levels,
            profile_image_url,
            timezone,
            locale,
            emergency_contact_json,
            address_json,
            employee_id,
            cost_center,
            security_clearance,
            notes,
            metadata_json,
            created_by
        ).map_err(|e| e.to_graphql_error())?;

        repo.create(user).await.map_err(|e| e.to_graphql_error())
    }

    /// Update an existing user
    async fn update_user(
        &self,
        ctx: &Context<'_>,
        id: String,
        username: Option<String>,
        email: Option<String>,
        first_name: Option<String>,
        last_name: Option<String>,
        display_name: Option<String>,
        user_type: Option<String>,
        primary_role_id: Option<String>,
        department: Option<String>,
        job_title: Option<String>,
        manager_id: Option<String>,
        contact_number: Option<String>,
        secondary_email: Option<String>,
        certification_levels: Option<String>,
        profile_image_url: Option<String>,
        timezone: Option<String>,
        locale: Option<String>,
        emergency_contact: Option<String>,
        address: Option<String>,
        employee_id: Option<String>,
        cost_center: Option<String>,
        security_clearance: Option<String>,
        notes: Option<String>,
        metadata: Option<String>
    ) -> Result<User, Error> {
        // info!("Updating user: {}", id);

        let db_client = ctx
            .data::<DbClient>()
            .map_err(|_| {
                AppError::InternalServerError("Database client not available".to_string())
            })?;

        let repo = Repository::new(db_client.clone());

        let mut user = repo
            .get::<User>(id.clone()).await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| AppError::NotFound(format!("User {} not found", id)))?;

        // Check if user is terminated
        if user.is_terminated() {
            return Err(
                AppError::ValidationError(
                    "Cannot update terminated user".to_string()
                ).to_graphql_error()
            );
        }

        // Update fields if provided
        if let Some(new_username) = username {
            // Check if new username already exists (excluding current user)
            let existing_users = repo.list::<User>(None).await.map_err(|e| e.to_graphql_error())?;
            if
                existing_users
                    .iter()
                    .any(
                        |u|
                            u.id != user.id &&
                            u.username.to_lowercase() == new_username.to_lowercase()
                    )
            {
                return Err(
                    AppError::ValidationError(
                        "Username already exists".to_string()
                    ).to_graphql_error()
                );
            }
            user.username = new_username;
        }

        if let Some(new_email) = email {
            // Check if new email already exists (excluding current user)
            let existing_users = repo.list::<User>(None).await.map_err(|e| e.to_graphql_error())?;
            if
                existing_users
                    .iter()
                    .any(|u| u.id != user.id && u.email.to_lowercase() == new_email.to_lowercase())
            {
                return Err(
                    AppError::ValidationError("Email already exists".to_string()).to_graphql_error()
                );
            }
            user.email = new_email;
        }

        if let Some(fname) = first_name {
            user.first_name = fname;
        }
        if let Some(lname) = last_name {
            user.last_name = lname;
        }
        if let Some(dname) = display_name {
            user.display_name = if dname.is_empty() { None } else { Some(dname) };
        }
        if let Some(utype) = user_type {
            user.user_type = UserType::from_string(&utype).map_err(|e| e.to_graphql_error())?;
        }
        if let Some(role_id) = primary_role_id {
            if !role_id.is_empty() {
                // Validate role exists
                let _role = repo
                    .get::<Role>(role_id.clone()).await
                    .map_err(|e| e.to_graphql_error())?
                    .ok_or_else(|| {
                        AppError::ValidationError(
                            format!("Role {} not found", role_id)
                        ).to_graphql_error()
                    })?;
                user.primary_role_id = Some(role_id);
            } else {
                user.primary_role_id = None;
            }
        }
        if let Some(dept) = department {
            user.department = if dept.is_empty() { None } else { Some(dept) };
        }
        if let Some(title) = job_title {
            user.job_title = if title.is_empty() { None } else { Some(title) };
        }
        if let Some(mgr_id) = manager_id {
            if !mgr_id.is_empty() {
                // Validate manager exists
                let _manager = repo
                    .get::<User>(mgr_id.clone()).await
                    .map_err(|e| e.to_graphql_error())?
                    .ok_or_else(|| {
                        AppError::ValidationError(
                            format!("Manager {} not found", mgr_id)
                        ).to_graphql_error()
                    })?;
                user.manager_id = Some(mgr_id);
            } else {
                user.manager_id = None;
            }
        }
        if let Some(contact) = contact_number {
            user.contact_number = if contact.is_empty() { None } else { Some(contact) };
        }
        if let Some(sec_email) = secondary_email {
            user.secondary_email = if sec_email.is_empty() { None } else { Some(sec_email) };
        }
        if let Some(cert_str) = certification_levels {
            user.certification_levels = serde_json
                ::from_str::<Json>(&cert_str)
                .map_err(|_| {
                    AppError::ValidationError(
                        "Invalid certification levels JSON".to_string()
                    ).to_graphql_error()
                })?;
        }
        if let Some(img_url) = profile_image_url {
            user.profile_image_url = if img_url.is_empty() { None } else { Some(img_url) };
        }
        if let Some(tz) = timezone {
            user.timezone = if tz.is_empty() { None } else { Some(tz) };
        }
        if let Some(loc) = locale {
            user.locale = if loc.is_empty() { None } else { Some(loc) };
        }
        if let Some(ec_str) = emergency_contact {
            user.emergency_contact = if ec_str.is_empty() {
                None
            } else {
                Some(
                    serde_json
                        ::from_str::<Json>(&ec_str)
                        .map_err(|_| {
                            AppError::ValidationError(
                                "Invalid emergency contact JSON".to_string()
                            ).to_graphql_error()
                        })?
                )
            };
        }
        if let Some(addr_str) = address {
            user.address = if addr_str.is_empty() {
                None
            } else {
                Some(
                    serde_json
                        ::from_str::<Json>(&addr_str)
                        .map_err(|_| {
                            AppError::ValidationError(
                                "Invalid address JSON".to_string()
                            ).to_graphql_error()
                        })?
                )
            };
        }
        if let Some(emp_id) = employee_id {
            user.employee_id = if emp_id.is_empty() { None } else { Some(emp_id) };
        }
        if let Some(cost) = cost_center {
            user.cost_center = if cost.is_empty() { None } else { Some(cost) };
        }
        if let Some(clearance) = security_clearance {
            user.security_clearance = if clearance.is_empty() { None } else { Some(clearance) };
        }
        if let Some(user_notes) = notes {
            user.notes = if user_notes.is_empty() { None } else { Some(user_notes) };
        }
        if let Some(meta_str) = metadata {
            user.metadata = if meta_str.is_empty() {
                None
            } else {
                Some(
                    serde_json
                        ::from_str::<Json>(&meta_str)
                        .map_err(|_| {
                            AppError::ValidationError(
                                "Invalid metadata JSON".to_string()
                            ).to_graphql_error()
                        })?
                )
            };
        }

        user.updated_at = Utc::now();

        repo.update(user).await.map_err(|e| e.to_graphql_error())
    }

    /// Update user status
    async fn update_user_status(
        &self,
        ctx: &Context<'_>,
        id: String,
        status: String,
        reason: Option<String>
    ) -> Result<User, Error> {
        // info!("Updating user status: {} to {}", id, status);

        let db_client = ctx
            .data::<DbClient>()
            .map_err(|_| {
                AppError::InternalServerError("Database client not available".to_string())
            })?;

        let repo = Repository::new(db_client.clone());

        let mut user = repo
            .get::<User>(id.clone()).await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| AppError::NotFound(format!("User {} not found", id)))?;

        let new_status = UserStatus::from_string(&status).map_err(|e| e.to_graphql_error())?;

        user.status = new_status;

        // Add reason to notes if provided
        if let Some(reason_text) = reason {
            let current_notes = user.notes.clone().unwrap_or_default();
            user.notes = Some(format!("{}; STATUS CHANGE: {}", current_notes, reason_text));
        }

        user.updated_at = Utc::now();

        repo.update(user).await.map_err(|e| e.to_graphql_error())
    }

    /// Suspend a user account
    async fn suspend_user(
        &self,
        ctx: &Context<'_>,
        id: String,
        reason: Option<String>
    ) -> Result<User, Error> {
        // info!("Suspending user: {}", id);

        let db_client = ctx
            .data::<DbClient>()
            .map_err(|_| {
                AppError::InternalServerError("Database client not available".to_string())
            })?;

        let repo = Repository::new(db_client.clone());

        let mut user = repo
            .get::<User>(id.clone()).await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| AppError::NotFound(format!("User {} not found", id)))?;

        user.status = UserStatus::Suspended;

        if let Some(reason_text) = reason {
            let current_notes = user.notes.clone().unwrap_or_default();
            user.notes = Some(format!("{}; SUSPENDED: {}", current_notes, reason_text));
        }

        user.updated_at = Utc::now();

        repo.update(user).await.map_err(|e| e.to_graphql_error())
    }

    /// Reactivate a suspended user account
    async fn reactivate_user(&self, ctx: &Context<'_>, id: String) -> Result<User, Error> {
        // info!("Reactivating user: {}", id);

        let db_client = ctx
            .data::<DbClient>()
            .map_err(|_| {
                AppError::InternalServerError("Database client not available".to_string())
            })?;

        let repo = Repository::new(db_client.clone());

        let mut user = repo
            .get::<User>(id.clone()).await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| AppError::NotFound(format!("User {} not found", id)))?;

        if !matches!(user.status, UserStatus::Suspended | UserStatus::Inactive) {
            return Err(
                AppError::ValidationError(
                    "Only suspended or inactive users can be reactivated".to_string()
                ).to_graphql_error()
            );
        }

        user.status = UserStatus::Active;
        user.failed_login_attempts = 0;
        user.account_locked_until = None;
        user.updated_at = Utc::now();

        repo.update(user).await.map_err(|e| e.to_graphql_error())
    }

    /// Terminate a user account
    async fn terminate_user(
        &self,
        ctx: &Context<'_>,
        id: String,
        termination_date: Option<DateTime<Utc>>,
        reason: Option<String>
    ) -> Result<User, Error> {
        // info!("Terminating user: {}", id);

        let db_client = ctx
            .data::<DbClient>()
            .map_err(|_| {
                AppError::InternalServerError("Database client not available".to_string())
            })?;

        let repo = Repository::new(db_client.clone());

        let mut user = repo
            .get::<User>(id.clone()).await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| AppError::NotFound(format!("User {} not found", id)))?;

        if matches!(user.status, UserStatus::Terminated) {
            return Err(
                AppError::ValidationError(
                    "User is already terminated".to_string()
                ).to_graphql_error()
            );
        }

        user.status = UserStatus::Terminated;
        user.termination_date = termination_date.or_else(|| Some(Utc::now()));

        if let Some(reason_text) = reason {
            let current_notes = user.notes.clone().unwrap_or_default();
            user.notes = Some(format!("{}; TERMINATED: {}", current_notes, reason_text));
        }

        user.updated_at = Utc::now();

        repo.update(user).await.map_err(|e| e.to_graphql_error())
    }

    /// Unlock a user account
    async fn unlock_user_account(&self, ctx: &Context<'_>, id: String) -> Result<User, Error> {
        // info!("Unlocking user account: {}", id);

        let db_client = ctx
            .data::<DbClient>()
            .map_err(|_| {
                AppError::InternalServerError("Database client not available".to_string())
            })?;

        let repo = Repository::new(db_client.clone());

        let mut user = repo
            .get::<User>(id.clone()).await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| AppError::NotFound(format!("User {} not found", id)))?;

        user.failed_login_attempts = 0;
        user.account_locked_until = None;
        user.updated_at = Utc::now();

        repo.update(user).await.map_err(|e| e.to_graphql_error())
    }

    /// Record user login
    async fn record_user_login(&self, ctx: &Context<'_>, id: String) -> Result<User, Error> {
        // info!("Recording user login: {}", id);

        let db_client = ctx
            .data::<DbClient>()
            .map_err(|_| {
                AppError::InternalServerError("Database client not available".to_string())
            })?;

        let repo = Repository::new(db_client.clone());

        let mut user = repo
            .get::<User>(id.clone()).await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| AppError::NotFound(format!("User {} not found", id)))?;

        user.record_login();

        repo.update(user).await.map_err(|e| e.to_graphql_error())
    }

    /// Record password change
    async fn record_user_password_change(
        &self,
        ctx: &Context<'_>,
        id: String
    ) -> Result<User, Error> {
        // info!("Recording password change for user: {}", id);

        let db_client = ctx
            .data::<DbClient>()
            .map_err(|_| {
                AppError::InternalServerError("Database client not available".to_string())
            })?;

        let repo = Repository::new(db_client.clone());

        let mut user = repo
            .get::<User>(id.clone()).await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| AppError::NotFound(format!("User {} not found", id)))?;

        user.password_changed_at = Some(Utc::now());
        user.updated_at = Utc::now();

        repo.update(user).await.map_err(|e| e.to_graphql_error())
    }

    /// Delete a user (soft delete by terminating)
    async fn delete_user(
        &self,
        ctx: &Context<'_>,
        id: String,
        reason: Option<String>
    ) -> Result<bool, Error> {
        // info!("Deleting user: {}", id);

        let db_client = ctx
            .data::<DbClient>()
            .map_err(|_| {
                AppError::InternalServerError("Database client not available".to_string())
            })?;

        let repo = Repository::new(db_client.clone());

        let mut user = repo
            .get::<User>(id.clone()).await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| AppError::NotFound(format!("User {} not found", id)))?;

        // Soft delete by terminating
        user.status = UserStatus::Terminated;
        user.termination_date = Some(Utc::now());

        if let Some(reason_text) = reason {
            let current_notes = user.notes.clone().unwrap_or_default();
            user.notes = Some(format!("{}; DELETED: {}", current_notes, reason_text));
        }

        user.updated_at = Utc::now();

        repo.update(user).await.map_err(|e| e.to_graphql_error())?;

        Ok(true)
    }

    /// Hard delete a user (permanent removal)
    async fn permanently_delete_user(&self, ctx: &Context<'_>, id: String) -> Result<bool, Error> {
        // info!("Permanently deleting user: {}", id);

        let db_client = ctx
            .data::<DbClient>()
            .map_err(|_| {
                AppError::InternalServerError("Database client not available".to_string())
            })?;

        let repo = Repository::new(db_client.clone());

        // Verify user exists
        let user = repo
            .get::<User>(id.clone()).await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| AppError::NotFound(format!("User {} not found", id)))?;

        // Business rule: Only allow permanent deletion of terminated users
        if !user.is_terminated() {
            return Err(
                AppError::ValidationError(
                    "Only terminated users can be permanently deleted".to_string()
                ).to_graphql_error()
            );
        }

        repo.delete::<User>(id).await.map_err(|e| e.to_graphql_error())
    }
}
