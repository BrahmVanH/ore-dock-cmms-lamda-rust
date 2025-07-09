use std::collections::HashMap;

use async_graphql::Enum;
use aws_sdk_dynamodb::types::AttributeValue;
use chrono::{ DateTime, Utc };
use serde::{ Deserialize, Serialize };
use tracing::info;

use crate::{
    error::AppError,
    models::notification::{ NotificationChannels, SeverityLevel },
    DynamoDbEntity,
};

#[derive(Enum, Copy, Clone, PartialEq, Eq, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum NotificationType {
    Alert,
    Reminder,
    Status,
    Maintenance,
    Emergency,
    Update,
}

impl NotificationType {
    pub(crate) fn to_str(&self) -> &str {
        match self {
            NotificationType::Alert => "alert",
            NotificationType::Reminder => "reminder",
            NotificationType::Status => "status",
            NotificationType::Maintenance => "maintenance",
            NotificationType::Emergency => "emergency",
            NotificationType::Update => "update",
        }
    }

    pub(crate) fn to_string(&self) -> String {
        self.to_str().to_string()
    }

    pub(crate) fn from_string(s: &str) -> Result<NotificationType, AppError> {
        match s {
            "alert" => Ok(Self::Alert),
            "reminder" => Ok(Self::Reminder),
            "status" => Ok(Self::Status),
            "maintenance" => Ok(Self::Maintenance),
            "emergency" => Ok(Self::Emergency),
            "update" => Ok(Self::Update),
            _ => Err(AppError::ValidationError("Invalid notification type".to_string())),
        }
    }
}

/// Represents template variables that can be used in message templates
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TemplateVariable {
    pub name: String,
    pub description: String,
    pub variable_type: String, // "string", "datetime", "number", "boolean"
    pub required: bool,
    pub default_value: Option<String>,
}

