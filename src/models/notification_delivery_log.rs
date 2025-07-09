use std::collections::HashMap;

use async_graphql::Enum;
use aws_sdk_dynamodb::types::AttributeValue;
use chrono::{ DateTime, Utc };
use serde::{ Deserialize, Serialize };
use tracing::info;

use crate::{ error::AppError, models::notification::NotificationChannels, DynamoDbEntity };

#[derive(Enum, Copy, Clone, PartialEq, Eq, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DeliveryStatus {
    Pending,
    Delivered,
    Failed,
    Retrying,
    Cancelled,
}

impl DeliveryStatus {
    pub(crate) fn to_str(&self) -> &str {
        match self {
            DeliveryStatus::Pending => "pending",
            DeliveryStatus::Delivered => "delivered",
            DeliveryStatus::Failed => "failed",
            DeliveryStatus::Retrying => "retrying",
            DeliveryStatus::Cancelled => "cancelled",
        }
    }

    pub(crate) fn to_string(&self) -> String {
        self.to_str().to_string()
    }

    pub(crate) fn from_string(s: &str) -> Result<DeliveryStatus, AppError> {
        match s {
            "pending" => Ok(Self::Pending),
            "delivered" => Ok(Self::Delivered),
            "failed" => Ok(Self::Failed),
            "retrying" => Ok(Self::Retrying),
            "cancelled" => Ok(Self::Cancelled),
            _ => Err(AppError::ValidationError("Invalid delivery status".to_string())),
        }
    }
}

