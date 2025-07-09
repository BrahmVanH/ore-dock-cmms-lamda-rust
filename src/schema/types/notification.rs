use crate::models::{
  notification::{ Notification, NotificationChannels, NotificationStatus, SeverityLevel },
  prelude::*,
};

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

  async fn severity(&self) -> SeverityLevel {
      self.severity
  }

  async fn status(&self) -> NotificationStatus {
      self.status
  }

  async fn scheduled_at(&self) -> &DateTime<Utc> {
      &self.scheduled_at
  }

  async fn sent_at(&self) -> Option<&DateTime<Utc>> {
      self.sent_at.as_ref()
  }

  async fn delivered_channels(&self) -> Vec<NotificationChannels> {
      self.delivered_channels
          .iter()
          .map(|c| c.clone())
          .collect()
  }

  async fn failed_channels(&self) -> Vec<NotificationChannels> {
      self.failed_channels
          .iter()
          .map(|c| c.clone())
          .collect()
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