/// Represents a Notification Template in the system
///
/// # Fields
///
/// * `id` - Unique identifier for the template
/// * `name` - Name of the template
/// * `notification_type` - Type of notification (alert, reminder, etc.)
/// * `subject_template` - Template for notification subject/title
/// * `message_template` - Template for notification message body
/// * `default_severity` - Default severity level for notifications using this template
/// * `supported_channels` - List of channels that support this template
/// * `variables` - List of template variables that can be substituted
/// * `template_engine` - Template engine to use (handlebars, liquid, etc.)
/// * `active` - Whether this template is currently active
/// * `version` - Version number for template versioning
/// * `created_by` - User who created the template
/// * `created_at` - Date and time of creation
/// * `updated_at` - Date and time of last update
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NotificationTemplate {
    pub id: String,
    pub name: String,
    pub notification_type: NotificationType,
    pub subject_template: String,
    pub message_template: String,
    pub default_severity: SeverityLevel,
    pub supported_channels: Vec<NotificationChannels>,
    pub variables: Vec<TemplateVariable>,
    pub template_engine: String, // "handlebars", "liquid", "mustache", "simple"
    pub active: bool,
    pub version: i32,
    pub created_by: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Defines methods for NotificationTemplate
impl NotificationTemplate {
    /// Creates new NotificationTemplate instance
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier
    /// * `name` - Name of the template
    /// * `notification_type` - Type as string
    /// * `subject_template` - Subject template string
    /// * `message_template` - Message template string
    /// * `default_severity` - Default severity as string
    /// * `supported_channels` - List of supported channels
    /// * `variables` - List of template variables
    /// * `template_engine` - Template engine to use
    /// * `active` - Whether template is active
    /// * `version` - Version number
    /// * `created_by` - Optional creator user ID
    ///
    /// # Returns
    ///
    /// New NotificationTemplate instance
    pub fn new(
        id: String,
        name: String,
        notification_type: String,
        subject_template: String,
        message_template: String,
        default_severity: String,
        supported_channels: Vec<String>,
        variables: Vec<TemplateVariable>,
        template_engine: String,
        active: bool,
        version: i32,
        created_by: Option<String>
    ) -> Result<Self, AppError> {
        let now = Utc::now();

        if name.trim().is_empty() {
            return Err(AppError::ValidationError("Template name cannot be empty".to_string()));
        }

        if message_template.trim().is_empty() {
            return Err(AppError::ValidationError("Message template cannot be empty".to_string()));
        }

        let notif_type = NotificationType::from_string(&notification_type)?;
        let severity = SeverityLevel::from_string(&default_severity)?;

        let channels: Result<Vec<NotificationChannels>, AppError> = supported_channels
            .iter()
            .map(|c| NotificationChannels::from_string(c))
            .collect();
        let channels = channels?;

        Ok(Self {
            id,
            name,
            notification_type: notif_type,
            subject_template,
            message_template,
            default_severity: severity,
            supported_channels: channels,
            variables,
            template_engine,
            active,
            version,
            created_by,
            created_at: now,
            updated_at: now,
        })
    }
}

impl DynamoDbEntity for NotificationTemplate {
    fn table_name() -> &'static str {
        "NotificationTemplates"
    }

    fn primary_key(&self) -> String {
        self.id.clone()
    }

    /// Creates NotificationTemplate instance from DynamoDB item
    ///
    /// # Arguments
    ///
    /// * `item` - The dynamo db item
    ///
    /// # Returns
    ///
    /// 'Some' NotificationTemplate if item fields match, 'None' otherwise
    fn from_item(item: &HashMap<String, AttributeValue>) -> Option<Self> {
        info!("calling from_item with: {:?}", &item);

        let id = item.get("id")?.as_s().ok()?.to_string();
        let name = item.get("name")?.as_s().ok()?.to_string();

        let notification_type_str = item.get("notification_type")?.as_s().ok()?;
        let notification_type = NotificationType::from_string(&notification_type_str)
            .map_err(|e| e)
            .ok()?;

        let subject_template = item.get("subject_template")?.as_s().ok()?.to_string();
        let message_template = item.get("message_template")?.as_s().ok()?.to_string();

        let default_severity_str = item.get("default_severity")?.as_s().ok()?;
        let default_severity = SeverityLevel::from_string(&default_severity_str)
            .map_err(|e| e)
            .ok()?;

        let supported_channels = item
            .get("supported_channels")
            .and_then(|v| v.as_ss().ok())
            .map(|channel_strs| {
                channel_strs
                    .iter()
                    .filter_map(|c| NotificationChannels::from_string(c).ok())
                    .collect::<Vec<NotificationChannels>>()
            })
            .unwrap_or_default();

        // Handle variables as a list of maps
        let variables = item
            .get("variables")
            .and_then(|v| v.as_l().ok())
            .map(|var_list| {
                var_list
                    .iter()
                    .filter_map(|var_attr| {
                        var_attr
                            .as_m()
                            .ok()
                            .and_then(|var_map| TemplateVariable::from_item(var_map))
                    })
                    .collect::<Vec<TemplateVariable>>()
            })
            .unwrap_or_default();

        let template_engine = item
            .get("template_engine")
            .and_then(|v| v.as_s().ok())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "simple".to_string());

        let active = item
            .get("active")
            .and_then(|v| v.as_bool().ok())
            .unwrap_or(&true);

        let version = item
            .get("version")
            .and_then(|v| v.as_n().ok())
            .and_then(|s| s.parse::<i32>().ok())
            .unwrap_or(1);

        let created_by = item
            .get("created_by")
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
            name,
            notification_type,
            subject_template,
            message_template,
            default_severity,
            supported_channels,
            variables,
            template_engine,
            active: *active,
            version,
            created_by,
            created_at,
            updated_at,
        });

        info!("result of from_item on notification_template: {:?}", res);
        res
    }

    /// Creates DynamoDB item from NotificationTemplate instance
    ///
    /// # Arguments
    ///
    /// * `self` - borrowed instance of self
    ///
    /// # Returns
    ///
    /// HashMap representing DB item for NotificationTemplate instance
    fn to_item(&self) -> HashMap<String, AttributeValue> {
        let mut item = HashMap::new();

        item.insert("id".to_string(), AttributeValue::S(self.id.clone()));
        item.insert("name".to_string(), AttributeValue::S(self.name.clone()));
        item.insert(
            "notification_type".to_string(),
            AttributeValue::S(self.notification_type.to_str().to_string())
        );
        item.insert(
            "subject_template".to_string(),
            AttributeValue::S(self.subject_template.clone())
        );
        item.insert(
            "message_template".to_string(),
            AttributeValue::S(self.message_template.clone())
        );
        item.insert(
            "default_severity".to_string(),
            AttributeValue::S(self.default_severity.to_str().to_string())
        );

        // Convert channels to string set
        let channel_strings: Vec<String> = self.supported_channels
            .iter()
            .map(|c| c.to_str().to_string())
            .collect();
        if !channel_strings.is_empty() {
            item.insert("supported_channels".to_string(), AttributeValue::Ss(channel_strings));
        }

        // Convert variables to list of maps
        if !self.variables.is_empty() {
            let var_list: Vec<AttributeValue> = self.variables
                .iter()
                .map(|var| AttributeValue::M(var.to_item()))
                .collect();
            item.insert("variables".to_string(), AttributeValue::L(var_list));
        }

        item.insert("template_engine".to_string(), AttributeValue::S(self.template_engine.clone()));
        item.insert("active".to_string(), AttributeValue::Bool(self.active));
        item.insert("version".to_string(), AttributeValue::N(self.version.to_string()));

        if let Some(creator) = &self.created_by {
            item.insert("created_by".to_string(), AttributeValue::S(creator.clone()));
        }

        item.insert("created_at".to_string(), AttributeValue::S(self.created_at.to_string()));
        item.insert("updated_at".to_string(), AttributeValue::S(self.updated_at.to_string()));

        item
    }
}

impl TemplateVariable {
    /// Creates TemplateVariable from DynamoDB item
    pub(crate) fn from_item(item: &HashMap<String, AttributeValue>) -> Option<Self> {
        let name = item.get("name")?.as_s().ok()?.to_string();
        let description = item.get("description")?.as_s().ok()?.to_string();
        let variable_type = item.get("variable_type")?.as_s().ok()?.to_string();
        let required = item.get("required")?.as_bool().ok().unwrap_or(&false);
        let default_value = item
            .get("default_value")
            .and_then(|v| v.as_s().ok())
            .map(|s| s.to_string());

        Some(Self {
            name,
            description,
            variable_type,
            required: *required,
            default_value,
        })
    }

    /// Creates DynamoDB item from TemplateVariable
    pub(crate) fn to_item(&self) -> HashMap<String, AttributeValue> {
        let mut item = HashMap::new();

        item.insert("name".to_string(), AttributeValue::S(self.name.clone()));
        item.insert("description".to_string(), AttributeValue::S(self.description.clone()));
        item.insert("variable_type".to_string(), AttributeValue::S(self.variable_type.clone()));
        item.insert("required".to_string(), AttributeValue::Bool(self.required));

        if let Some(default) = &self.default_value {
            item.insert("default_value".to_string(), AttributeValue::S(default.clone()));
        }

        item
    }
}
