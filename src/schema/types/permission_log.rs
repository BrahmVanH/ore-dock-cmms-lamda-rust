use crate::models::{ prelude::*, permission_log::PermissionLog };
#[Object]
impl PermissionLog {
    async fn id(&self) -> &str {
        &self.id
    }

    async fn user_id(&self) -> &str {
        &self.user_id
    }

    async fn resource_type(&self) -> &str {
        self.resource_type.to_str()
    }

    async fn resource_id(&self) -> &str {
        &self.resource_id
    }

    async fn action(&self) -> &str {
        self.action.to_str()
    }

    async fn status(&self) -> &str {
        self.status.to_str()
    }

    async fn attempted_at(&self) -> &DateTime<Utc> {
        &self.attempted_at
    }

    async fn granted_at(&self) -> Option<&DateTime<Utc>> {
        self.granted_at.as_ref()
    }

    async fn denied_reason(&self) -> Option<&str> {
        self.denied_reason.as_deref()
    }

    async fn ip_address(&self) -> &str {
        &self.ip_address
    }

    async fn user_agent(&self) -> &str {
        &self.user_agent
    }

    async fn session_id(&self) -> Option<&str> {
        self.session_id.as_deref()
    }

    async fn role_at_time(&self) -> Option<&str> {
        self.role_at_time.as_deref()
    }

    async fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }

    async fn updated_at(&self) -> &DateTime<Utc> {
        &self.updated_at
    }
}
