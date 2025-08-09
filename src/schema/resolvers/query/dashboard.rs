use std::collections::HashMap;
use tracing::warn;

use crate::{
    error::AppError,
    models::{
        asset::AssetCurrentStatusOptions,
        maintenance_schedule::MaintenanceSchedule,
        prelude::*,
        work_order::{ WorkOrderPriority, WorkOrderStatus },
    },
    schema::dashboard::{
        AssetsByStatus,
        DashboardFilter,
        DashboardStatistics,
        WorkOrdersByPriority,
        WorkOrdersByStatus,
    },
    DbClient,
    Repository,
};

#[derive(Debug, Default)]
pub(crate) struct DashboardQuery;

#[Object]
impl DashboardQuery {
    /// Get comprehensive dashboard statistics
    async fn dashboard_statistics(
        &self,
        ctx: &Context<'_>,
        filter: Option<DashboardFilter>
    ) -> Result<DashboardStatistics, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        let assets = repo.list::<Asset>(None).await.map_err(|e| e.to_graphql_error())?;
        let work_orders = repo.list::<WorkOrder>(None).await.map_err(|e| e.to_graphql_error())?;
        let maintenance_schedules = repo
            .list::<MaintenanceSchedule>(None).await
            .map_err(|e| e.to_graphql_error())?;

        // Apply filters if provided (for future use)
        let filtered_assets = if let Some(_filter) = &filter {
            // Future: apply filters here
            assets
        } else {
            assets
        };

        let filtered_work_orders = if let Some(_filter) = &filter {
            // Future: apply filters here
            work_orders
        } else {
            work_orders
        };

        let filtered_maintenance_schedules = if let Some(_filter) = &filter {
            // Future: apply filters here
            maintenance_schedules
        } else {
            maintenance_schedules
        };

        // Calculate basic counts
        let total_assets = filtered_assets.len() as i32;

        let active_work_orders = filtered_work_orders
            .iter()
            .filter(|wo| !wo.is_completed())
            .count() as i32;

        let now = Utc::now();
        let upcoming_days = 30; // Default to 30 days ahead
        let cutoff_date = now + chrono::Duration::days(upcoming_days);

        let overdue_maintenances = filtered_maintenance_schedules
            .iter()
            .filter(|ms| ms.active && ms.next_due_at < now)
            .count() as i32;

        let upcoming_maintenances = filtered_maintenance_schedules
            .iter()
            .filter(|ms| ms.active && ms.next_due_at >= now && ms.next_due_at <= cutoff_date)
            .count() as i32;

        // Calculate assets by status
        let mut assets_by_status_map: HashMap<AssetCurrentStatusOptions, i32> = HashMap::new();
        for asset in &filtered_assets {
            *assets_by_status_map.entry(asset.current_status).or_insert(0) += 1;
        }

        let assets_by_status = AssetsByStatus {
            operational: *assets_by_status_map
                .get(&AssetCurrentStatusOptions::Operational)
                .unwrap_or(&0),
            down: *assets_by_status_map.get(&AssetCurrentStatusOptions::Down).unwrap_or(&0),
            maintenance: *assets_by_status_map
                .get(&AssetCurrentStatusOptions::Maintenance)
                .unwrap_or(&0),
            retired: *assets_by_status_map.get(&AssetCurrentStatusOptions::Retired).unwrap_or(&0),
            needs_attention: *assets_by_status_map
                .get(&AssetCurrentStatusOptions::NeedsAttention)
                .unwrap_or(&0),
        };

        // Calculate work orders by priority
        let mut work_orders_by_priority_map: HashMap<WorkOrderPriority, i32> = HashMap::new();
        for work_order in &filtered_work_orders {
            *work_orders_by_priority_map.entry(work_order.priority.clone()).or_insert(0) += 1;
        }

        let work_orders_by_priority = WorkOrdersByPriority {
            low: *work_orders_by_priority_map.get(&WorkOrderPriority::Low).unwrap_or(&0),
            normal: *work_orders_by_priority_map.get(&WorkOrderPriority::Normal).unwrap_or(&0),
            high: *work_orders_by_priority_map.get(&WorkOrderPriority::High).unwrap_or(&0),
            urgent: *work_orders_by_priority_map.get(&WorkOrderPriority::Urgent).unwrap_or(&0),
            emergency: *work_orders_by_priority_map
                .get(&WorkOrderPriority::Emergency)
                .unwrap_or(&0),
        };

        // Calculate work orders by status
        let mut work_orders_by_status_map: HashMap<WorkOrderStatus, i32> = HashMap::new();
        for work_order in &filtered_work_orders {
            *work_orders_by_status_map.entry(work_order.status).or_insert(0) += 1;
        }

        let work_orders_by_status = WorkOrdersByStatus {
            open: *work_orders_by_status_map.get(&WorkOrderStatus::Scheduled).unwrap_or(&0),
            in_progress: *work_orders_by_status_map.get(&WorkOrderStatus::InProgress).unwrap_or(&0),
            on_hold: *work_orders_by_status_map.get(&WorkOrderStatus::OnHold).unwrap_or(&0),
            completed: *work_orders_by_status_map.get(&WorkOrderStatus::Completed).unwrap_or(&0),
            cancelled: *work_orders_by_status_map.get(&WorkOrderStatus::Cancelled).unwrap_or(&0),
        };

        // Calculate additional metrics
        let assets_down = assets_by_status.down;
        let assets_needing_attention = assets_by_status.needs_attention;
        let critical_work_orders =
            work_orders_by_priority.urgent + work_orders_by_priority.emergency;

        let total_maintenance_schedules = filtered_maintenance_schedules.len() as i32;
        let active_maintenance_schedules = filtered_maintenance_schedules
            .iter()
            .filter(|ms| ms.active)
            .count() as i32;

        // Assets under warranty
        let assets_under_warranty = filtered_assets
            .iter()
            .filter(|asset| {
                if let Some(warranty_end) = asset.warranty_end_date {
                    warranty_end > now
                } else {
                    false
                }
            })
            .count() as i32;

        // Recent work order completions (last 7 days)
        let seven_days_ago = now - chrono::Duration::days(7);
        let recent_completions = filtered_work_orders
            .iter()
            .filter(|wo| {
                if let Some(completed_date) = wo.completed_date {
                    completed_date >= seven_days_ago && wo.is_completed()
                } else {
                    false
                }
            })
            .count() as i32;

        Ok(DashboardStatistics {
            total_assets,
            active_work_orders,
            overdue_maintenances,
            upcoming_maintenances,
            assets_by_status,
            work_orders_by_priority,
            work_orders_by_status,
            assets_down,
            assets_needing_attention,
            critical_work_orders,
            total_maintenance_schedules,
            active_maintenance_schedules,
            assets_under_warranty,
            recent_completions,
        })
    }
}
