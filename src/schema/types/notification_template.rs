use crate::models::{
    notification::{NotificationChannels, SeverityLevel},
    notification_template::{ NotificationTemplate, NotificationType, TemplateVariable },
    prelude::*,
};
#[Object]
impl NotificationTemplate {
    async fn id(&self) -> &str {
        &self.id
    }

    async fn name(&self) -> &str {
        &self.name
    }

    async fn notification_type(&self) -> NotificationType {
        self.notification_type
    }

    async fn subject_template(&self) -> &str {
        &self.subject_template
    }

    async fn message_template(&self) -> &str {
        &self.message_template
    }

    async fn default_severity(&self) -> SeverityLevel {
        self.default_severity
    }

    async fn supported_channels(&self) -> Vec<NotificationChannels> {
        self.supported_channels
            .iter()
            .map(|c| c.clone())
            .collect()
    }

    async fn variables(&self) -> &Vec<TemplateVariable> {
        &self.variables
    }

    async fn template_engine(&self) -> &str {
        &self.template_engine
    }

    async fn active(&self) -> bool {
        self.active
    }

    async fn version(&self) -> i32 {
        self.version
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
}

#[Object]
impl TemplateVariable {
    async fn name(&self) -> &str {
        &self.name
    }

    async fn description(&self) -> &str {
        &self.description
    }

    async fn variable_type(&self) -> &str {
        &self.variable_type
    }

    async fn required(&self) -> bool {
        self.required
    }

    async fn default_value(&self) -> Option<&str> {
        self.default_value.as_deref()
    }
}
