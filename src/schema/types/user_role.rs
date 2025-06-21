use crate::models::{ prelude::*, user_role::UserRole };
#[Object]
impl UserRole {
    async fn id(&self) -> &str {
        &self.id
    }

    async fn user_id(&self) -> &str {
        &self.user_id
    }

    async fn role_id(&self) -> &str {
        &self.role_id
    }

    async fn assignment_source(&self) -> &str {
        self.assignment_source.to_str()
    }

    async fn status(&self) -> &str {
        self.status.to_str()
    }

    async fn is_primary_role(&self) -> bool {
        self.is_primary_role
    }

    async fn assigned_at(&self) -> &DateTime<Utc> {
        &self.assigned_at
    }

    async fn assigned_by_user_id(&self) -> Option<&str> {
        self.assigned_by_user_id.as_deref()
    }

    async fn effective_from(&self) -> &DateTime<Utc> {
        &self.effective_from
    }

    async fn expires_at(&self) -> Option<&DateTime<Utc>> {
        self.expires_at.as_ref()
    }

    async fn last_used_at(&self) -> Option<&DateTime<Utc>> {
        self.last_used_at.as_ref()
    }

    async fn conditions(&self) -> Option<&str> {
        self.conditions.as_deref()
    }

    async fn elevation_request_id(&self) -> Option<&str> {
        self.elevation_request_id.as_deref()
    }

    async fn revoked_at(&self) -> Option<&DateTime<Utc>> {
        self.revoked_at.as_ref()
    }

    async fn revoked_by_user_id(&self) -> Option<&str> {
        self.revoked_by_user_id.as_deref()
    }

    async fn revocation_reason(&self) -> Option<&str> {
        self.revocation_reason.as_deref()
    }

    async fn metadata(&self) -> Option<&str> {
        self.metadata.as_deref()
    }

    async fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }

    async fn updated_at(&self) -> &DateTime<Utc> {
        &self.updated_at
    }
    // Domain entity methods - expose if needed

    // #[graphql(name = "is_effective")]
    // async fn check_is_effective(&self) -> bool {
    //     self.is_effective()
    // }

    // async fn is_temporary(&self) -> bool {
    //     self.expires_at.is_some()
    // }

    // async fn is_from_elevation(&self) -> bool {
    //     self.elevation_request_id.is_some()
    // }

    // async fn days_until_expiration(&self) -> Option<i64> {
    //     if let Some(expires_at) = &self.expires_at {
    //         let duration = *expires_at - Utc::now();
    //         Some(duration.num_days().max(0))
    //     } else {
    //         None
    //     }
    // }

    // async fn days_since_last_used(&self) -> Option<i64> {
    //     if let Some(last_used) = &self.last_used_at {
    //         let duration = Utc::now() - *last_used;
    //         Some(duration.num_days())
    //     } else {
    //         None
    //     }
    // }
}
