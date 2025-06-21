use crate::models::{ prelude::*, role::Role };

#[Object]
impl Role {
    async fn id(&self) -> &str {
        &self.id
    }

    async fn name(&self) -> &str {
        &self.name
    }

    async fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    async fn role_type(&self) -> &str {
        self.role_type.to_str()
    }

    async fn is_system_role(&self) -> bool {
        self.is_system_role
    }

    async fn permission_ids(&self) -> &Vec<String> {
        &self.permission_ids
    }

    async fn parent_role_id(&self) -> Option<&str> {
        self.parent_role_id.as_deref()
    }

    async fn priority(&self) -> i32 {
        self.priority
    }

    async fn active(&self) -> bool {
        self.active
    }

    async fn expires_at(&self) -> Option<&DateTime<Utc>> {
        self.expires_at.as_ref()
    }

    async fn max_users(&self) -> Option<i32> {
        self.max_users
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
    // Domain entity methods - expose if needed

    // #[graphql(name = "is_expired")]
    // async fn get_is_expired(&self) -> bool {
    //     self.is_expired()
    // }

    // #[graphql(name = "is_usable")]
    // async fn get_is_usable(&self) -> bool {
    //     self.is_usable()
    // }

    // #[graphql(name = "has_permission")]
    // async fn check_permission(&self, permission_id: String) -> bool {
    //     self.has_permission(&permission_id)
    // }

    // async fn permission_count(&self) -> i32 {
    //     self.permission_ids.len() as i32
    // }
}
