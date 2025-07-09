use crate::models::{
    permission_log::{ PermissionAction, PermissionLog, PermissionStatus, ResourceType },
    prelude::*,
};
#[Object]
impl PermissionLog {
    async fn id(&self) -> &str {
        &self.id
    }

    async fn user_id(&self) -> &str {
        &self.user_id
    }

    async fn resource_type(&self) -> ResourceType {
        self.resource_type
    }

    async fn resource_id(&self) -> &str {
        &self.resource_id
    }

    async fn action(&self) -> PermissionAction {
        self.action
    }

    async fn status(&self) -> PermissionStatus {
        self.status
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
