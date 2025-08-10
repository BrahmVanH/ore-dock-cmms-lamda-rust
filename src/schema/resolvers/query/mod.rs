use async_graphql::MergedObject;

mod asset_type;
mod asset;
mod location;
mod location_type;
mod user;
mod user_role;
mod role;
mod permission;
mod work_order;
mod manufacturer;
mod maintenance_schedule;
mod dashboard;
mod task;

#[derive(Debug, Default, MergedObject)]
pub struct QueryRoot(
    asset_type::AssetTypeQuery,
    asset::AssetQuery,
    location::LocationQuery,
    location_type::LocationTypeQuery,
    user::UserQuery,
    user_role::UserRoleQuery,
    role::RoleQuery,
    permission::PermissionQuery,
    work_order::WorkOrderQuery,
    manufacturer::ManufacturerQuery,
    maintenance_schedule::MaintenanceScheduleQuery,
    dashboard::DashboardQuery,
    task::TaskQuery,
);
