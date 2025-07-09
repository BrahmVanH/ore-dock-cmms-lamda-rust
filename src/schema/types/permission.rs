use crate::models::{ permission::{Permission, PermissionScope}, permission_log::{PermissionAction, ResourceType}, prelude::* };
#[Object]
impl Permission {
    async fn id(&self) -> &str {
        &self.id
    }

    async fn role_id(&self) -> &str {
        &self.role_id
    }

    async fn resource_type(&self) -> ResourceType {
        self.resource_type
    }

    async fn actions(&self) -> Vec<PermissionAction> {
        self.actions
            .iter()
            .map(|a| a.clone())
            .collect()
    }

    async fn scope(&self) -> PermissionScope {
        self.scope
    }

    async fn conditions(&self) -> Option<String> {
        self.conditions.as_ref().and_then(|c| serde_json::to_string(c).ok())
    }

    async fn resource_filters(&self) -> Option<String> {
        self.resource_filters.as_ref().and_then(|f| serde_json::to_string(f).ok())
    }

    async fn active(&self) -> bool {
        self.active
    }

    async fn expires_at(&self) -> Option<&DateTime<Utc>> {
        self.expires_at.as_ref()
    }

    async fn created_by(&self) -> &str {
        &self.created_by
    }

    async fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }

    async fn updated_at(&self) -> &DateTime<Utc> {
        &self.updated_at
    }
  // Domain entity methods - expose if needed
    // #[graphql(name = "is_expired")]
    // async fn get_is_expired(&self) -> bool {
    //     self.is_expired()
    // }

    // #[graphql(name = "allows_action")]
    // async fn check_allows_action(&self, action: String) -> bool {
    //     if let Ok(action_enum) = PermissionAction::from_string(&action) {
    //         self.allows_action(&action_enum)
    //     } else {
    //         false
    //     }
    // }
}
