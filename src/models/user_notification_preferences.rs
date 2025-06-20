use std::collections::HashMap;

use async_graphql::Object;
use aws_sdk_dynamodb::types::AttributeValue;
use chrono::{ DateTime, Utc, NaiveTime };
use serde::{ Deserialize, Serialize };
use tracing::info;

use crate::{
    error::AppError,
    models::{
        notification::{ NotificationChannels, SeverityLevel },
        notification_template::NotificationType,
    },
};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PreferenceScope {
    Global, // Apply to all notifications
    Template, // Apply to specific template
    Type, // Apply to specific notification type
    Severity, // Apply to specific severity level
}

impl PreferenceScope {
    fn to_str(&self) -> &str {
        match self {
            PreferenceScope::Global => "global",
            PreferenceScope::Template => "template",
            PreferenceScope::Type => "type",
            PreferenceScope::Severity => "severity",
        }
    }

    fn to_string(&self) -> String {
        self.to_str().to_string()
    }

    fn from_string(s: &str) -> Result<PreferenceScope, AppError> {
        match s {
            "global" => Ok(Self::Global),
            "template" => Ok(Self::Template),
            "type" => Ok(Self::Type),
            "severity" => Ok(Self::Severity),
            _ => Err(AppError::ValidationError("Invalid preference scope".to_string())),
        }
    }
}

