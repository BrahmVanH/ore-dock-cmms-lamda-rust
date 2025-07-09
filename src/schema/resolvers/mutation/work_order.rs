use async_graphql::*;
use chrono::{ DateTime, Utc };
use rust_decimal::Decimal;
use tracing::{ info, warn };
use uuid::Uuid;

use crate::{
    DbClient,
    models::{
        work_order::{
            WorkOrder,
            WorkOrderStatus,
            WorkOrderPriority,
            WorkOrderSeverity,
            WorkOrderDifficulty,
            WorkOrderCost,
        },
        asset::Asset,
    },
    AppError,
    Repository,
};

#[derive(Debug, Default)]
pub struct WorkOrderMutation;

#[Object]
impl WorkOrderMutation {
    /// Create a new work order
    async fn create_work_order(
        &self,
        ctx: &Context<'_>,
        work_order_number: String,
        title: String,
        description: String,
        asset_id: String,
        work_order_type: String,
        notes: Option<String>,
        priority: String,
        severity: String,
        difficulty: String,
        assigned_technician_id: Option<String>,
        estimated_duration_minutes: i32,
        estimated_cost: String,
        created_by: String
    ) -> Result<WorkOrder, Error> {
        info!("Creating new work order: {}", title);

        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());
        let id = Uuid::new_v4().to_string();

