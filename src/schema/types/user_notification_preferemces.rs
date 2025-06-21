use crate::models::{
    notification::{ NotificationChannels, SeverityLevel },
    prelude::*,
    user_notification_preferences::UserNotificationPreferences,
};
#[Object]
impl UserNotificationPreferences {
    async fn id(&self) -> &str {
        &self.id
    }

    async fn user_id(&self) -> &str {
        &self.user_id
    }

    async fn scope(&self) -> &str {
        self.scope.to_str()
    }

    async fn scope_value(&self) -> Option<&str> {
        self.scope_value.as_deref()
    }

    async fn enabled(&self) -> bool {
        self.enabled
    }

    async fn preferred_channels(&self) -> Vec<String> {
        self.preferred_channels
            .iter()
            .map(|c| c.to_str().to_string())
            .collect()
    }

    async fn blocked_channels(&self) -> Vec<String> {
        self.blocked_channels
            .iter()
            .map(|c| c.to_str().to_string())
            .collect()
    }

    async fn do_not_disturb(&self) -> bool {
        self.do_not_disturb
    }

    async fn quiet_hours_start(&self) -> Option<String> {
        self.quiet_hours_start.map(|t| t.format("%H:%M:%S").to_string())
    }

    async fn quiet_hours_end(&self) -> Option<String> {
        self.quiet_hours_end.map(|t| t.format("%H:%M:%S").to_string())
    }

    async fn quiet_hours_timezone(&self) -> Option<&str> {
        self.quiet_hours_timezone.as_deref()
    }

    async fn min_severity_level(&self) -> Option<String> {
        self.min_severity_level.as_ref().map(|s| s.to_str().to_string())
    }

    async fn frequency_limit(&self) -> i32 {
        self.frequency_limit
    }

    async fn digest_enabled(&self) -> bool {
        self.digest_enabled
    }

    async fn digest_frequency_hours(&self) -> i32 {
        self.digest_frequency_hours
    }

    async fn escalation_enabled(&self) -> bool {
        self.escalation_enabled
    }

    async fn escalation_delay_minutes(&self) -> i32 {
        self.escalation_delay_minutes
    }

    async fn active(&self) -> bool {
        self.active
    }

    async fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }

    async fn updated_at(&self) -> &DateTime<Utc> {
        &self.updated_at
    }

    #[graphql(name = "is_notification_allowed_now")]
    async fn check_is_notification_allowed_now(&self) -> bool {
        self.is_notification_allowed_now()
    }

    // Domain entity methods - expose if needed
    // #[graphql(name = "is_channel_allowed")]
    // async fn check_is_channel_allowed(&self, channel: String) -> bool {
    //     if let Ok(channel_enum) = NotificationChannels::from_string(&channel) {
    //         self.is_channel_allowed(&channel_enum)
    //     } else {
    //         false
    //     }
    // }

    // #[graphql(name = "is_severity_allowed")]
    // async fn check_is_severity_allowed(&self, severity: String) -> bool {
    //     if let Ok(severity_enum) = SeverityLevel::from_string(&severity) {
    //         self.is_severity_allowed(&severity_enum)
    //     } else {
    //         false
    //     }
    // }
}
