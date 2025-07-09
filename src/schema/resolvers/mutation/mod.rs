use async_graphql::MergedObject;

mod asset;
mod asset_type;
mod location;
mod location_type;
mod manufacturer;
mod user;
mod role;
mod work_order;
mod maintenance_schedule;
mod notification;
mod vendor;
mod vendor_category;
mod user_role;
mod user_notification_preferences;
mod notification_template;
mod permission;

#[derive(Debug, Default, MergedObject)]
pub struct MutationRoot(
    location_type::LocationTypeMutation,
    asset::AssetMutation,
    asset_type::AssetTypeMutation,
    location::LocationMutation,
    // manufacturer_mutation_root: manufacturer::ManufacturerMutationRoot,
    user::UserMutation,
    work_order::WorkOrderMutation,
    role::RoleMutation,
    permission::PermissionMutation,
    maintenance_schedule::MaintenanceScheduleMutation,
    manufacturer::ManufacturerMutation,
    // notification_mutation_root: notification::NotificationMutationRoot,
    // vendor_mutation_root: vendor::VendorMutationRoot,
    // vendor_category_mutation_root: vendor_category::VendorCategoryMutationRoot,
    user_role::UserRoleMutation,
    // user_notification_preferences_mutation_root: user_notification_preferences::UserNotificationPreferencesMutationRoot,
    // notification_template_mutation_root: notification_template::NotificationTemplateMutationRoot,
);
