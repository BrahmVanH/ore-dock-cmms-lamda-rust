use std::collections::HashMap;

use async_graphql::Object;
use aws_sdk_dynamodb::types::AttributeValue;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value as Json;
use tracing::info;

use crate::error::AppError;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SeverityLevel {
    Low,
    Medium,
    High,
    Critical,
    Informational,
}

impl SeverityLevel {
    pub fn to_str(&self) -> &str {
        match self {
            SeverityLevel::Low => "low",
            SeverityLevel::Medium => "medium",
            SeverityLevel::High => "high",
            SeverityLevel::Critical => "critical",
            SeverityLevel::Informational => "informational",
        }
    }

    pub fn to_string(&self) -> String {
        self.to_str().to_string()
    }

    pub fn from_string(s: &str) -> Result<SeverityLevel, AppError> {
        match s {
            "low" => Ok(Self::Low),
            "medium" => Ok(Self::Medium),
            "high" => Ok(Self::High),
            "critical" => Ok(Self::Critical),
            "informational" => Ok(Self::Informational),
            _ => Err(AppError::ValidationError("Invalid severity level".to_string())),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum NotificationChannels {
    Email,
    Push,
    Sms,
    Slack,
    Teams,
    Webhook,
    InApp,
    Desktop,
}

impl NotificationChannels {
    pub fn to_str(&self) -> &str {
        match self {
            NotificationChannels::Email => "email",
            NotificationChannels::Push => "push",
            NotificationChannels::Sms => "sms",
            NotificationChannels::Slack => "slack",
            NotificationChannels::Teams => "teams",
            NotificationChannels::Webhook => "webhook",
            NotificationChannels::InApp => "in_app",
            NotificationChannels::Desktop => "desktop",
        }
    }

    pub fn to_string(&self) -> String {
        self.to_str().to_string()
    }

    pub fn from_string(s: &str) -> Result<NotificationChannels, AppError> {
        match s {
            "email" => Ok(Self::Email),
            "push" => Ok(Self::Push),
            "sms" => Ok(Self::Sms),
            "slack" => Ok(Self::Slack),
            "teams" => Ok(Self::Teams),
            "webhook" => Ok(Self::Webhook),
            "in_app" => Ok(Self::InApp),
            "desktop" => Ok(Self::Desktop),
            _ => Err(AppError::ValidationError("Invalid notification channel".to_string())),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum NotificationStatus {
    Draft,
    Pending,
    Sent,
    Failed,
    Cancelled,
    Partial, // Some channels succeeded, others failed
}

impl NotificationStatus {
    fn to_str(&self) -> &str {
        match self {
            NotificationStatus::Draft => "draft",
            NotificationStatus::Pending => "pending",
            NotificationStatus::Sent => "sent",
            NotificationStatus::Failed => "failed",
            NotificationStatus::Cancelled => "cancelled",
            NotificationStatus::Partial => "partial",
        }
    }

    fn to_string(&self) -> String {
        self.to_str().to_string()
    }

    fn from_string(s: &str) -> Result<NotificationStatus, AppError> {
        match s {
            "draft" => Ok(Self::Draft),
            "pending" => Ok(Self::Pending),
            "sent" => Ok(Self::Sent),
            "failed" => Ok(Self::Failed),
            "cancelled" => Ok(Self::Cancelled),
            "partial" => Ok(Self::Partial),
            _ => Err(AppError::ValidationError("Invalid notification status".to_string())),
        }
    }
}

/// Represents a Notification in the system
///
/// # Fields
///
/// * `id` - Unique identifier for the notification
/// * `template_id` - ID of the notification template used
/// * `recipient_id` - ID of the user who should receive the notification
/// * `subject` - Subject/title of the notification
/// * `message` - The rendered message body
/// * `context` - JSON context data used for template rendering
/// * `severity` - Severity level of the notification
/// * `status` - Current status of the notification
/// * `scheduled_at` - When the notification should be sent
/// * `sent_at` - When the notification was actually sent
/// * `delivered_channels` - List of channels where delivery was successful
/// * `failed_channels` - List of channels where delivery failed
/// * `read_at` - When the notification was read by the recipient
/// * `expires_at` - When the notification expires
/// * `retry_count` - Number of retry attempts
/// * `created_at` - Date and time of creation
/// * `updated_at` - Date and time of last update
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Notification {
    pub id: String,
    pub template_id: String,
    pub recipient_id: String,
    pub subject: String,
    pub message: String,
    pub context: Option<Json>,
    pub severity: SeverityLevel,
    pub status: NotificationStatus,
    pub scheduled_at: DateTime<Utc>,
    pub sent_at: Option<DateTime<Utc>>,
    pub delivered_channels: Vec<NotificationChannels>,
    pub failed_channels: Vec<NotificationChannels>,
    pub read_at: Option<DateTime<Utc>>,
    pub expires_at: Option<DateTime<Utc>>,
    pub retry_count: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Defines methods for Notification
impl Notification {
    /// Creates new Notification instance
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier
    /// * `template_id` - Template ID used
    /// * `recipient_id` - Recipient user ID
    /// * `subject` - Notification subject
    /// * `message` - Notification message
    /// * `context` - Optional JSON context
    /// * `severity` - Severity level as string
    /// * `scheduled_at` - When to send the notification
    /// * `expires_at` - Optional expiration time
    ///
    /// # Returns
    ///
    /// New Notification instance
    pub fn new(
        id: String,
        template_id: String,
        recipient_id: String,
        subject: String,
        message: String,
        context: Option<Json>,
        severity: String,
        scheduled_at: DateTime<Utc>,
        expires_at: Option<DateTime<Utc>>,
    ) -> Result<Self, AppError> {
        let now = Utc::now();

        if template_id.trim().is_empty() {
            return Err(AppError::ValidationError("Template ID cannot be empty".to_string()));
        }

        if recipient_id.trim().is_empty() {
            return Err(AppError::ValidationError("Recipient ID cannot be empty".to_string()));
        }

        if message.trim().is_empty() {
            return Err(AppError::ValidationError("Message cannot be empty".to_string()));
        }

        let severity_enum = SeverityLevel::from_string(&severity)?;

        Ok(Self {
            id,
            template_id,
            recipient_id,
            subject,
            message,
            context,
            severity: severity_enum,
            status: NotificationStatus::Draft,
            scheduled_at,
            sent_at: None,
            delivered_channels: Vec::new(),
            failed_channels: Vec::new(),
            read_at: None,
            expires_at,
            retry_count: 0,
            created_at: now,
            updated_at: now,
        })
    }

    /// Creates Notification instance from DynamoDB item
    ///
    /// # Arguments
    ///
    /// * `item` - The dynamo db item
    ///
    /// # Returns
    ///
    /// 'Some' Notification if item fields match, 'None' otherwise
    pub fn from_item(item: &HashMap<String, AttributeValue>) -> Option<Self> {
        info!("calling from_item with: {:?}", &item);

        let id = item.get("id")?.as_s().ok()?.to_string();
        let template_id = item.get("template_id")?.as_s().ok()?.to_string();
        let recipient_id = item.get("recipient_id")?.as_s().ok()?.to_string();
        let subject = item.get("subject")?.as_s().ok()?.to_string();
        let message = item.get("message")?.as_s().ok()?.to_string();

        let context = item
            .get("context")
            .and_then(|v| v.as_s().ok())
            .and_then(|s| serde_json::from_str::<Json>(s).ok());

        let severity_str = item.get("severity")?.as_s().ok()?;
        let severity = SeverityLevel::from_string(&severity_str)
            .map_err(|e| e)
            .ok()?;

        let status_str = item.get("status")?.as_s().ok()?;
        let status = NotificationStatus::from_string(&status_str)
            .map_err(|e| e)
            .ok()?;

        let scheduled_at = item
            .get("scheduled_at")
            .and_then(|v| v.as_s().ok())
            .and_then(|s| s.parse::<DateTime<Utc>>().ok())
            .unwrap_or_else(|| Utc::now());

        let sent_at = item
            .get("sent_at")
            .and_then(|v| v.as_s().ok())
            .and_then(|s| s.parse::<DateTime<Utc>>().ok());

        let delivered_channels = item
            .get("delivered_channels")
            .and_then(|v| v.as_ss().ok())
            .map(|channel_strs| {
                channel_strs
                    .iter()
                    .filter_map(|c| NotificationChannels::from_string(c).ok())
                    .collect::<Vec<NotificationChannels>>()
            })
            .unwrap_or_default();

        let failed_channels = item
            .get("failed_channels")
            .and_then(|v| v.as_ss().ok())
            .map(|channel_strs| {
                channel_strs
                    .iter()
                    .filter_map(|c| NotificationChannels::from_string(c).ok())
                    .collect::<Vec<NotificationChannels>>()
            })
            .unwrap_or_default();

        let read_at = item
            .get("read_at")
            .and_then(|v| v.as_s().ok())
            .and_then(|s| s.parse::<DateTime<Utc>>().ok());

        let expires_at = item
            .get("expires_at")
            .and_then(|v| v.as_s().ok())
            .and_then(|s| s.parse::<DateTime<Utc>>().ok());

        let retry_count = item
            .get("retry_count")
            .and_then(|v| v.as_n().ok())
            .and_then(|s| s.parse::<i32>().ok())
            .unwrap_or(0);

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
            template_id,
            recipient_id,
            subject,
            message,
            context,
            severity,
            status,
            scheduled_at,
            sent_at,
            delivered_channels,
            failed_channels,
            read_at,
            expires_at,
            retry_count,
            created_at,
            updated_at,
        });

        info!("result of from_item on notification: {:?}", res);
        res
    }

    /// Creates DynamoDB item from Notification instance
    ///
    /// # Arguments
    ///
    /// * `self` - borrowed instance of self
    ///
    /// # Returns
    ///
    /// HashMap representing DB item for Notification instance
    pub fn to_item(&self) -> HashMap<String, AttributeValue> {
        let mut item = HashMap::new();

        item.insert("id".to_string(), AttributeValue::S(self.id.clone()));
        item.insert("template_id".to_string(), AttributeValue::S(self.template_id.clone()));
        item.insert("recipient_id".to_string(), AttributeValue::S(self.recipient_id.clone()));
        item.insert("subject".to_string(), AttributeValue::S(self.subject.clone()));
        item.insert("message".to_string(), AttributeValue::S(self.message.clone()));

        if let Some(context) = &self.context {
            if let Ok(context_json) = serde_json::to_string(context) {
                item.insert("context".to_string(), AttributeValue::S(context_json));
            }
        }

        item.insert("severity".to_string(), AttributeValue::S(self.severity.to_str().to_string()));
        item.insert("status".to_string(), AttributeValue::S(self.status.to_str().to_string()));
        item.insert("scheduled_at".to_string(), AttributeValue::S(self.scheduled_at.to_string()));

        if let Some(sent) = &self.sent_at {
            item.insert("sent_at".to_string(), AttributeValue::S(sent.to_string()));
        }

        // Convert delivered channels to string set
        if !self.delivered_channels.is_empty() {
            let delivered_strings: Vec<String> = self
                .delivered_channels
                .iter()
                .map(|c| c.to_str().to_string())
                .collect();
            item.insert("delivered_channels".to_string(), AttributeValue::Ss(delivered_strings));
        }

        // Convert failed channels to string set
        if !self.failed_channels.is_empty() {
            let failed_strings: Vec<String> = self
                .failed_channels
                .iter()
                .map(|c| c.to_str().to_string())
                .collect();
            item.insert("failed_channels".to_string(), AttributeValue::Ss(failed_strings));
        }

        if let Some(read) = &self.read_at {
            item.insert("read_at".to_string(), AttributeValue::S(read.to_string()));
        }

        if let Some(expires) = &self.expires_at {
            item.insert("expires_at".to_string(), AttributeValue::S(expires.to_string()));
        }

        item.insert("retry_count".to_string(), AttributeValue::N(self.retry_count.to_string()));
        item.insert("created_at".to_string(), AttributeValue::S(self.created_at.to_string()));
        item.insert("updated_at".to_string(), AttributeValue::S(self.updated_at.to_string()));

        item
    }
}

#[Object]
impl Notification {
    async fn id(&self) -> &str {
        &self.id
    }

    async fn template_id(&self) -> &str {
        &self.template_id
    }

    async fn recipient_id(&self) -> &str {
        &self.recipient_id
    }

    async fn subject(&self) -> &str {
        &self.subject
    }

    async fn message(&self) -> &str {
        &self.message
    }

    async fn context(&self) -> Option<String> {
        self.context.as_ref().and_then(|c| serde_json::to_string(c).ok())
    }

    async fn severity(&self) -> &str {
        self.severity.to_str()
    }

    async fn status(&self) -> &str {
        self.status.to_str()
    }

    async fn scheduled_at(&self) -> &DateTime<Utc> {
        &self.scheduled_at
    }

    async fn sent_at(&self) -> Option<&DateTime<Utc>> {
        self.sent_at.as_ref()
    }

    async fn delivered_channels(&self) -> Vec<String> {
        self.delivered_channels.iter().map(|c| c.to_str().to_string()).collect()
    }

    async fn failed_channels(&self) -> Vec<String> {
        self.failed_channels.iter().map(|c| c.to_str().to_string()).collect()
    }

    async fn read_at(&self) -> Option<&DateTime<Utc>> {
        self.read_at.as_ref()
    }

    async fn expires_at(&self) -> Option<&DateTime<Utc>> {
        self.expires_at.as_ref()
    }

    async fn retry_count(&self) -> i32 {
        self.retry_count
    }

    async fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }

    async fn updated_at(&self) -> &DateTime<Utc> {
        &self.updated_at
    }
}