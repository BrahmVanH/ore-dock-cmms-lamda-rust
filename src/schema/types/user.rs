use async_graphql::Object;
use super::User;

#[Object]
impl User {
    async fn id(&self) -> &str {
        &self.id
    }

    async fn username(&self) -> &str {
        &self.username
    }

    async fn email(&self) -> &str {
        &self.email
    }

    async fn first_name(&self) -> &str {
        &self.first_name
    }

    async fn last_name(&self) -> &str {
        &self.last_name
    }

    async fn display_name(&self) -> Option<&str> {
        self.display_name.as_deref()
    }

    async fn user_type(&self) -> &str {
        self.user_type.to_str()
    }

    async fn status(&self) -> &str {
        self.status.to_str()
    }

    async fn primary_role_id(&self) -> Option<&str> {
        self.primary_role_id.as_deref()
    }

    async fn department(&self) -> Option<&str> {
        self.department.as_deref()
    }

    async fn job_title(&self) -> Option<&str> {
        self.job_title.as_deref()
    }

    async fn manager_id(&self) -> Option<&str> {
        self.manager_id.as_deref()
    }

    async fn contact_number(&self) -> Option<&str> {
        self.contact_number.as_deref()
    }

    async fn secondary_email(&self) -> Option<&str> {
        self.secondary_email.as_deref()
    }

    async fn hire_date(&self) -> Option<&DateTime<Utc>> {
        self.hire_date.as_ref()
    }

    async fn termination_date(&self) -> Option<&DateTime<Utc>> {
        self.termination_date.as_ref()
    }

    async fn last_login_at(&self) -> Option<&DateTime<Utc>> {
        self.last_login_at.as_ref()
    }

    async fn password_changed_at(&self) -> Option<&DateTime<Utc>> {
        self.password_changed_at.as_ref()
    }

    async fn failed_login_attempts(&self) -> i32 {
        self.failed_login_attempts
    }

    async fn account_locked_until(&self) -> Option<&DateTime<Utc>> {
        self.account_locked_until.as_ref()
    }

    async fn certification_levels(&self) -> String {
        serde_json::to_string(&self.certification_levels).unwrap_or_default()
    }

    async fn profile_image_url(&self) -> Option<&str> {
        self.profile_image_url.as_deref()
    }

    async fn timezone(&self) -> Option<&str> {
        self.timezone.as_deref()
    }

    async fn locale(&self) -> Option<&str> {
        self.locale.as_deref()
    }

    async fn emergency_contact(&self) -> Option<String> {
        self.emergency_contact.as_ref().and_then(|ec| serde_json::to_string(ec).ok())
    }

    async fn address(&self) -> Option<String> {
        self.address.as_ref().and_then(|addr| serde_json::to_string(addr).ok())
    }

    async fn employee_id(&self) -> Option<&str> {
        self.employee_id.as_deref()
    }

    async fn cost_center(&self) -> Option<&str> {
        self.cost_center.as_deref()
    }

    async fn security_clearance(&self) -> Option<&str> {
        self.security_clearance.as_deref()
    }

    async fn notes(&self) -> Option<&str> {
        self.notes.as_deref()
    }

    async fn metadata(&self) -> Option<String> {
        self.metadata.as_ref().and_then(|meta| serde_json::to_string(meta).ok())
    }

    async fn created_by(&self) -> Option<&str> {
        self.created_by.as_deref()
    }

    async fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }

    async fn updated_at(&self) -> &DateTime<Utc> {
        &self.updated_at
    }
    #[graphql(name = "full_name")]
    async fn check_full_name(&self) -> String {
        self.full_name()
    }
    #[graphql(name = "effective_display_name")]
    async fn check_effective_display_name(&self) -> String {
        self.effective_display_name()
    }
    #[graphql(name = "is_active")]
    async fn check_is_active(&self) -> bool {
        self.is_active()
    }
    #[graphql(name = "is_notification_allowed_now")]
    async fn check_is_account_locked(&self) -> bool {
        self.is_account_locked()
    }
    #[graphql(name = "is_terminated")]
    async fn check_is_terminated(&self) -> bool {
        self.is_terminated()
    }

    async fn days_since_last_login(&self) -> Option<i64> {
        self.last_login_at.map(|last_login| {
            let duration = Utc::now() - last_login;
            duration.num_days()
        })
    }

    async fn days_since_password_change(&self) -> Option<i64> {
        self.password_changed_at.map(|pwd_change| {
            let duration = Utc::now() - pwd_change;
            duration.num_days()
        })
    }

    async fn has_secondary_email(&self) -> bool {
        self.secondary_email.is_some() && !self.secondary_email.as_ref().unwrap().trim().is_empty()
    }

    async fn is_employee(&self) -> bool {
        matches!(self.user_type, UserType::Employee)
    }

    async fn is_contractor(&self) -> bool {
        matches!(self.user_type, UserType::Contractor)
    }

    async fn has_manager(&self) -> bool {
        self.manager_id.is_some()
    }
}
