use async_graphql::MergedObject;

use crate::schema::resolvers::mutation::location_type::LocationTypeMutationRoot;

mod asset;
mod asset_type;
mod location;
mod location_type;
mod manufacturer;
mod user;
mod work_order;
mod maintenance_schedule;
mod notification;
mod vendor;
mod vendor_category;
mod user_role;
mod user_notification_preferences;
mod notification_template;

#[derive(Debug, Default, MergedObject)]
pub struct MutationRoot {
    location_type_mutation_root: LocationTypeMutationRoot,
    asset_mutation_root: asset::AssetMutationRoot,
    asset_type_mutation_root: asset_type::AssetTypeMutationRoot,
    location_mutation_root: location::LocationMutationRoot,
    // manufacturer_mutation_root: manufacturer::ManufacturerMutationRoot,
    user_mutation_root: user::UserMutationRoot,
    work_order_mutation_root: work_order::WorkOrderMutationRoot,
    maintenance_schedule_mutation_root: maintenance_schedule::MaintenanceScheduleMutationRoot,
    notification_mutation_root: notification::NotificationMutationRoot,
    vendor_mutation_root: vendor::VendorMutationRoot,
    vendor_category_mutation_root: vendor_category::VendorCategoryMutationRoot,
    user_role_mutation_root: user_role::UserRoleMutationRoot,
    user_notification_preferences_mutation_root: user_notification_preferences::UserNotificationPreferencesMutationRoot,
    notification_template_mutation_root: notification_template::NotificationTemplateMutationRoot,
}
