use std::collections::HashMap;

use async_graphql::Object;
use aws_sdk_dynamodb::types::AttributeValue;
use chrono::{ DateTime, Utc };
use serde::{ Deserialize, Serialize };
use serde_json::Value as Json;
use tracing::info;

use crate::error::AppError;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UserStatus {
    Active,
    Inactive,
    Suspended,
    Pending,
    Locked,
    Terminated,
}

impl UserStatus {
    pub(crate) fn to_str(&self) -> &str {
        match self {
            UserStatus::Active => "active",
            UserStatus::Inactive => "inactive",
            UserStatus::Suspended => "suspended",
            UserStatus::Pending => "pending",
            UserStatus::Locked => "locked",
            UserStatus::Terminated => "terminated",
        }
    }

    pub(crate) fn to_string(&self) -> String {
        self.to_str().to_string()
    }

    pub(crate) fn from_string(s: &str) -> Result<UserStatus, AppError> {
        match s {
            "active" => Ok(Self::Active),
            "inactive" => Ok(Self::Inactive),
            "suspended" => Ok(Self::Suspended),
            "pending" => Ok(Self::Pending),
            "locked" => Ok(Self::Locked),
            "terminated" => Ok(Self::Terminated),
            _ => Err(AppError::ValidationError("Invalid user status".to_string())),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UserType {
    Employee,
    Admin,
    System,
    Service,
}

impl UserType {
    pub(crate) fn to_str(&self) -> &str {
        match self {
            UserType::Employee => "employee",
            UserType::Admin => "admin",
            UserType::System => "system",
            UserType::Service => "service",
        }
    }

    pub(crate) fn to_string(&self) -> String {
        self.to_str().to_string()
    }

    pub(crate) fn from_string(s: &str) -> Result<UserType, AppError> {
        match s {
            "employee" => Ok(Self::Employee),
            "admin" => Ok(Self::Admin),
            "system" => Ok(Self::System),
            "service" => Ok(Self::Service),
            _ => Err(AppError::ValidationError("Invalid user type".to_string())),
        }
    }
}

/// Represents a User in the system
///
/// # Fields
///
/// * `id` - Unique identifier for the user
/// * `username` - Unique username for login
/// * `email` - User's email address
/// * `first_name` - User's first name
/// * `last_name` - User's last name
/// * `display_name` - Optional display name
/// * `user_type` - Type of user (employee, contractor, etc.)
/// * `status` - Current status of the user account
/// * `primary_role_id` - ID of the user's primary role
/// * `department` - Department the user belongs to
/// * `job_title` - User's job title
/// * `manager_id` - ID of the user's manager
/// * `contact_number` - Primary contact phone number
/// * `secondary_email` - Optional secondary email
/// * `hire_date` - Date when user was hired
/// * `termination_date` - Date when user was terminated (if applicable)
/// * `last_login_at` - Last login timestamp
/// * `password_changed_at` - When password was last changed
/// * `failed_login_attempts` - Number of consecutive failed login attempts
/// * `account_locked_until` - When account lock expires (if locked)
/// * `certification_levels` - JSON object containing certification information
/// * `profile_image_url` - URL to user's profile image
/// * `timezone` - User's timezone preference
/// * `locale` - User's locale preference
/// * `emergency_contact` - Emergency contact information as JSON
/// * `address` - User's address information as JSON
/// * `employee_id` - External employee ID
/// * `cost_center` - Cost center for billing/reporting
/// * `security_clearance` - Security clearance level
/// * `notes` - Administrative notes about the user
/// * `metadata` - Additional metadata as JSON
/// * `created_by` - User who created this account
/// * `created_at` - Date and time of creation
/// * `updated_at` - Date and time of last update
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub display_name: Option<String>,
    pub user_type: UserType,
    pub status: UserStatus,
    pub primary_role_id: Option<String>,
    pub department: Option<String>,
    pub job_title: Option<String>,
    pub manager_id: Option<String>,
    pub contact_number: Option<String>,
    pub secondary_email: Option<String>,
    pub hire_date: Option<DateTime<Utc>>,
    pub termination_date: Option<DateTime<Utc>>,
    pub last_login_at: Option<DateTime<Utc>>,
    pub password_changed_at: Option<DateTime<Utc>>,
    pub failed_login_attempts: i32,
    pub account_locked_until: Option<DateTime<Utc>>,
    pub certification_levels: Json,
    pub profile_image_url: Option<String>,
    pub timezone: Option<String>,
    pub locale: Option<String>,
    pub emergency_contact: Option<Json>,
    pub address: Option<Json>,
    pub employee_id: Option<String>,
    pub cost_center: Option<String>,
    pub security_clearance: Option<String>,
    pub notes: Option<String>,
    pub metadata: Option<Json>,
    pub created_by: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Defines methods for User
impl User {
    /// Creates new User instance
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier
    /// * `username` - Username for login
    /// * `email` - Email address
    /// * `first_name` - First name
    /// * `last_name` - Last name
    /// * `display_name` - Optional display name
    /// * `user_type` - Type of user as string
    /// * `status` - User status as string
    /// * `primary_role_id` - Optional primary role ID
    /// * `department` - Optional department
    /// * `job_title` - Optional job title
    /// * `manager_id` - Optional manager user ID
    /// * `contact_number` - Optional contact number
    /// * `secondary_email` - Optional secondary email
    /// * `hire_date` - Optional hire date
    /// * `certification_levels` - Certification data as JSON
    /// * `profile_image_url` - Optional profile image URL
    /// * `timezone` - Optional timezone
    /// * `locale` - Optional locale
    /// * `emergency_contact` - Optional emergency contact as JSON
    /// * `address` - Optional address as JSON
    /// * `employee_id` - Optional employee ID
    /// * `cost_center` - Optional cost center
    /// * `security_clearance` - Optional security clearance
    /// * `notes` - Optional notes
    /// * `metadata` - Optional metadata as JSON
    /// * `created_by` - Optional creator user ID
    ///
    /// # Returns
    ///
    /// New User instance
    pub fn new(
        id: String,
        username: String,
        email: String,
        first_name: String,
        last_name: String,
        display_name: Option<String>,
        user_type: String,
        status: String,
        primary_role_id: Option<String>,
        department: Option<String>,
        job_title: Option<String>,
        manager_id: Option<String>,
        contact_number: Option<String>,
        secondary_email: Option<String>,
        hire_date: Option<DateTime<Utc>>,
        certification_levels: Json,
        profile_image_url: Option<String>,
        timezone: Option<String>,
        locale: Option<String>,
        emergency_contact: Option<Json>,
        address: Option<Json>,
        employee_id: Option<String>,
        cost_center: Option<String>,
        security_clearance: Option<String>,
        notes: Option<String>,
        metadata: Option<Json>,
        created_by: Option<String>
    ) -> Result<Self, AppError> {
        let now = Utc::now();

        // Validate required fields
        if username.trim().is_empty() {
            return Err(AppError::ValidationError("Username cannot be empty".to_string()));
        }

        if email.trim().is_empty() {
            return Err(AppError::ValidationError("Email cannot be empty".to_string()));
        }

        if first_name.trim().is_empty() {
            return Err(AppError::ValidationError("First name cannot be empty".to_string()));
        }

        if last_name.trim().is_empty() {
            return Err(AppError::ValidationError("Last name cannot be empty".to_string()));
        }

        // Validate email format (basic validation)
        if !email.contains('@') || !email.contains('.') {
            return Err(AppError::ValidationError("Invalid email format".to_string()));
        }

        // Validate secondary email if provided
        if let Some(ref sec_email) = secondary_email {
            if
                !sec_email.trim().is_empty() &&
                (!sec_email.contains('@') || !sec_email.contains('.'))
            {
                return Err(AppError::ValidationError("Invalid secondary email format".to_string()));
            }
        }

        let user_type_enum = UserType::from_string(&user_type)?;
        let status_enum = UserStatus::from_string(&status)?;

        Ok(Self {
            id,
            username,
            email,
            first_name,
            last_name,
            display_name,
            user_type: user_type_enum,
            status: status_enum,
            primary_role_id,
            department,
            job_title,
            manager_id,
            contact_number,
            secondary_email,
            hire_date,
            termination_date: None,
            last_login_at: None,
            password_changed_at: Some(now), // Set to creation time initially
            failed_login_attempts: 0,
            account_locked_until: None,
            certification_levels,
            profile_image_url,
            timezone,
            locale,
            emergency_contact,
            address,
            employee_id,
            cost_center,
            security_clearance,
            notes,
            metadata,
            created_by,
            created_at: now,
            updated_at: now,
        })
    }

    /// Creates User instance from DynamoDB item
    ///
    /// # Arguments
    ///
    /// * `item` - The dynamo db item
    ///
    /// # Returns
    ///
    /// 'Some' User if item fields match, 'None' otherwise
    pub(crate) fn from_item(item: &HashMap<String, AttributeValue>) -> Option<Self> {
        info!("calling from_item with: {:?}", &item);

        let id = item.get("id")?.as_s().ok()?.to_string();
        let username = item.get("username")?.as_s().ok()?.to_string();
        let email = item.get("email")?.as_s().ok()?.to_string();
        let first_name = item.get("first_name")?.as_s().ok()?.to_string();
        let last_name = item.get("last_name")?.as_s().ok()?.to_string();

        let display_name = item
            .get("display_name")
            .and_then(|v| v.as_s().ok())
            .map(|s| s.to_string());

        let user_type_str = item.get("user_type")?.as_s().ok()?;
        let user_type = UserType::from_string(&user_type_str)
            .map_err(|e| e)
            .ok()?;

        let status_str = item.get("status")?.as_s().ok()?;
        let status = UserStatus::from_string(&status_str)
            .map_err(|e| e)
            .ok()?;

        let primary_role_id = item
            .get("primary_role_id")
            .and_then(|v| v.as_s().ok())
            .map(|s| s.to_string());

        let department = item
            .get("department")
            .and_then(|v| v.as_s().ok())
            .map(|s| s.to_string());

        let job_title = item
            .get("job_title")
            .and_then(|v| v.as_s().ok())
            .map(|s| s.to_string());

        let manager_id = item
            .get("manager_id")
            .and_then(|v| v.as_s().ok())
            .map(|s| s.to_string());

        let contact_number = item
            .get("contact_number")
            .and_then(|v| v.as_s().ok())
            .map(|s| s.to_string());

        let secondary_email = item
            .get("secondary_email")
            .and_then(|v| v.as_s().ok())
            .map(|s| s.to_string());

        let hire_date = item
            .get("hire_date")
            .and_then(|v| v.as_s().ok())
            .and_then(|s| s.parse::<DateTime<Utc>>().ok());

        let termination_date = item
            .get("termination_date")
            .and_then(|v| v.as_s().ok())
            .and_then(|s| s.parse::<DateTime<Utc>>().ok());

        let last_login_at = item
            .get("last_login_at")
            .and_then(|v| v.as_s().ok())
            .and_then(|s| s.parse::<DateTime<Utc>>().ok());

        let password_changed_at = item
            .get("password_changed_at")
            .and_then(|v| v.as_s().ok())
            .and_then(|s| s.parse::<DateTime<Utc>>().ok());

        let failed_login_attempts = item
            .get("failed_login_attempts")
            .and_then(|v| v.as_n().ok())
            .and_then(|s| s.parse::<i32>().ok())
            .unwrap_or(0);

        let account_locked_until = item
            .get("account_locked_until")
            .and_then(|v| v.as_s().ok())
            .and_then(|s| s.parse::<DateTime<Utc>>().ok());

        let certification_levels = item
            .get("certification_levels")
            .and_then(|v| v.as_s().ok())
            .and_then(|s| serde_json::from_str::<Json>(s).ok())
            .unwrap_or(Json::Object(serde_json::Map::new()));

        let profile_image_url = item
            .get("profile_image_url")
            .and_then(|v| v.as_s().ok())
            .map(|s| s.to_string());

        let timezone = item
            .get("timezone")
            .and_then(|v| v.as_s().ok())
            .map(|s| s.to_string());

        let locale = item
            .get("locale")
            .and_then(|v| v.as_s().ok())
            .map(|s| s.to_string());

        let emergency_contact = item
            .get("emergency_contact")
            .and_then(|v| v.as_s().ok())
            .and_then(|s| serde_json::from_str::<Json>(s).ok());

        let address = item
            .get("address")
            .and_then(|v| v.as_s().ok())
            .and_then(|s| serde_json::from_str::<Json>(s).ok());

        let employee_id = item
            .get("employee_id")
            .and_then(|v| v.as_s().ok())
            .map(|s| s.to_string());

        let cost_center = item
            .get("cost_center")
            .and_then(|v| v.as_s().ok())
            .map(|s| s.to_string());

        let security_clearance = item
            .get("security_clearance")
            .and_then(|v| v.as_s().ok())
            .map(|s| s.to_string());

        let notes = item
            .get("notes")
            .and_then(|v| v.as_s().ok())
            .map(|s| s.to_string());

        let metadata = item
            .get("metadata")
            .and_then(|v| v.as_s().ok())
            .and_then(|s| serde_json::from_str::<Json>(s).ok());

        let created_by = item
            .get("created_by")
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
            username,
            email,
            first_name,
            last_name,
            display_name,
            user_type,
            status,
            primary_role_id,
            department,
            job_title,
            manager_id,
            contact_number,
            secondary_email,
            hire_date,
            termination_date,
            last_login_at,
            password_changed_at,
            failed_login_attempts,
            account_locked_until,
            certification_levels,
            profile_image_url,
            timezone,
            locale,
            emergency_contact,
            address,
            employee_id,
            cost_center,
            security_clearance,
            notes,
            metadata,
            created_by,
            created_at,
            updated_at,
        });

        info!("result of from_item on user: {:?}", res);
        res
    }

    /// Creates DynamoDB item from User instance
    ///
    /// # Arguments
    ///
    /// * `self` - borrowed instance of self
    ///
    /// # Returns
    ///
    /// HashMap representing DB item for User instance
    pub(crate) fn to_item(&self) -> HashMap<String, AttributeValue> {
        let mut item = HashMap::new();

        item.insert("id".to_string(), AttributeValue::S(self.id.clone()));
        item.insert("username".to_string(), AttributeValue::S(self.username.clone()));
        item.insert("email".to_string(), AttributeValue::S(self.email.clone()));
        item.insert("first_name".to_string(), AttributeValue::S(self.first_name.clone()));
        item.insert("last_name".to_string(), AttributeValue::S(self.last_name.clone()));

        if let Some(display_name) = &self.display_name {
            item.insert("display_name".to_string(), AttributeValue::S(display_name.clone()));
        }

        item.insert(
            "user_type".to_string(),
            AttributeValue::S(self.user_type.to_str().to_string())
        );
        item.insert("status".to_string(), AttributeValue::S(self.status.to_str().to_string()));

        if let Some(role_id) = &self.primary_role_id {
            item.insert("primary_role_id".to_string(), AttributeValue::S(role_id.clone()));
        }

        if let Some(dept) = &self.department {
            item.insert("department".to_string(), AttributeValue::S(dept.clone()));
        }

        if let Some(title) = &self.job_title {
            item.insert("job_title".to_string(), AttributeValue::S(title.clone()));
        }

        if let Some(manager) = &self.manager_id {
            item.insert("manager_id".to_string(), AttributeValue::S(manager.clone()));
        }

        if let Some(contact) = &self.contact_number {
            item.insert("contact_number".to_string(), AttributeValue::S(contact.clone()));
        }

        if let Some(sec_email) = &self.secondary_email {
            item.insert("secondary_email".to_string(), AttributeValue::S(sec_email.clone()));
        }

        if let Some(hire) = &self.hire_date {
            item.insert("hire_date".to_string(), AttributeValue::S(hire.to_string()));
        }

        if let Some(term) = &self.termination_date {
            item.insert("termination_date".to_string(), AttributeValue::S(term.to_string()));
        }

        if let Some(login) = &self.last_login_at {
            item.insert("last_login_at".to_string(), AttributeValue::S(login.to_string()));
        }

        if let Some(pwd_changed) = &self.password_changed_at {
            item.insert(
                "password_changed_at".to_string(),
                AttributeValue::S(pwd_changed.to_string())
            );
        }

        item.insert(
            "failed_login_attempts".to_string(),
            AttributeValue::N(self.failed_login_attempts.to_string())
        );

        if let Some(locked_until) = &self.account_locked_until {
            item.insert(
                "account_locked_until".to_string(),
                AttributeValue::S(locked_until.to_string())
            );
        }

        if let Ok(cert_json) = serde_json::to_string(&self.certification_levels) {
            item.insert("certification_levels".to_string(), AttributeValue::S(cert_json));
        }

        if let Some(image_url) = &self.profile_image_url {
            item.insert("profile_image_url".to_string(), AttributeValue::S(image_url.clone()));
        }

        if let Some(tz) = &self.timezone {
            item.insert("timezone".to_string(), AttributeValue::S(tz.clone()));
        }

        if let Some(loc) = &self.locale {
            item.insert("locale".to_string(), AttributeValue::S(loc.clone()));
        }

        if let Some(emergency) = &self.emergency_contact {
            if let Ok(emergency_json) = serde_json::to_string(emergency) {
                item.insert("emergency_contact".to_string(), AttributeValue::S(emergency_json));
            }
        }

        if let Some(addr) = &self.address {
            if let Ok(addr_json) = serde_json::to_string(addr) {
                item.insert("address".to_string(), AttributeValue::S(addr_json));
            }
        }

        if let Some(emp_id) = &self.employee_id {
            item.insert("employee_id".to_string(), AttributeValue::S(emp_id.clone()));
        }

        if let Some(cost) = &self.cost_center {
            item.insert("cost_center".to_string(), AttributeValue::S(cost.clone()));
        }

        if let Some(clearance) = &self.security_clearance {
            item.insert("security_clearance".to_string(), AttributeValue::S(clearance.clone()));
        }

        if let Some(user_notes) = &self.notes {
            item.insert("notes".to_string(), AttributeValue::S(user_notes.clone()));
        }

        if let Some(meta) = &self.metadata {
            if let Ok(meta_json) = serde_json::to_string(meta) {
                item.insert("metadata".to_string(), AttributeValue::S(meta_json));
            }
        }

        if let Some(creator) = &self.created_by {
            item.insert("created_by".to_string(), AttributeValue::S(creator.clone()));
        }

        item.insert("created_at".to_string(), AttributeValue::S(self.created_at.to_string()));
        item.insert("updated_at".to_string(), AttributeValue::S(self.updated_at.to_string()));

        item
    }

    /// Gets the user's full name
    pub fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    /// Gets the user's display name or falls back to full name
    pub fn effective_display_name(&self) -> String {
        self.display_name.clone().unwrap_or_else(|| self.full_name())
    }

    /// Checks if the user account is currently active
    pub fn is_active(&self) -> bool {
        matches!(self.status, UserStatus::Active) && !self.is_account_locked()
    }

    /// Checks if the user account is locked
    pub fn is_account_locked(&self) -> bool {
        if let Some(locked_until) = &self.account_locked_until {
            Utc::now() < *locked_until
        } else {
            false
        }
    }

    /// Checks if the user is terminated
    pub fn is_terminated(&self) -> bool {
        matches!(self.status, UserStatus::Terminated) || self.termination_date.is_some()
    }

    /// Records a successful login
    pub fn record_login(&mut self) {
        self.last_login_at = Some(Utc::now());
        self.failed_login_attempts = 0;
        self.account_locked_until = None;
        self.updated_at = Utc::now();
    }

    /// Records a failed login attempt
    fn record_failed_login(&mut self, max_attempts: i32, lockout_duration_minutes: i64) {
        self.failed_login_attempts += 1;

        if self.failed_login_attempts >= max_attempts {
            let lockout_duration = chrono::Duration::minutes(lockout_duration_minutes);
            self.account_locked_until = Some(Utc::now() + lockout_duration);
        }

        self.updated_at = Utc::now();
    }

    /// Unlocks the user account
    fn unlock_account(&mut self) {
        self.failed_login_attempts = 0;
        self.account_locked_until = None;
        self.updated_at = Utc::now();
    }

    /// Suspends the user account
    fn suspend(&mut self, reason: Option<String>) -> Result<(), AppError> {
        if matches!(self.status, UserStatus::Terminated) {
            return Err(AppError::ValidationError("Cannot suspend terminated user".to_string()));
        }

        self.status = UserStatus::Suspended;
        if let Some(reason_text) = reason {
            // Add suspension reason to notes
            let current_notes = self.notes.clone().unwrap_or_default();
            self.notes = Some(format!("{}; SUSPENDED: {}", current_notes, reason_text));
        }
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Reactivates a suspended user account
    fn reactivate(&mut self) -> Result<(), AppError> {
        if !matches!(self.status, UserStatus::Suspended | UserStatus::Inactive) {
            return Err(
                AppError::ValidationError(
                    "Only suspended or inactive users can be reactivated".to_string()
                )
            );
        }

        self.status = UserStatus::Active;
        self.failed_login_attempts = 0;
        self.account_locked_until = None;
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Terminates the user account
    fn terminate(&mut self, termination_date: Option<DateTime<Utc>>) -> Result<(), AppError> {
        if matches!(self.status, UserStatus::Terminated) {
            return Err(AppError::ValidationError("User is already terminated".to_string()));
        }

        self.status = UserStatus::Terminated;
        self.termination_date = termination_date.or_else(|| Some(Utc::now()));
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Updates the user's password change timestamp
    fn record_password_change(&mut self) {
        self.password_changed_at = Some(Utc::now());
        self.updated_at = Utc::now();
    }
}
