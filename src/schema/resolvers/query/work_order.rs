use async_graphql::*;
use tracing::warn;

use crate::{
    error::AppError,
    models::work_order::{
        WorkOrder,
        WorkOrderStatus,
        WorkOrderPriority,
        WorkOrderType,
        WorkOrderSeverity,
        WorkOrderDifficulty,
    },
    DbClient,
    Repository,
};

#[derive(Default, Debug)]
pub(crate) struct WorkOrderQuery;

#[Object]
impl WorkOrderQuery {
    /// Get work order by ID
    async fn work_order_by_id(
        &self,
        ctx: &Context<'_>,
        id: String
    ) -> Result<Option<WorkOrder>, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        repo.get::<WorkOrder>(id).await.map_err(|e| e.to_graphql_error())
    }

    /// Get all work orders with filtering
    async fn work_orders(
        &self,
        ctx: &Context<'_>,
        limit: Option<i32>,
        status_filter: Option<String>,
        priority_filter: Option<String>,
        type_filter: Option<String>,
        severity_filter: Option<String>,
        difficulty_filter: Option<String>
    ) -> Result<Vec<WorkOrder>, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        let mut work_orders = repo
            .list::<WorkOrder>(limit).await
            .map_err(|e| e.to_graphql_error())?;

        // Apply filters
        if let Some(status) = status_filter {
            let status_enum = WorkOrderStatus::from_string(&status).map_err(|e|
                e.to_graphql_error()
            )?;
            work_orders = work_orders
                .into_iter()
                .filter(|wo| wo.status.to_string() == status_enum.to_string())
                .collect();
        }

        if let Some(priority) = priority_filter {
            let priority_enum = WorkOrderPriority::from_string(&priority).map_err(|e|
                e.to_graphql_error()
            )?;
            work_orders = work_orders
                .into_iter()
                .filter(|wo| wo.priority.to_string() == priority_enum.to_string())
                .collect();
        }

        if let Some(wo_type) = type_filter {
            let type_enum = WorkOrderType::from_string(&wo_type).map_err(|e| e.to_graphql_error())?;
            work_orders = work_orders
                .into_iter()
                .filter(|wo| wo.work_order_type.to_string() == type_enum.to_string())
                .collect();
        }

        if let Some(severity) = severity_filter {
            let severity_enum = WorkOrderSeverity::from_string(&severity).map_err(|e|
                e.to_graphql_error()
            )?;
            work_orders = work_orders
                .into_iter()
                .filter(|wo| wo.severity.to_string() == severity_enum.to_string())
                .collect();
        }

        if let Some(difficulty) = difficulty_filter {
            let difficulty_enum = WorkOrderDifficulty::from_string(&difficulty).map_err(|e|
                e.to_graphql_error()
            )?;
            work_orders = work_orders
                .into_iter()
                .filter(|wo| wo.difficulty.to_string() == difficulty_enum.to_string())
                .collect();
        }

        Ok(work_orders)
    }

    /// Get work orders by severity level
    async fn work_orders_by_severity(
        &self,
        ctx: &Context<'_>,
        severity: String,
        limit: Option<i32>,
        status_filter: Option<String>
    ) -> Result<Vec<WorkOrder>, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        let severity_enum = WorkOrderSeverity::from_string(&severity).map_err(|e|
            e.to_graphql_error()
        )?;

        let mut work_orders = repo.list::<WorkOrder>(None).await.map_err(|e| e.to_graphql_error())?;

        work_orders = work_orders
            .into_iter()
            .filter(|wo| wo.severity.to_string() == severity_enum.to_string())
            .collect();

        // Apply status filter if provided
        if let Some(status) = status_filter {
            let status_enum = WorkOrderStatus::from_string(&status).map_err(|e|
                e.to_graphql_error()
            )?;
            work_orders = work_orders
                .into_iter()
                .filter(|wo| wo.status.to_string() == status_enum.to_string())
                .collect();
        }

        // Apply limit if provided
        if let Some(limit_val) = limit {
            work_orders.truncate(limit_val as usize);
        }

        Ok(work_orders)
    }

    /// Get work orders by difficulty level
    async fn work_orders_by_difficulty(
        &self,
        ctx: &Context<'_>,
        difficulty: String,
        limit: Option<i32>,
        status_filter: Option<String>
    ) -> Result<Vec<WorkOrder>, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        let difficulty_enum = WorkOrderDifficulty::from_string(&difficulty).map_err(|e|
            e.to_graphql_error()
        )?;

        let mut work_orders = repo.list::<WorkOrder>(None).await.map_err(|e| e.to_graphql_error())?;

        work_orders = work_orders
            .into_iter()
            .filter(|wo| wo.difficulty.to_string() == difficulty_enum.to_string())
            .collect();

        // Apply status filter if provided
        if let Some(status) = status_filter {
            let status_enum = WorkOrderStatus::from_string(&status).map_err(|e|
                e.to_graphql_error()
            )?;
            work_orders = work_orders
                .into_iter()
                .filter(|wo| wo.status.to_string() == status_enum.to_string())
                .collect();
        }

        // Apply limit if provided
        if let Some(limit_val) = limit {
            work_orders.truncate(limit_val as usize);
        }

        Ok(work_orders)
    }

    // Remove the work_orders_by_date_range query since scheduled_start no longer exists
    // Update work_orders_by_location to not reference asset_location_id since it's removed

    /// Get work orders by asset
    async fn work_orders_by_asset(
        &self,
        ctx: &Context<'_>,
        asset_id: String,
        limit: Option<i32>,
        status_filter: Option<String>
    ) -> Result<Vec<WorkOrder>, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        let mut work_orders = repo.list::<WorkOrder>(None).await.map_err(|e| e.to_graphql_error())?;

        work_orders = work_orders
            .into_iter()
            .filter(|wo| wo.asset_id == asset_id)
            .collect();

        // Apply status filter if provided
        if let Some(status) = status_filter {
            let status_enum = WorkOrderStatus::from_string(&status).map_err(|e|
                e.to_graphql_error()
            )?;
            work_orders = work_orders
                .into_iter()
                .filter(|wo| wo.status.to_string() == status_enum.to_string())
                .collect();
        }

        // Apply limit if provided
        if let Some(limit_val) = limit {
            work_orders.truncate(limit_val as usize);
        }

        Ok(work_orders)
    }

    // ... rest of the existing queries, but remove references to removed fields
}
