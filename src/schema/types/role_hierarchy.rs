use crate::models::{ prelude::*, role_hierarchy::{HierarchyType, RoleHierarchy} };
#[Object]
impl RoleHierarchy {
    async fn id(&self) -> &str {
        &self.id
    }

    async fn parent_role_id(&self) -> &str {
        &self.parent_role_id
    }

    async fn child_role_id(&self) -> &str {
        &self.child_role_id
    }

    async fn hierarchy_type(&self) -> HierarchyType {
        self.hierarchy_type
    }

    async fn inherited_permissions(&self) -> bool {
        self.inherited_permissions
    }

    async fn permission_overrides(&self) -> &Vec<String> {
        &self.permission_overrides
    }

    async fn depth_level(&self) -> i32 {
        self.depth_level
    }

    async fn active(&self) -> bool {
        self.active
    }

    async fn priority(&self) -> i32 {
        self.priority
    }

    async fn conditions(&self) -> Option<&str> {
        self.conditions.as_deref()
    }

    async fn delegation_expires_at(&self) -> Option<&DateTime<Utc>> {
        self.delegation_expires_at.as_ref()
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

    // #[graphql(name = "is_effective")]
    // async fn check_is_effective(&self) -> bool {
    //     self.is_effective()
    // }
    // #[graphql(name = "has_permission_override")]
    // async fn check_has_permission_override(&self, permission_id: String) -> bool {
    //     self.has_permission_override(&permission_id)
    // }

    // async fn override_count(&self) -> i32 {
    //     self.permission_overrides.len() as i32
    // }
}
