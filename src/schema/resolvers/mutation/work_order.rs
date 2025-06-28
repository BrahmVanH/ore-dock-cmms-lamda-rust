use async_graphql::*;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde_json::Value as Json;
use tracing::{info, warn};
use uuid::Uuid;

use crate::{
    DbClient,
    models::{
        work_order::{WorkOrder, WorkOrderStatus, WorkOrderPriority, WorkOrderType},
        asset::Asset,
        location::Location,
        // user::User, // Uncomment when you have user model
    },
    AppError,
    Repository,
};

#[derive(Debug, Default)]
pub struct WorkOrderMutationRoot;

#[Object]
impl WorkOrderMutationRoot {
    /// Create a new work order
    async fn create_work_order(
        &self,
        ctx: &Context<'_>,
        work_order_number: String,
        title: String,
        description: String,
        task_id: Option<String>,
        asset_id: String,
        work_order_type: String,
        priority: String,
        assigned_technician_id: Option<String>,
        assigned_team_ids: Option<Vec<String>>,
        requested_by_user_id: String,
        scheduled_start: DateTime<Utc>,
        scheduled_end: Option<DateTime<Utc>>,
        estimated_duration_minutes: i32,
        estimated_cost: String, // Will be parsed to Decimal
        tools_required: Option<Vec<String>>,
        safety_requirements: Option<Vec<String>>,
        follow_up_required: Option<bool>,
        follow_up_date: Option<DateTime<Utc>>,
        attachment_urls: Option<Vec<String>>,
        tags: Option<Vec<String>>,
        custom_fields: Option<String>, // JSON string
        created_by: String,
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

        // Validate that asset exists and get its location
        let asset = repo.get::<Asset>(asset_id.clone())
            .await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| {
                AppError::ValidationError(format!("Asset {} not found", asset_id))
                    .to_graphql_error()
            })?;

        let asset_location_id = asset.location_id.clone();

        // Validate requesting user exists (when user model is available)
        // repo.get::<User>(requested_by_user_id.clone())
        //     .await
        //     .map_err(|e| e.to_graphql_error())?
        //     .ok_or_else(|| {
        //         AppError::ValidationError(format!("User {} not found", requested_by_user_id))
        //             .to_graphql_error()
        //     })?;

        // Validate technician exists if assigned (when user model is available)
        // if let Some(ref tech_id) = assigned_technician_id {
        //     repo.get::<User>(tech_id.clone())
        //         .await
        //         .map_err(|e| e.to_graphql_error())?
        //         .ok_or_else(|| {
        //             AppError::ValidationError(format!("Technician {} not found", tech_id))
        //                 .to_graphql_error()
        //         })?;
        // }

        // Parse estimated cost
        let estimated_cost_decimal = estimated_cost.parse::<Decimal>()
            .map_err(|_| {
                AppError::ValidationError("Invalid estimated cost format".to_string())
                    .to_graphql_error()
            })?;

        // Parse custom fields if provided
        let custom_fields_json = if let Some(ref cf) = custom_fields {
            Some(serde_json::from_str::<Json>(cf).map_err(|_| {
                AppError::ValidationError("Invalid custom fields JSON".to_string())
                    .to_graphql_error()
            })?)
        } else {
            None
        };

        let work_order = WorkOrder::new(
            id,
            work_order_number,
            title,
            description,
            task_id,
            asset_id,
            asset_location_id,
            work_order_type,
            priority,
            assigned_technician_id,
            assigned_team_ids.unwrap_or_default(),
            requested_by_user_id,
            scheduled_start,
            scheduled_end,
            estimated_duration_minutes,
            estimated_cost_decimal,
            tools_required.unwrap_or_default(),
            safety_requirements.unwrap_or_default(),
            follow_up_required.unwrap_or(false),
            follow_up_date,
            attachment_urls.unwrap_or_default(),
            tags.unwrap_or_default(),
            custom_fields_json,
            created_by,
        ).map_err(|e| e.to_graphql_error())?;

