//! Common types and traits used throughout the models module

// Re-export all entity types
pub use super::address::Address;
pub use super::asset::Asset;
pub use super::asset_type::AssetType;
pub use super::location::Location;
pub use super::location_type::LocationType;
pub use super::maintenance_schedule::MaintenanceSchedule;
pub use super::manufacturer::Manufacturer;
pub use super::notification::Notification;
pub use super::notification_delivery_log::NotificationDeliveryLog;
pub use super::notification_template::NotificationTemplate;
pub use super::permission::Permission;
pub use super::permission_log::PermissionLog;
pub use super::role::Role;
pub use super::role_hierarchy::RoleHierarchy;
pub use super::temp_role_elevation::TempRoleElevation;
pub use super::user::User;
pub use super::user_notification_preferences::UserNotificationPreferences;
pub use super::user_preferences::UserPreferences;
pub use super::user_role::UserRole;
pub use super::vendor::Vendor;
pub use super::vendor_category::VendorCategory;
pub use super::work_order::WorkOrder;

// Re-export common enums and types from asset module
pub use super::asset::{
    AssetCurrentStatusOptions,
    MaintenanceFrequencyOptions,
    AssetCondition,
    AssetCriticality,
    AssetStatus,
};

// Re-export common enums and types from asset_type module
pub use super::asset_type::{
    AssetCategory,
    AssetTypeStatus,
};

// Re-export common enums and types from location module
pub use super::location::{
    LocationStatus,
};

// Re-export common enums and types from location_type module
pub use super::location_type::{
    LocationTypeCategory,
    LocationTypeStatus,
};

// Re-export common enums and types from maintenance_schedule module
pub use super::maintenance_schedule::{
    MaintenanceFrequency,
    MaintenanceType,
    ScheduleStatus,
};

// Re-export common enums and types from manufacturer module
pub use super::manufacturer::{
    ManufacturerStatus,
    ManufacturerType,
};

// Re-export common enums and types from notification module
pub use super::notification::{
    NotificationPriority,
    NotificationStatus,
    NotificationType,
};

// Re-export common enums and types from notification_delivery_log module
pub use super::notification_delivery_log::{
    DeliveryMethod,
    DeliveryStatus,
};

// Re-export common enums and types from notification_template module
pub use super::notification_template::{
    TemplateStatus,
    TemplateType,
};

// Re-export common enums and types from permission module
pub use super::permission::{
    PermissionScope,
    PermissionType,
};

// Re-export common enums and types from permission_log module
pub use super::permission_log::{
    PermissionAction,
    PermissionLogType,
};

// Re-export common enums and types from role module
pub use super::role::{
    RoleStatus,
    RoleType,
};

// Re-export common enums and types from role_hierarchy module
pub use super::role_hierarchy::{
    HierarchyRelationType,
    HierarchyStatus,
};

// Re-export common enums and types from temp_role_elevation module
pub use super::temp_role_elevation::{
    ElevationRequestStatus,
    ElevationReason,
    ElevationStatus,
};

// Re-export common enums and types from user module
pub use super::user::{
    UserStatus,
    UserType,
};

// Re-export common enums and types from user_notification_preferences module
pub use super::user_notification_preferences::{
    NotificationChannel,
    NotificationFrequency,
    PreferenceStatus,
};

// Re-export common enums and types from user_preferences module
pub use super::user_preferences::{
    Language,
    Theme,
    TimeZone,
};

// Re-export common enums and types from user_role module
pub use super::user_role::{
    AssignmentStatus,
    AssignmentType,
};

// Re-export common enums and types from vendor module
pub use super::vendor::{
    VendorStatus,
    VendorTier,
};

// Re-export common enums and types from vendor_category module
pub use super::vendor_category::{
    CategoryStatus,
    CategoryType,
};

// Re-export common enums and types from work_order module
pub use super::work_order::{
    WorkOrderPriority,
    WorkOrderStatus,
    WorkOrderType,
};

// Re-export error types
pub use crate::error::{AppError, AppResult};

// Re-export common external dependencies
pub use async_graphql::Object;
pub use aws_sdk_dynamodb::types::AttributeValue;
pub use chrono::{DateTime, Utc};
pub use rust_decimal::Decimal;
pub use serde::{Deserialize, Serialize};
pub use serde_json::Value as Json;
pub use std::collections::HashMap;
pub use tracing::info;

// Common type aliases
pub type EntityId = String;
pub type DbResult<T> = Result<T, AppError>;

// Common traits that might be implemented across models
pub trait Timestamped {
    fn created_at(&self) -> &DateTime<Utc>;
    fn updated_at(&self) -> &DateTime<Utc>;
    fn touch(&mut self);
}

pub trait Identifiable {
    fn id(&self) -> &str;
}

pub trait Activatable {
    fn is_active(&self) -> bool;
    fn activate(&mut self);
    fn deactivate(&mut self);
}

// DynamoDB operations trait
pub trait DynamoDbItem {
    fn from_item(item: &HashMap<String, AttributeValue>) -> Option<Self>
    where
        Self: Sized;
    fn to_item(&self) -> HashMap<String, AttributeValue>;
}