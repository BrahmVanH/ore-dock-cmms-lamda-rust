use crate::models::{ prelude::*, temp_role_elevation::TempRoleElevation };

#[Object]
impl TempRoleElevation {
    async fn id(&self) -> &str {
        &self.id
    }

    async fn user_id(&self) -> &str {
        &self.user_id
    }

    async fn original_role_id(&self) -> &str {
        &self.original_role_id
    }

    async fn elevated_role_id(&self) -> &str {
        &self.elevated_role_id
    }

    async fn reason(&self) -> Option<&str> {
        self.reason.as_deref()
    }

    async fn justification(&self) -> Option<&str> {
        self.justification.as_deref()
    }

    async fn requested_by_user_id(&self) -> &str {
        &self.requested_by_user_id
    }

    async fn approved_by_user_id(&self) -> Option<&str> {
        self.approved_by_user_id.as_deref()
    }

    async fn start_time(&self) -> &DateTime<Utc> {
        &self.start_time
    }

    async fn end_time(&self) -> &DateTime<Utc> {
        &self.end_time
    }

    async fn actual_start_time(&self) -> Option<&DateTime<Utc>> {
        self.actual_start_time.as_ref()
    }

    async fn actual_end_time(&self) -> Option<&DateTime<Utc>> {
        self.actual_end_time.as_ref()
    }

    async fn status(&self) -> &str {
        self.status.to_str()
    }

    async fn priority(&self) -> &str {
        self.priority.to_str()
    }

    async fn auto_revoke(&self) -> bool {
        self.auto_revoke
    }

    async fn notification_sent(&self) -> bool {
        self.notification_sent
    }

    async fn approval_required(&self) -> bool {
        self.approval_required
    }

    async fn approval_deadline(&self) -> Option<&DateTime<Utc>> {
        self.approval_deadline.as_ref()
    }

    async fn revoked_by_user_id(&self) -> Option<&str> {
        self.revoked_by_user_id.as_deref()
    }

    async fn revocation_reason(&self) -> Option<&str> {
        self.revocation_reason.as_deref()
    }

    async fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }

    async fn updated_at(&self) -> &DateTime<Utc> {
        &self.updated_at
    }
    // Domain entity methods - expose if needed
    // #[graphql(name = "is_active")]
    // async fn check_is_active(&self) -> bool {
    //     self.is_active()
    // }

    // #[graphql(name = "is_expired")]
    // async fn get_is_expired(&self) -> bool {
    //     self.is_expired()
    // }

    // #[graphql(name = "is_pending")]
    // async fn check_is_pending(&self) -> bool {
    //     self.is_pending()
    // }

    // async fn duration_minutes(&self) -> i64 {
    //     (self.end_time - self.start_time).num_minutes()
    // }

    // async fn time_remaining_minutes(&self) -> Option<i64> {
    //     if self.is_active() {
    //         let remaining = self.end_time - Utc::now();
    //         Some(remaining.num_minutes().max(0))
    //     } else {
    //         None
    //     }
    // }
}