/// Represents User Notification Preferences in the system
///
/// # Fields
///
/// * `id` - Unique identifier for the preference entry
/// * `user_id` - ID of the user these preferences belong to
/// * `scope` - Scope of the preference (global, template, type, severity)
/// * `scope_value` - Value for scoped preferences (template_id, type, severity level)
/// * `enabled` - Whether notifications are enabled for this scope
/// * `preferred_channels` - List of preferred notification channels
/// * `blocked_channels` - List of blocked notification channels
/// * `do_not_disturb` - Whether user is in do not disturb mode
/// * `quiet_hours_start` - Start time for quiet hours (24-hour format)
/// * `quiet_hours_end` - End time for quiet hours (24-hour format)
/// * `quiet_hours_timezone` - Timezone for quiet hours
/// * `min_severity_level` - Minimum severity level to receive notifications
/// * `frequency_limit` - Maximum notifications per hour (0 = unlimited)
/// * `digest_enabled` - Whether to enable digest mode for low priority notifications
/// * `digest_frequency_hours` - How often to send digest (in hours)
/// * `escalation_enabled` - Whether to enable escalation for critical notifications
/// * `escalation_delay_minutes` - Minutes to wait before escalation
/// * `active` - Whether this preference is currently active
/// * `created_at` - Date and time of creation
/// * `updated_at` - Date and time of last update
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserNotificationPreferences {
    pub id: String,
    pub user_id: String,
    pub scope: PreferenceScope,
    pub scope_value: Option<String>,
    pub enabled: bool,
    pub preferred_channels: Vec<NotificationChannels>,
    pub blocked_channels: Vec<NotificationChannels>,
    pub do_not_disturb: bool,
    pub quiet_hours_start: Option<NaiveTime>,
    pub quiet_hours_end: Option<NaiveTime>,
    pub quiet_hours_timezone: Option<String>,
    pub min_severity_level: Option<SeverityLevel>,
    pub frequency_limit: i32,
    pub digest_enabled: bool,
    pub digest_frequency_hours: i32,
    pub escalation_enabled: bool,
    pub escalation_delay_minutes: i32,
    pub active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Defines methods for UserNotificationPreferences
impl UserNotificationPreferences {
    /// Creates new UserNotificationPreferences instance
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier
    /// * `user_id` - User ID these preferences belong to
    /// * `scope` - Preference scope as string
    /// * `scope_value` - Optional scope value
    /// * `enabled` - Whether notifications are enabled
    /// * `preferred_channels` - List of preferred channels as strings
    /// * `blocked_channels` - List of blocked channels as strings
    /// * `do_not_disturb` - Do not disturb mode
    /// * `quiet_hours_start` - Optional quiet hours start time
    /// * `quiet_hours_end` - Optional quiet hours end time
    /// * `quiet_hours_timezone` - Optional timezone
    /// * `min_severity_level` - Optional minimum severity as string
    /// * `frequency_limit` - Frequency limit per hour
    /// * `digest_enabled` - Whether digest is enabled
    /// * `digest_frequency_hours` - Digest frequency in hours
    /// * `escalation_enabled` - Whether escalation is enabled
    /// * `escalation_delay_minutes` - Escalation delay in minutes
    /// * `active` - Whether preference is active
    ///
    /// # Returns
    ///
    /// New UserNotificationPreferences instance
    pub fn new(
        id: String,
        user_id: String,
        scope: String,
        scope_value: Option<String>,
        enabled: bool,
        preferred_channels: Vec<String>,
        blocked_channels: Vec<String>,
        do_not_disturb: bool,
        quiet_hours_start: Option<NaiveTime>,
        quiet_hours_end: Option<NaiveTime>,
        quiet_hours_timezone: Option<String>,
        min_severity_level: Option<String>,
        frequency_limit: i32,
        digest_enabled: bool,
        digest_frequency_hours: i32,
        escalation_enabled: bool,
        escalation_delay_minutes: i32,
        active: bool
    ) -> Result<Self, AppError> {
        let now = Utc::now();

        if user_id.trim().is_empty() {
            return Err(AppError::ValidationError("User ID cannot be empty".to_string()));
        }

        let scope_enum = PreferenceScope::from_string(&scope)?;

        // Convert channel strings to enums
        let preferred_channel_enums: Result<
            Vec<NotificationChannels>,
            AppError
        > = preferred_channels
            .iter()
            .map(|c| NotificationChannels::from_string(c))
            .collect();
        let preferred_channel_enums = preferred_channel_enums?;

        let blocked_channel_enums: Result<Vec<NotificationChannels>, AppError> = blocked_channels
            .iter()
            .map(|c| NotificationChannels::from_string(c))
            .collect();
        let blocked_channel_enums = blocked_channel_enums?;

        let min_severity_enum = if let Some(severity_str) = min_severity_level {
            Some(SeverityLevel::from_string(&severity_str)?)
        } else {
            None
        };

        Ok(Self {
            id,
            user_id,
            scope: scope_enum,
            scope_value,
            enabled,
            preferred_channels: preferred_channel_enums,
            blocked_channels: blocked_channel_enums,
            do_not_disturb,
            quiet_hours_start,
            quiet_hours_end,
            quiet_hours_timezone,
            min_severity_level: min_severity_enum,
            frequency_limit,
            digest_enabled,
            digest_frequency_hours,
            escalation_enabled,
            escalation_delay_minutes,
            active,
            created_at: now,
            updated_at: now,
        })
    }

    /// Creates UserNotificationPreferences instance from DynamoDB item
    ///
    /// # Arguments
    ///
    /// * `item` - The dynamo db item
    ///
    /// # Returns
    ///
    /// 'Some' UserNotificationPreferences if item fields match, 'None' otherwise
    pub fn from_item(item: &HashMap<String, AttributeValue>) -> Option<Self> {
        info!("calling from_item with: {:?}", &item);

        let id = item.get("id")?.as_s().ok()?.to_string();
        let user_id = item.get("user_id")?.as_s().ok()?.to_string();

        let scope_str = item.get("scope")?.as_s().ok()?;
        let scope = PreferenceScope::from_string(&scope_str)
            .map_err(|e| e)
            .ok()?;

        let scope_value = item
            .get("scope_value")
            .and_then(|v| v.as_s().ok())
            .map(|s| s.to_string());

        let enabled = item
            .get("enabled")
            .and_then(|v| v.as_bool().ok())
            .unwrap_or(&true);

        let preferred_channels = item
            .get("preferred_channels")
            .and_then(|v| v.as_ss().ok())
            .map(|channel_strs| {
                channel_strs
                    .iter()
                    .filter_map(|c| NotificationChannels::from_string(c).ok())
                    .collect::<Vec<NotificationChannels>>()
            })
            .unwrap_or_default();

        let blocked_channels = item
            .get("blocked_channels")
            .and_then(|v| v.as_ss().ok())
            .map(|channel_strs| {
                channel_strs
                    .iter()
                    .filter_map(|c| NotificationChannels::from_string(c).ok())
                    .collect::<Vec<NotificationChannels>>()
            })
            .unwrap_or_default();

        let do_not_disturb = item
            .get("do_not_disturb")
            .and_then(|v| v.as_bool().ok())
            .unwrap_or(&false);

        let quiet_hours_start = item
            .get("quiet_hours_start")
            .and_then(|v| v.as_s().ok())
            .and_then(|s| NaiveTime::parse_from_str(s, "%H:%M:%S").ok());

        let quiet_hours_end = item
            .get("quiet_hours_end")
            .and_then(|v| v.as_s().ok())
            .and_then(|s| NaiveTime::parse_from_str(s, "%H:%M:%S").ok());

        let quiet_hours_timezone = item
            .get("quiet_hours_timezone")
            .and_then(|v| v.as_s().ok())
            .map(|s| s.to_string());

        let min_severity_level = item
            .get("min_severity_level")
            .and_then(|v| v.as_s().ok())
            .and_then(|s| SeverityLevel::from_string(s).ok());

        let frequency_limit = item
            .get("frequency_limit")
            .and_then(|v| v.as_n().ok())
            .and_then(|s| s.parse::<i32>().ok())
            .unwrap_or(0);

        let digest_enabled = item
            .get("digest_enabled")
            .and_then(|v| v.as_bool().ok())
            .unwrap_or(&false);

        let digest_frequency_hours = item
            .get("digest_frequency_hours")
            .and_then(|v| v.as_n().ok())
            .and_then(|s| s.parse::<i32>().ok())
            .unwrap_or(24);

        let escalation_enabled = item
            .get("escalation_enabled")
            .and_then(|v| v.as_bool().ok())
            .unwrap_or(&false);

        let escalation_delay_minutes = item
            .get("escalation_delay_minutes")
            .and_then(|v| v.as_n().ok())
            .and_then(|s| s.parse::<i32>().ok())
            .unwrap_or(15);

        let active = item
            .get("active")
            .and_then(|v| v.as_bool().ok())
            .unwrap_or(&true);

        let created_at = item
            .get("created_at")
            .and_then(|v| v.as_s().ok())
            .and_then(|s| s.parse::<DateTime<Utc>>().ok())
            .unwrap_or_else(|| Utc::now());

        let updated_at = item
            .get("updated_at")
            .and_then(|v| v.as_s().ok())
            .and_then(|s| s.parse::<DateTime<Utc>>().ok())
            .unwrap_or_else(|| Utc::now());

        let res = Some(Self {
            id,
            user_id,
            scope,
            scope_value,
            enabled,
            preferred_channels,
            blocked_channels,
            do_not_disturb,
            quiet_hours_start,
            quiet_hours_end,
            quiet_hours_timezone,
            min_severity_level,
            frequency_limit,
            digest_enabled,
            digest_frequency_hours,
            escalation_enabled,
            escalation_delay_minutes,
            active,
            created_at,
            updated_at,
        });

        info!("result of from_item on user_notification_preferences: {:?}", res);
        res
    }

    /// Creates DynamoDB item from UserNotificationPreferences instance
    ///
    /// # Arguments
    ///
    /// * `self` - borrowed instance of self
    ///
    /// # Returns
    ///
    /// HashMap representing DB item for UserNotificationPreferences instance
    pub fn to_item(&self) -> HashMap<String, AttributeValue> {
        let mut item = HashMap::new();

        item.insert("id".to_string(), AttributeValue::S(self.id.clone()));
        item.insert("user_id".to_string(), AttributeValue::S(self.user_id.clone()));
        item.insert("scope".to_string(), AttributeValue::S(self.scope.to_str().to_string()));

        if let Some(scope_val) = &self.scope_value {
            item.insert("scope_value".to_string(), AttributeValue::S(scope_val.clone()));
        }

        item.insert("enabled".to_string(), AttributeValue::Bool(self.enabled));

        // Convert preferred channels to string set
        if !self.preferred_channels.is_empty() {
            let preferred_strings: Vec<String> = self.preferred_channels
                .iter()
                .map(|c| c.to_str().to_string())
                .collect();
            item.insert("preferred_channels".to_string(), AttributeValue::Ss(preferred_strings));
        }

        // Convert blocked channels to string set
        if !self.blocked_channels.is_empty() {
            let blocked_strings: Vec<String> = self.blocked_channels
                .iter()
                .map(|c| c.to_str().to_string())
                .collect();
            item.insert("blocked_channels".to_string(), AttributeValue::Ss(blocked_strings));
        }

        item.insert("do_not_disturb".to_string(), AttributeValue::Bool(self.do_not_disturb));

        if let Some(start_time) = &self.quiet_hours_start {
            item.insert(
                "quiet_hours_start".to_string(),
                AttributeValue::S(start_time.format("%H:%M:%S").to_string())
            );
        }

        if let Some(end_time) = &self.quiet_hours_end {
            item.insert(
                "quiet_hours_end".to_string(),
                AttributeValue::S(end_time.format("%H:%M:%S").to_string())
            );
        }

        if let Some(timezone) = &self.quiet_hours_timezone {
            item.insert("quiet_hours_timezone".to_string(), AttributeValue::S(timezone.clone()));
        }

        if let Some(min_severity) = &self.min_severity_level {
            item.insert(
                "min_severity_level".to_string(),
                AttributeValue::S(min_severity.to_str().to_string())
            );
        }

        item.insert(
            "frequency_limit".to_string(),
            AttributeValue::N(self.frequency_limit.to_string())
        );
        item.insert("digest_enabled".to_string(), AttributeValue::Bool(self.digest_enabled));
        item.insert(
            "digest_frequency_hours".to_string(),
            AttributeValue::N(self.digest_frequency_hours.to_string())
        );
        item.insert(
            "escalation_enabled".to_string(),
            AttributeValue::Bool(self.escalation_enabled)
        );
        item.insert(
            "escalation_delay_minutes".to_string(),
            AttributeValue::N(self.escalation_delay_minutes.to_string())
        );
        item.insert("active".to_string(), AttributeValue::Bool(self.active));
        item.insert("created_at".to_string(), AttributeValue::S(self.created_at.to_string()));
        item.insert("updated_at".to_string(), AttributeValue::S(self.updated_at.to_string()));

        item
    }

    /// Checks if notifications are allowed at the current time
    pub fn is_notification_allowed_now(&self) -> bool {
        if !self.enabled || !self.active {
            return false;
        }

        if self.do_not_disturb {
            return false;
        }

        // Check quiet hours
        if let (Some(start), Some(end)) = (&self.quiet_hours_start, &self.quiet_hours_end) {
            let now = Utc::now().time();

            // Handle quiet hours that span midnight
            if start <= end {
                // Same day quiet hours (e.g., 22:00 to 06:00)
                if now >= *start && now <= *end {
                    return false;
                }
            } else {
                // Cross-midnight quiet hours (e.g., 22:00 to 06:00)
                if now >= *start || now <= *end {
                    return false;
                }
            }
        }

        true
    }

    /// Checks if a specific channel is allowed
    pub fn is_channel_allowed(&self, channel: &NotificationChannels) -> bool {
        if self.blocked_channels.contains(channel) {
            return false;
        }

        // If preferred channels are specified, only allow those
        if !self.preferred_channels.is_empty() {
            return self.preferred_channels.contains(channel);
        }

        true
    }

    /// Checks if a severity level meets the minimum threshold
    pub fn is_severity_allowed(&self, severity: &SeverityLevel) -> bool {
        if let Some(min_severity) = &self.min_severity_level {
            // Define severity hierarchy (higher number = more severe)
            let severity_level = match severity {
                SeverityLevel::Informational => 1,
                SeverityLevel::Low => 2,
                SeverityLevel::Medium => 3,
                SeverityLevel::High => 4,
                SeverityLevel::Critical => 5,
            };

            let min_level = match min_severity {
                SeverityLevel::Informational => 1,
                SeverityLevel::Low => 2,
                SeverityLevel::Medium => 3,
                SeverityLevel::High => 4,
                SeverityLevel::Critical => 5,
            };

            severity_level >= min_level
        } else {
            true
        }
    }
}

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

    #[graphql(name = "is_channel_allowed")]
    async fn check_is_channel_allowed(&self, channel: String) -> bool {
        if let Ok(channel_enum) = NotificationChannels::from_string(&channel) {
            self.is_channel_allowed(&channel_enum)
        } else {
            false
        }
    }

    #[graphql(name = "is_severity_allowed")]
    async fn check_is_severity_allowed(&self, severity: String) -> bool {
        if let Ok(severity_enum) = SeverityLevel::from_string(&severity) {
            self.is_severity_allowed(&severity_enum)
        } else {
            false
        }
    }
}