/// Represents a Notification Delivery Log in the system
///
/// # Fields
///
/// * `id` - Unique identifier for the delivery log entry
/// * `notification_id` - ID of the notification being delivered
/// * `channel` - The delivery channel used (email, sms, push, etc.)
/// * `delivery_status` - Current status of the delivery attempt
/// * `attempted_at` - When the delivery was attempted
/// * `delivered_at` - When the notification was successfully delivered
/// * `error_message` - Error message if delivery failed
/// * `retry_count` - Number of retry attempts
/// * `recipient_address` - The actual address/endpoint used for delivery
/// * `created_at` - Date and time of creation
/// * `updated_at` - Date and time of last update
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NotificationDeliveryLog {
    pub id: String,
    pub notification_id: String,
    pub channel: NotificationChannels,
    pub delivery_status: DeliveryStatus,
    pub attempted_at: DateTime<Utc>,
    pub delivered_at: Option<DateTime<Utc>>,
    pub error_message: Option<String>,
    pub retry_count: i32,
    pub recipient_address: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Defines methods for NotificationDeliveryLog
impl NotificationDeliveryLog {
    /// Creates new NotificationDeliveryLog instance
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier
    /// * `notification_id` - ID of the notification
    /// * `channel` - Delivery channel as string
    /// * `delivery_status` - Delivery status as string
    /// * `attempted_at` - When delivery was attempted
    /// * `delivered_at` - Optional delivery success time
    /// * `error_message` - Optional error message
    /// * `retry_count` - Number of retries
    /// * `recipient_address` - Optional recipient address
    ///
    /// # Returns
    ///
    /// New NotificationDeliveryLog instance
    pub fn new(
        id: String,
        notification_id: String,
        channel: String,
        delivery_status: String,
        attempted_at: DateTime<Utc>,
        delivered_at: Option<DateTime<Utc>>,
        error_message: Option<String>,
        retry_count: i32,
        recipient_address: Option<String>
    ) -> Result<Self, AppError> {
        let now = Utc::now();

        let channel_enum = NotificationChannels::from_string(&channel)?;
        let status_enum = DeliveryStatus::from_string(&delivery_status)?;

        if notification_id.trim().is_empty() {
            return Err(AppError::ValidationError("Notification ID cannot be empty".to_string()));
        }

        Ok(Self {
            id,
            notification_id,
            channel: channel_enum,
            delivery_status: status_enum,
            attempted_at,
            delivered_at,
            error_message,
            retry_count,
            recipient_address,
            created_at: now,
            updated_at: now,
        })
    }
}

impl DynamoDbEntity for NotificationDeliveryLog {
    fn table_name() -> &'static str {
        "NotificationDeliveryLogs"
    }

    fn primary_key(&self) -> String {
        self.id.clone()
    }

    /// Creates NotificationDeliveryLog instance from DynamoDB item
    ///
    /// # Arguments
    ///
    /// * `item` - The dynamo db item
    ///
    /// # Returns
    ///
    /// 'Some' NotificationDeliveryLog if item fields match, 'None' otherwise
    fn from_item(item: &HashMap<String, AttributeValue>) -> Option<Self> {
        info!("calling from_item with: {:?}", &item);

        let id = item.get("id")?.as_s().ok()?.to_string();
        let notification_id = item.get("notification_id")?.as_s().ok()?.to_string();

        let channel_str = item.get("channel")?.as_s().ok()?;
        let channel = NotificationChannels::from_string(&channel_str)
            .map_err(|e| e)
            .ok()?;

        let delivery_status_str = item.get("delivery_status")?.as_s().ok()?;
        let delivery_status = DeliveryStatus::from_string(&delivery_status_str)
            .map_err(|e| e)
            .ok()?;

        let attempted_at = item
            .get("attempted_at")
            .and_then(|v| v.as_s().ok())
            .and_then(|s| s.parse::<DateTime<Utc>>().ok())
            .unwrap_or_else(|| Utc::now());

        let delivered_at = item
            .get("delivered_at")
            .and_then(|v| v.as_s().ok())
            .and_then(|s| s.parse::<DateTime<Utc>>().ok());

        let error_message = item
            .get("error_message")
            .and_then(|v| v.as_s().ok())
            .map(|s| s.to_string());

        let retry_count = item
            .get("retry_count")
            .and_then(|v| v.as_n().ok())
            .and_then(|s| s.parse::<i32>().ok())
            .unwrap_or(0);

        let recipient_address = item
            .get("recipient_address")
            .and_then(|v| v.as_s().ok())
            .map(|s| s.to_string());

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
            notification_id,
            channel,
            delivery_status,
            attempted_at,
            delivered_at,
            error_message,
            retry_count,
            recipient_address,
            created_at,
            updated_at,
        });

        info!("result of from_item on notification_delivery_log: {:?}", res);
        res
    }

    /// Creates DynamoDB item from NotificationDeliveryLog instance
    ///
    /// # Arguments
    ///
    /// * `self` - borrowed instance of self
    ///
    /// # Returns
    ///
    /// HashMap representing DB item for NotificationDeliveryLog instance
    fn to_item(&self) -> HashMap<String, AttributeValue> {
        let mut item = HashMap::new();

        item.insert("id".to_string(), AttributeValue::S(self.id.clone()));
        item.insert("notification_id".to_string(), AttributeValue::S(self.notification_id.clone()));
        item.insert("channel".to_string(), AttributeValue::S(self.channel.to_str().to_string()));
        item.insert(
            "delivery_status".to_string(),
            AttributeValue::S(self.delivery_status.to_str().to_string())
        );
        item.insert("attempted_at".to_string(), AttributeValue::S(self.attempted_at.to_string()));

        if let Some(delivered) = &self.delivered_at {
            item.insert("delivered_at".to_string(), AttributeValue::S(delivered.to_string()));
        }

        if let Some(error) = &self.error_message {
            item.insert("error_message".to_string(), AttributeValue::S(error.clone()));
        }

        item.insert("retry_count".to_string(), AttributeValue::N(self.retry_count.to_string()));

        if let Some(address) = &self.recipient_address {
            item.insert("recipient_address".to_string(), AttributeValue::S(address.clone()));
        }

        item.insert("created_at".to_string(), AttributeValue::S(self.created_at.to_string()));
        item.insert("updated_at".to_string(), AttributeValue::S(self.updated_at.to_string()));

        item
    }
}