        // Validate that asset exists
        let _asset = repo
            .get::<Asset>(asset_id.clone()).await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| {
                AppError::ValidationError(
                    format!("Asset {} not found", asset_id)
                ).to_graphql_error()
            })?;

        // Parse severity and difficulty enums
        let severity_enum = WorkOrderSeverity::from_string(&severity).map_err(|e|
            e.to_graphql_error()
        )?;
        let difficulty_enum = WorkOrderDifficulty::from_string(&difficulty).map_err(|e|
            e.to_graphql_error()
        )?;

        // Parse estimated cost enum
        let estimated_cost_enum = WorkOrderCost::from_string(&estimated_cost).map_err(|e|
            e.to_graphql_error()
        )?;

        let work_order = WorkOrder::new(
            id,
            work_order_number,
            title,
            description,
            notes,
            asset_id,
            work_order_type,
            priority,
            severity_enum,
            difficulty_enum,
            assigned_technician_id,
            estimated_duration_minutes,
            estimated_cost_enum,
            created_by
        ).map_err(|e| e.to_graphql_error())?;

        repo.create(work_order).await.map_err(|e| e.to_graphql_error())
    }

    /// Update an existing work order
    async fn update_work_order(
        &self,
        ctx: &Context<'_>,
        id: String,
        work_order_number: Option<String>,
        completed_date: Option<DateTime<Utc>>,
        title: Option<String>,
        notes: Option<String>,
        description: Option<String>,
        priority: Option<String>,
        severity: Option<String>,
        difficulty: Option<String>,
        assigned_technician_id: Option<String>,
        estimated_duration_minutes: Option<i32>,
        estimated_cost: Option<String>
    ) -> Result<WorkOrder, Error> {
        info!("Updating work order: {}", id);

        let db_client = ctx
            .data::<DbClient>()
            .map_err(|_| {
                AppError::InternalServerError("Database client not available".to_string())
            })?;

        let repo = Repository::new(db_client.clone());

        let mut work_order = repo
            .get::<WorkOrder>(id.clone()).await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| AppError::NotFound(format!("Work order {} not found", id)))?;

        // Only allow updates if work order is not completed or cancelled
        if matches!(work_order.status, WorkOrderStatus::Completed | WorkOrderStatus::Cancelled) {
            return Err(
                AppError::ValidationError(
                    "Cannot update completed or cancelled work orders".to_string()
                ).to_graphql_error()
            );
        }

        // Update fields
        if let Some(number) = work_order_number {
            work_order.work_order_number = number;
        }
        if let Some(title) = title {
            work_order.title = title;
        }
        if let Some(description) = description {
            work_order.description = description;
        }

        work_order.notes = notes;
        
        if let Some(priority_str) = priority {
            work_order.priority = WorkOrderPriority::from_string(&priority_str).map_err(|e|
                e.to_graphql_error()
            )?;
        }
        if let Some(severity_str) = severity {
            work_order.severity = WorkOrderSeverity::from_string(&severity_str).map_err(|e|
                e.to_graphql_error()
            )?;
        }
        if let Some(difficulty_str) = difficulty {
            work_order.difficulty = WorkOrderDifficulty::from_string(&difficulty_str).map_err(|e|
                e.to_graphql_error()
            )?;
        }

        if let Some(tech_id) = assigned_technician_id {
            work_order.assigned_technician_id = if tech_id.is_empty() {
                None
            } else {
                Some(tech_id)
            };
        }
        if let Some(duration) = estimated_duration_minutes {
            work_order.estimated_duration_minutes = duration;
        }
        if let Some(cost_str) = estimated_cost {
            work_order.estimated_cost = WorkOrderCost::from_string(&cost_str).map_err(|e|
                e.to_graphql_error()
            )?;
        }
        if let Some(completed_date) = completed_date {
            work_order.completed_date = Some(completed_date);
        }
        work_order.updated_at = Utc::now();

        repo.update(work_order).await.map_err(|e| e.to_graphql_error())
    }

    /// Update work order classification (severity and difficulty)
    async fn update_work_order_classification(
        &self,
        ctx: &Context<'_>,
        id: String,
        severity: String,
        difficulty: String
    ) -> Result<WorkOrder, Error> {
        info!("Updating work order classification: {}", id);

        let db_client = ctx
            .data::<DbClient>()
            .map_err(|_| {
                AppError::InternalServerError("Database client not available".to_string())
            })?;

        let repo = Repository::new(db_client.clone());

        let mut work_order = repo
            .get::<WorkOrder>(id.clone()).await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| AppError::NotFound(format!("Work order {} not found", id)))?;

        let severity_enum = WorkOrderSeverity::from_string(&severity).map_err(|e|
            e.to_graphql_error()
        )?;
        let difficulty_enum = WorkOrderDifficulty::from_string(&difficulty).map_err(|e|
            e.to_graphql_error()
        )?;

        work_order.set_classification(severity_enum, difficulty_enum);

        repo.update(work_order).await.map_err(|e| e.to_graphql_error())
    }

    /// Start a work order
    async fn start_work_order(
        &self,
        ctx: &Context<'_>,
        id: String,
        technician_id: String
    ) -> Result<WorkOrder, Error> {
        info!("Starting work order: {}", id);

        let db_client = ctx
            .data::<DbClient>()
            .map_err(|_| {
                AppError::InternalServerError("Database client not available".to_string())
            })?;

        let repo = Repository::new(db_client.clone());

        let mut work_order = repo
            .get::<WorkOrder>(id.clone()).await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| AppError::NotFound(format!("Work order {} not found", id)))?;

        work_order.start_work(technician_id).map_err(|e| e.to_graphql_error())?;

        repo.update(work_order).await.map_err(|e| e.to_graphql_error())
    }

    /// Complete a work order
    async fn complete_work_order(
        &self,
        ctx: &Context<'_>,
        id: String,
        completion_notes: Option<String>,
        actual_cost: Option<String>,
        labor_hours: Option<f64>
    ) -> Result<WorkOrder, Error> {
        info!("Completing work order: {}", id);

        let db_client = ctx
            .data::<DbClient>()
            .map_err(|_| {
                AppError::InternalServerError("Database client not available".to_string())
            })?;

        let repo = Repository::new(db_client.clone());

        let mut work_order = repo
            .get::<WorkOrder>(id.clone()).await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| AppError::NotFound(format!("Work order {} not found", id)))?;

        // Set additional completion data
        if let Some(cost_str) = actual_cost {
            work_order.actual_cost = Some(
                cost_str
                    .parse::<Decimal>()
                    .map_err(|_| {
                        AppError::ValidationError(
                            "Invalid actual cost format".to_string()
                        ).to_graphql_error()
                    })?
            );
        }
        if let Some(hours) = labor_hours {
            work_order.labor_hours = Some(hours);
        }

        work_order.complete_work(completion_notes).map_err(|e| e.to_graphql_error())?;

        repo.update(work_order).await.map_err(|e| e.to_graphql_error())
    }

    /// Cancel a work order
    async fn cancel_work_order(
        &self,
        ctx: &Context<'_>,
        id: String,
        reason: String
    ) -> Result<WorkOrder, Error> {
        info!("Cancelling work order: {}", id);

        let db_client = ctx
            .data::<DbClient>()
            .map_err(|_| {
                AppError::InternalServerError("Database client not available".to_string())
            })?;

        let repo = Repository::new(db_client.clone());

        let mut work_order = repo
            .get::<WorkOrder>(id.clone()).await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| AppError::NotFound(format!("Work order {} not found", id)))?;

        work_order.cancel_work(reason).map_err(|e| e.to_graphql_error())?;

        repo.update(work_order).await.map_err(|e| e.to_graphql_error())
    }

    /// Put work order on hold
    async fn put_work_order_on_hold(
        &self,
        ctx: &Context<'_>,
        id: String,
        reason: String
    ) -> Result<WorkOrder, Error> {
        info!("Putting work order on hold: {}", id);

        let db_client = ctx
            .data::<DbClient>()
            .map_err(|_| {
                AppError::InternalServerError("Database client not available".to_string())
            })?;

        let repo = Repository::new(db_client.clone());

        let mut work_order = repo
            .get::<WorkOrder>(id.clone()).await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| AppError::NotFound(format!("Work order {} not found", id)))?;

        work_order.put_on_hold(reason).map_err(|e| e.to_graphql_error())?;

        repo.update(work_order).await.map_err(|e| e.to_graphql_error())
    }

    /// Resume work order from hold
    async fn resume_work_order_from_hold(
        &self,
        ctx: &Context<'_>,
        id: String
    ) -> Result<WorkOrder, Error> {
        info!("Resuming work order from hold: {}", id);

        let db_client = ctx
            .data::<DbClient>()
            .map_err(|_| {
                AppError::InternalServerError("Database client not available".to_string())
            })?;

        let repo = Repository::new(db_client.clone());

        let mut work_order = repo
            .get::<WorkOrder>(id.clone()).await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| AppError::NotFound(format!("Work order {} not found", id)))?;

        work_order.resume_from_hold().map_err(|e| e.to_graphql_error())?;

        repo.update(work_order).await.map_err(|e| e.to_graphql_error())
    }

    /// Update work order status
    async fn update_work_order_status(
        &self,
        ctx: &Context<'_>,
        id: String,
        status: String
    ) -> Result<WorkOrder, Error> {
        info!("Updating work order status: {} to {}", id, status);

        let db_client = ctx
            .data::<DbClient>()
            .map_err(|_| {
                AppError::InternalServerError("Database client not available".to_string())
            })?;

        let repo = Repository::new(db_client.clone());

        let mut work_order = repo
            .get::<WorkOrder>(id.clone()).await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| AppError::NotFound(format!("Work order {} not found", id)))?;

        let new_status = WorkOrderStatus::from_string(&status).map_err(|e| e.to_graphql_error())?;

        work_order.status = new_status;
        work_order.updated_at = Utc::now();

        repo.update(work_order).await.map_err(|e| e.to_graphql_error())
    }

    /// Delete a work order
    async fn delete_work_order(&self, ctx: &Context<'_>, id: String) -> Result<bool, Error> {
        info!("Deleting work order: {}", id);

        let db_client = ctx
            .data::<DbClient>()
            .map_err(|_| {
                AppError::InternalServerError("Database client not available".to_string())
            })?;

        let repo = Repository::new(db_client.clone());

        // Verify work order exists
        let work_order = repo
            .get::<WorkOrder>(id.clone()).await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| AppError::NotFound(format!("Work order {} not found", id)))?;

        // Business rules: Check if work order can be deleted
        if matches!(work_order.status, WorkOrderStatus::InProgress) {
            return Err(
                AppError::ValidationError(
                    "Cannot delete work order that is in progress".to_string()
                ).to_graphql_error()
            );
        }

        repo.delete::<WorkOrder>(id).await.map_err(|e| e.to_graphql_error())
    }
}