        repo.create(work_order)
            .await
            .map_err(|e| e.to_graphql_error())
    }

    /// Update an existing work order
    async fn update_work_order(
        &self,
        ctx: &Context<'_>,
        id: String,
        work_order_number: Option<String>,
        title: Option<String>,
        description: Option<String>,
        priority: Option<String>,
        assigned_technician_id: Option<String>,
        assigned_team_ids: Option<Vec<String>>,
        scheduled_start: Option<DateTime<Utc>>,
        scheduled_end: Option<DateTime<Utc>>,
        estimated_duration_minutes: Option<i32>,
        estimated_cost: Option<String>,
        tools_required: Option<Vec<String>>,
        safety_requirements: Option<Vec<String>>,
        follow_up_required: Option<bool>,
        follow_up_date: Option<DateTime<Utc>>,
        tags: Option<Vec<String>>,
        custom_fields: Option<String>,
    ) -> Result<WorkOrder, Error> {
        info!("Updating work order: {}", id);

        let db_client = ctx.data::<DbClient>().map_err(|_| {
            AppError::InternalServerError("Database client not available".to_string())
        })?;

        let repo = Repository::new(db_client.clone());

        let mut work_order = repo
            .get::<WorkOrder>(id.clone())
            .await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| AppError::NotFound(format!("Work order {} not found", id)))?;

        // Only allow updates if work order is not completed or cancelled
        if matches!(work_order.status, WorkOrderStatus::Completed | WorkOrderStatus::Cancelled) {
            return Err(AppError::ValidationError(
                "Cannot update completed or cancelled work orders".to_string()
            ).to_graphql_error());
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
        if let Some(priority_str) = priority {
            work_order.priority = WorkOrderPriority::from_string(&priority_str)
                .map_err(|e| e.to_graphql_error())?;
        }
        if let Some(tech_id) = assigned_technician_id {
            work_order.assigned_technician_id = if tech_id.is_empty() { None } else { Some(tech_id) };
        }
        if let Some(team_ids) = assigned_team_ids {
            work_order.assigned_team_ids = team_ids;
        }
        if let Some(start) = scheduled_start {
            work_order.scheduled_start = start;
        }
        if let Some(end) = scheduled_end {
            work_order.scheduled_end = Some(end);
        }
        if let Some(duration) = estimated_duration_minutes {
            work_order.estimated_duration_minutes = duration;
        }
        if let Some(cost_str) = estimated_cost {
            work_order.estimated_cost = cost_str.parse::<Decimal>()
                .map_err(|_| {
                    AppError::ValidationError("Invalid estimated cost format".to_string())
                        .to_graphql_error()
                })?;
        }
        if let Some(tools) = tools_required {
            work_order.tools_required = tools;
        }
        if let Some(safety) = safety_requirements {
            work_order.safety_requirements = safety;
        }
        if let Some(follow_up) = follow_up_required {
            work_order.follow_up_required = follow_up;
        }
        if let Some(follow_date) = follow_up_date {
            work_order.follow_up_date = Some(follow_date);
        }
        if let Some(tags_list) = tags {
            work_order.tags = tags_list;
        }
        if let Some(cf) = custom_fields {
            work_order.custom_fields = Some(serde_json::from_str::<Json>(&cf).map_err(|_| {
                AppError::ValidationError("Invalid custom fields JSON".to_string())
                    .to_graphql_error()
            })?);
        }
        work_order.updated_at = Utc::now();

        repo.update(work_order).await.map_err(|e| e.to_graphql_error())
    }

    /// Start a work order
    async fn start_work_order(
        &self,
        ctx: &Context<'_>,
        id: String,
        technician_id: String,
    ) -> Result<WorkOrder, Error> {
        info!("Starting work order: {}", id);

        let db_client = ctx.data::<DbClient>().map_err(|_| {
            AppError::InternalServerError("Database client not available".to_string())
        })?;

        let repo = Repository::new(db_client.clone());

        let mut work_order = repo
            .get::<WorkOrder>(id.clone())
            .await
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
        quality_rating: Option<i32>,
        actual_cost: Option<String>,
        labor_hours: Option<f64>,
        parts_used: Option<String>, // JSON string
    ) -> Result<WorkOrder, Error> {
        info!("Completing work order: {}", id);

        let db_client = ctx.data::<DbClient>().map_err(|_| {
            AppError::InternalServerError("Database client not available".to_string())
        })?;

        let repo = Repository::new(db_client.clone());

        let mut work_order = repo
            .get::<WorkOrder>(id.clone())
            .await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| AppError::NotFound(format!("Work order {} not found", id)))?;

        // Set additional completion data
        if let Some(cost_str) = actual_cost {
            work_order.actual_cost = Some(cost_str.parse::<Decimal>()
                .map_err(|_| {
                    AppError::ValidationError("Invalid actual cost format".to_string())
                        .to_graphql_error()
                })?);
        }
        if let Some(hours) = labor_hours {
            work_order.labor_hours = Some(hours);
        }
        if let Some(parts_str) = parts_used {
            work_order.parts_used = Some(serde_json::from_str::<Json>(&parts_str).map_err(|_| {
                AppError::ValidationError("Invalid parts used JSON".to_string())
                    .to_graphql_error()
            })?);
        }

        work_order.complete_work(completion_notes, quality_rating)
            .map_err(|e| e.to_graphql_error())?;

        repo.update(work_order).await.map_err(|e| e.to_graphql_error())
    }

    /// Cancel a work order
    async fn cancel_work_order(
        &self,
        ctx: &Context<'_>,
        id: String,
        reason: String,
    ) -> Result<WorkOrder, Error> {
        info!("Cancelling work order: {}", id);

        let db_client = ctx.data::<DbClient>().map_err(|_| {
            AppError::InternalServerError("Database client not available".to_string())
        })?;

        let repo = Repository::new(db_client.clone());

        let mut work_order = repo
            .get::<WorkOrder>(id.clone())
            .await
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
        reason: String,
    ) -> Result<WorkOrder, Error> {
        info!("Putting work order on hold: {}", id);

        let db_client = ctx.data::<DbClient>().map_err(|_| {
            AppError::InternalServerError("Database client not available".to_string())
        })?;

        let repo = Repository::new(db_client.clone());

        let mut work_order = repo
            .get::<WorkOrder>(id.clone())
            .await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| AppError::NotFound(format!("Work order {} not found", id)))?;

        work_order.put_on_hold(reason).map_err(|e| e.to_graphql_error())?;

        repo.update(work_order).await.map_err(|e| e.to_graphql_error())
    }

    /// Resume work order from hold
    async fn resume_work_order_from_hold(
        &self,
        ctx: &Context<'_>,
        id: String,
    ) -> Result<WorkOrder, Error> {
        info!("Resuming work order from hold: {}", id);

        let db_client = ctx.data::<DbClient>().map_err(|_| {
            AppError::InternalServerError("Database client not available".to_string())
        })?;

        let repo = Repository::new(db_client.clone());

        let mut work_order = repo
            .get::<WorkOrder>(id.clone())
            .await
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
        status: String,
    ) -> Result<WorkOrder, Error> {
        info!("Updating work order status: {} to {}", id, status);

        let db_client = ctx.data::<DbClient>().map_err(|_| {
            AppError::InternalServerError("Database client not available".to_string())
        })?;

        let repo = Repository::new(db_client.clone());

        let mut work_order = repo
            .get::<WorkOrder>(id.clone())
            .await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| AppError::NotFound(format!("Work order {} not found", id)))?;

        let new_status = WorkOrderStatus::from_string(&status)
            .map_err(|e| e.to_graphql_error())?;

        work_order.status = new_status;
        work_order.updated_at = Utc::now();

        repo.update(work_order).await.map_err(|e| e.to_graphql_error())
    }

    /// Add tag to work order
    async fn add_tag_to_work_order(
        &self,
        ctx: &Context<'_>,
        id: String,
        tag: String,
    ) -> Result<WorkOrder, Error> {
        info!("Adding tag {} to work order {}", tag, id);

        let db_client = ctx.data::<DbClient>().map_err(|_| {
            AppError::InternalServerError("Database client not available".to_string())
        })?;

        let repo = Repository::new(db_client.clone());

        let mut work_order = repo
            .get::<WorkOrder>(id.clone())
            .await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| AppError::NotFound(format!("Work order {} not found", id)))?;

        work_order.add_tag(tag);

        repo.update(work_order).await.map_err(|e| e.to_graphql_error())
    }

    /// Remove tag from work order
    async fn remove_tag_from_work_order(
        &self,
        ctx: &Context<'_>,
        id: String,
        tag: String,
    ) -> Result<WorkOrder, Error> {
        info!("Removing tag {} from work order {}", tag, id);

        let db_client = ctx.data::<DbClient>().map_err(|_| {
            AppError::InternalServerError("Database client not available".to_string())
        })?;

        let repo = Repository::new(db_client.clone());

        let mut work_order = repo
            .get::<WorkOrder>(id.clone())
            .await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| AppError::NotFound(format!("Work order {} not found", id)))?;

        work_order.remove_tag(&tag);

        repo.update(work_order).await.map_err(|e| e.to_graphql_error())
    }

    /// Schedule work order
    async fn schedule_work_order(
        &self,
        ctx: &Context<'_>,
        id: String,
        scheduled_start: DateTime<Utc>,
        scheduled_end: Option<DateTime<Utc>>,
    ) -> Result<WorkOrder, Error> {
        info!("Scheduling work order: {}", id);

        let db_client = ctx.data::<DbClient>().map_err(|_| {
            AppError::InternalServerError("Database client not available".to_string())
        })?;

        let repo = Repository::new(db_client.clone());

        let mut work_order = repo
            .get::<WorkOrder>(id.clone())
            .await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| AppError::NotFound(format!("Work order {} not found", id)))?;

        // Validate dates
        if let Some(end) = scheduled_end {
            if end <= scheduled_start {
                return Err(AppError::ValidationError(
                    "Scheduled end must be after scheduled start".to_string()
                ).to_graphql_error());
            }
        }

        work_order.scheduled_start = scheduled_start;
        work_order.scheduled_end = scheduled_end;
        work_order.status = WorkOrderStatus::Scheduled;
        work_order.updated_at = Utc::now();

        repo.update(work_order).await.map_err(|e| e.to_graphql_error())
    }

    /// Delete a work order
    async fn delete_work_order(&self, ctx: &Context<'_>, id: String) -> Result<bool, Error> {
        info!("Deleting work order: {}", id);

        let db_client = ctx.data::<DbClient>().map_err(|_| {
            AppError::InternalServerError("Database client not available".to_string())
        })?;

        let repo = Repository::new(db_client.clone());

        // Verify work order exists
        let work_order = repo
            .get::<WorkOrder>(id.clone())
            .await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| AppError::NotFound(format!("Work order {} not found", id)))?;

        // Business rules: Check if work order can be deleted
        if matches!(work_order.status, WorkOrderStatus::InProgress) {
            return Err(AppError::ValidationError(
                "Cannot delete work order that is in progress".to_string()
            ).to_graphql_error());
        }

        repo.delete::<WorkOrder>(id).await.map_err(|e| e.to_graphql_error())
    }
}