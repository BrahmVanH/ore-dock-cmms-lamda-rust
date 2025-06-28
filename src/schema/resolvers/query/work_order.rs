use async_graphql::*;
use chrono::{DateTime, Utc};
use tracing::warn;

use crate::{
    error::AppError,
    models::work_order::{WorkOrder, WorkOrderStatus, WorkOrderPriority, WorkOrderType},
    schema::resolvers::query::QueryRoot,
    DbClient,
    Repository,
};

#[Object]
impl QueryRoot {
    /// Get work order by ID
    async fn work_order_by_id(
        &self,
        ctx: &Context<'_>,
        id: String,
    ) -> Result<Option<WorkOrder>, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        repo.get::<WorkOrder>(id)
            .await
            .map_err(|e| e.to_graphql_error())
    }

    /// Get all work orders
    async fn work_orders(
        &self,
        ctx: &Context<'_>,
        limit: Option<i32>,
        status_filter: Option<String>,
        priority_filter: Option<String>,
        type_filter: Option<String>,
    ) -> Result<Vec<WorkOrder>, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        let mut work_orders = repo
            .list::<WorkOrder>(limit)
            .await
            .map_err(|e| e.to_graphql_error())?;

        // Apply filters
        if let Some(status) = status_filter {
            let status_enum = WorkOrderStatus::from_string(&status)
                .map_err(|e| e.to_graphql_error())?;
            work_orders = work_orders
                .into_iter()
                .filter(|wo| wo.status.to_string() == status_enum.to_string())
                .collect();
        }

        if let Some(priority) = priority_filter {
            let priority_enum = WorkOrderPriority::from_string(&priority)
                .map_err(|e| e.to_graphql_error())?;
            work_orders = work_orders
                .into_iter()
                .filter(|wo| wo.priority.to_string() == priority_enum.to_string())
                .collect();
        }

        if let Some(wo_type) = type_filter {
            let type_enum = WorkOrderType::from_string(&wo_type)
                .map_err(|e| e.to_graphql_error())?;
            work_orders = work_orders
                .into_iter()
                .filter(|wo| wo.work_order_type.to_string() == type_enum.to_string())
                .collect();
        }

        Ok(work_orders)
    }

    /// Get work orders by asset
    async fn work_orders_by_asset(
        &self,
        ctx: &Context<'_>,
        asset_id: String,
        limit: Option<i32>,
        status_filter: Option<String>,
    ) -> Result<Vec<WorkOrder>, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        // This would ideally use a GSI on asset_id
        // For now, we'll scan and filter (not ideal for production)
        let mut work_orders = repo
            .list::<WorkOrder>(None)
            .await
            .map_err(|e| e.to_graphql_error())?;

        work_orders = work_orders
            .into_iter()
            .filter(|wo| wo.asset_id == asset_id)
            .collect();

        // Apply status filter if provided
        if let Some(status) = status_filter {
            let status_enum = WorkOrderStatus::from_string(&status)
                .map_err(|e| e.to_graphql_error())?;
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

    /// Get work orders by technician
    async fn work_orders_by_technician(
        &self,
        ctx: &Context<'_>,
        technician_id: String,
        limit: Option<i32>,
        status_filter: Option<String>,
    ) -> Result<Vec<WorkOrder>, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        // This would ideally use a GSI on assigned_technician_id
        let mut work_orders = repo
            .list::<WorkOrder>(None)
            .await
            .map_err(|e| e.to_graphql_error())?;

        work_orders = work_orders
            .into_iter()
            .filter(|wo| {
                wo.assigned_technician_id.as_ref() == Some(&technician_id)
            })
            .collect();

        // Apply status filter if provided
        if let Some(status) = status_filter {
            let status_enum = WorkOrderStatus::from_string(&status)
                .map_err(|e| e.to_graphql_error())?;
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

    /// Get work orders by date range
    async fn work_orders_by_date_range(
        &self,
        ctx: &Context<'_>,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
        limit: Option<i32>,
        status_filter: Option<String>,
    ) -> Result<Vec<WorkOrder>, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        let mut work_orders = repo
            .list::<WorkOrder>(None)
            .await
            .map_err(|e| e.to_graphql_error())?;

        work_orders = work_orders
            .into_iter()
            .filter(|wo| {
                wo.scheduled_start >= start_date && wo.scheduled_start <= end_date
            })
            .collect();

        // Apply status filter if provided
        if let Some(status) = status_filter {
            let status_enum = WorkOrderStatus::from_string(&status)
                .map_err(|e| e.to_graphql_error())?;
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

    /// Get overdue work orders
    async fn overdue_work_orders(
        &self,
        ctx: &Context<'_>,
        limit: Option<i32>,
    ) -> Result<Vec<WorkOrder>, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        let mut work_orders = repo
            .list::<WorkOrder>(None)
            .await
            .map_err(|e| e.to_graphql_error())?;

        work_orders = work_orders
            .into_iter()
            .filter(|wo| wo.is_overdue())
            .collect();

        // Apply limit if provided
        if let Some(limit_val) = limit {
            work_orders.truncate(limit_val as usize);
        }

        Ok(work_orders)
    }

    /// Get work orders by priority
    async fn work_orders_by_priority(
        &self,
        ctx: &Context<'_>,
        priority: String,
        limit: Option<i32>,
        status_filter: Option<String>,
    ) -> Result<Vec<WorkOrder>, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        let priority_enum = WorkOrderPriority::from_string(&priority)
            .map_err(|e| e.to_graphql_error())?;

        let mut work_orders = repo
            .list::<WorkOrder>(None)
            .await
            .map_err(|e| e.to_graphql_error())?;

        work_orders = work_orders
            .into_iter()
            .filter(|wo| wo.priority.to_string() == priority_enum.to_string())
            .collect();

        // Apply status filter if provided
        if let Some(status) = status_filter {
            let status_enum = WorkOrderStatus::from_string(&status)
                .map_err(|e| e.to_graphql_error())?;
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

    /// Get work orders by location
    async fn work_orders_by_location(
        &self,
        ctx: &Context<'_>,
        location_id: String,
        limit: Option<i32>,
        status_filter: Option<String>,
    ) -> Result<Vec<WorkOrder>, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        let mut work_orders = repo
            .list::<WorkOrder>(None)
            .await
            .map_err(|e| e.to_graphql_error())?;

        work_orders = work_orders
            .into_iter()
            .filter(|wo| wo.asset_location_id == location_id)
            .collect();

        // Apply status filter if provided
        if let Some(status) = status_filter {
            let status_enum = WorkOrderStatus::from_string(&status)
                .map_err(|e| e.to_graphql_error())?;
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

    /// Get work orders that need follow-up
    async fn work_orders_needing_followup(
        &self,
        ctx: &Context<'_>,
        limit: Option<i32>,
    ) -> Result<Vec<WorkOrder>, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        let mut work_orders = repo
            .list::<WorkOrder>(None)
            .await
            .map_err(|e| e.to_graphql_error())?;

        work_orders = work_orders
            .into_iter()
            .filter(|wo| {
                wo.follow_up_required && 
                matches!(wo.status, WorkOrderStatus::Completed) &&
                wo.follow_up_date.map_or(true, |date| date <= Utc::now())
            })
            .collect();

        // Apply limit if provided
        if let Some(limit_val) = limit {
            work_orders.truncate(limit_val as usize);
        }

        Ok(work_orders)
    }
}