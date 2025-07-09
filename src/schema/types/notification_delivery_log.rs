use crate::models::{
    notification::NotificationChannels,
    notification_delivery_log::{ DeliveryStatus, NotificationDeliveryLog },
    prelude::*,
};
#[Object]
impl NotificationDeliveryLog {
    async fn id(&self) -> &str {
        &self.id
    }

    async fn notification_id(&self) -> &str {
        &self.notification_id
    }

    async fn channel(&self) -> NotificationChannels {
        self.channel
    }

    async fn delivery_status(&self) -> DeliveryStatus {
        self.delivery_status
    }

    async fn attempted_at(&self) -> &DateTime<Utc> {
        &self.attempted_at
    }

    async fn delivered_at(&self) -> Option<&DateTime<Utc>> {
        self.delivered_at.as_ref()
    }

    async fn error_message(&self) -> Option<&str> {
        self.error_message.as_deref()
    }

    async fn retry_count(&self) -> i32 {
        self.retry_count
    }

    async fn recipient_address(&self) -> Option<&str> {
        self.recipient_address.as_deref()
    }

    async fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }

    async fn updated_at(&self) -> &DateTime<Utc> {
        &self.updated_at
    }
}
