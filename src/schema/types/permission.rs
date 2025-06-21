use crate::models::{ prelude::*, permission::Permission };
#[Object]
impl Permission {
    async fn id(&self) -> &str {
        &self.id
    }

    async fn role_id(&self) -> &str {
        &self.role_id
    }

    async fn resource_type(&self) -> &str {
        self.resource_type.to_str()
    }

    async fn actions(&self) -> Vec<String> {
        self.actions
            .iter()
            .map(|a| a.to_str().to_string())
            .collect()
    }

    async fn scope(&self) -> &str {
        self.scope.to_str()
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
