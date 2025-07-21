use async_graphql::*;
use tracing::warn;

use crate::{
    error::AppError,
    models::{
        maintenance_request::{ MaintenanceRequest, MaintenanceRequestStatus },
        work_order::WorkOrderSeverity,
    },
    DbClient,
    Repository,
};

#[derive(Default, Debug)]
pub(crate) struct MaintenanceRequestMutation;

#[derive(InputObject)]
pub struct CreateMaintenanceRequestInput {
    pub submitted_by: String,
    pub manager_on_site: String,
    pub description: String,
    pub reported_location: String,
    pub troubleshooting_performed: String,
    pub severity: WorkOrderSeverity,
}

#[derive(InputObject)]
pub struct UpdateMaintenanceRequestInput {
    pub id: String,
    pub submitted_by: Option<String>,
    pub manager_on_site: Option<String>,
    pub description: Option<String>,
    pub reported_location: Option<String>,
    pub troubleshooting_performed: Option<String>,
    pub severity: Option<WorkOrderSeverity>,
    pub status: Option<MaintenanceRequestStatus>,
    pub read_by_id: Option<String>,
}

#[derive(InputObject)]
pub struct UpdateMaintenanceRequestStatusInput {
    pub id: String,
    pub status: MaintenanceRequestStatus,
    pub read_by_id: Option<String>,
}

#[derive(InputObject)]
pub struct AddWorkOrderToRequestInput {
    pub maintenance_request_id: String,
    pub work_order_id: String,
}

#[derive(InputObject)]
pub struct RemoveWorkOrderFromRequestInput {
    pub maintenance_request_id: String,
    pub work_order_id: String,
}

#[Object]
impl MaintenanceRequestMutation {
    /// Create a new maintenance request
    async fn create_maintenance_request(
        &self,
        ctx: &Context<'_>,
        input: CreateMaintenanceRequestInput,
    ) -> Result<MaintenanceRequest, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        // Generate a unique ID for the maintenance request
        let id = format!("mr-{}", uuid::Uuid::new_v4());

        let maintenance_request = MaintenanceRequest::new(
            id,
            input.submitted_by,
            input.manager_on_site,
            input.description,
            input.reported_location,
            input.troubleshooting_performed,
            input.severity,
        ).map_err(|e| e.to_graphql_error())?;

        repo.create(maintenance_request.clone()).await.map_err(|e| e.to_graphql_error())?;

        Ok(maintenance_request)
    }

    /// Update an existing maintenance request
    async fn update_maintenance_request(
        &self,
        ctx: &Context<'_>,
        input: UpdateMaintenanceRequestInput,
    ) -> Result<MaintenanceRequest, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        // Get the existing maintenance request
        let mut maintenance_request = repo
            .get::<MaintenanceRequest>(input.id.clone()).await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| {
                AppError::NotFound(format!("Maintenance request with id {} not found", input.id))
                    .to_graphql_error()
            })?;

        // Update fields if provided
        if let Some(submitted_by) = input.submitted_by {
            if submitted_by.trim().is_empty() {
                return Err(AppError::ValidationError("submitted_by cannot be empty".to_string())
                    .to_graphql_error());
            }
            maintenance_request.submitted_by = submitted_by;
        }

        if let Some(manager_on_site) = input.manager_on_site {
            if manager_on_site.trim().is_empty() {
                return Err(AppError::ValidationError("manager_on_site cannot be empty".to_string())
                    .to_graphql_error());
            }
            maintenance_request.manager_on_site = manager_on_site;
        }

        if let Some(description) = input.description {
            if description.trim().is_empty() {
                return Err(AppError::ValidationError("description cannot be empty".to_string())
                    .to_graphql_error());
            }
            maintenance_request.description = description;
        }

        if let Some(reported_location) = input.reported_location {
            if reported_location.trim().is_empty() {
                return Err(AppError::ValidationError("reported_location cannot be empty".to_string())
                    .to_graphql_error());
            }
            maintenance_request.reported_location = reported_location;
        }

        if let Some(troubleshooting_performed) = input.troubleshooting_performed {
            if troubleshooting_performed.trim().is_empty() {
                return Err(AppError::ValidationError("troubleshooting_performed cannot be empty".to_string())
                    .to_graphql_error());
            }
            maintenance_request.troubleshooting_performed = troubleshooting_performed;
        }

        if let Some(severity) = input.severity {
            maintenance_request.severity = severity;
        }

        if let Some(status) = input.status {
            maintenance_request.status = status;
        }

        if let Some(read_by_id) = input.read_by_id {
            maintenance_request.read_by_id = Some(read_by_id);
        }

        // Update the timestamp
        maintenance_request.updated_at = chrono::Utc::now();

        repo.update(maintenance_request.clone()).await.map_err(|e| e.to_graphql_error())?;

        Ok(maintenance_request)
    }

    /// Update maintenance request status
    async fn update_maintenance_request_status(
        &self,
        ctx: &Context<'_>,
        input: UpdateMaintenanceRequestStatusInput,
    ) -> Result<MaintenanceRequest, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        // Get the existing maintenance request
        let mut maintenance_request = repo
            .get::<MaintenanceRequest>(input.id.clone()).await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| {
                AppError::NotFound(format!("Maintenance request with id {} not found", input.id))
                    .to_graphql_error()
            })?;

        // Update status
        maintenance_request.status = input.status;

        // If marking as read and read_by_id is provided, set it
        if input.status == MaintenanceRequestStatus::Read {
            if let Some(read_by_id) = input.read_by_id {
                maintenance_request.read_by_id = Some(read_by_id);
            }
        }

        // Update the timestamp
        maintenance_request.updated_at = chrono::Utc::now();

        repo.update(maintenance_request.clone()).await.map_err(|e| e.to_graphql_error())?;

        Ok(maintenance_request)
    }

    /// Mark maintenance request as read
    async fn mark_maintenance_request_as_read(
        &self,
        ctx: &Context<'_>,
        id: String,
        read_by_id: String,
    ) -> Result<MaintenanceRequest, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        // Get the existing maintenance request
        let mut maintenance_request = repo
            .get::<MaintenanceRequest>(id.clone()).await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| {
                AppError::NotFound(format!("Maintenance request with id {} not found", id))
                    .to_graphql_error()
            })?;

        // Use the model's method to mark as read
        maintenance_request.mark_as_read(read_by_id).map_err(|e| e.to_graphql_error())?;

        // Update the timestamp
        maintenance_request.updated_at = chrono::Utc::now();

        repo.update(maintenance_request.clone()).await.map_err(|e| e.to_graphql_error())?;

        Ok(maintenance_request)
    }

    /// Archive maintenance request
    async fn archive_maintenance_request(
        &self,
        ctx: &Context<'_>,
        id: String,
    ) -> Result<MaintenanceRequest, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        // Get the existing maintenance request
        let mut maintenance_request = repo
            .get::<MaintenanceRequest>(id.clone()).await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| {
                AppError::NotFound(format!("Maintenance request with id {} not found", id))
                    .to_graphql_error()
            })?;

        // Use the model's method to archive
        maintenance_request.archive().map_err(|e| e.to_graphql_error())?;

        repo.update(maintenance_request.clone()).await.map_err(|e| e.to_graphql_error())?;

        Ok(maintenance_request)
    }

    /// Add work order to maintenance request
    async fn add_work_order_to_maintenance_request(
        &self,
        ctx: &Context<'_>,
        input: AddWorkOrderToRequestInput,
    ) -> Result<MaintenanceRequest, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        // Get the existing maintenance request
        let mut maintenance_request = repo
            .get::<MaintenanceRequest>(input.maintenance_request_id.clone()).await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| {
                AppError::NotFound(format!(
                    "Maintenance request with id {} not found", 
                    input.maintenance_request_id
                )).to_graphql_error()
            })?;

        // Check if work order ID is already in the list
        if !maintenance_request.work_order_ids.contains(&input.work_order_id) {
            maintenance_request.work_order_ids.push(input.work_order_id);
        }

        // If this is the first work order being added and status is submitted/read, mark as accepted
        if maintenance_request.work_order_ids.len() == 1 && 
           (maintenance_request.status == MaintenanceRequestStatus::Submitted || 
            maintenance_request.status == MaintenanceRequestStatus::Read) {
            maintenance_request.status = MaintenanceRequestStatus::Accepted;
        }

        // Update the timestamp
        maintenance_request.updated_at = chrono::Utc::now();

        repo.update(maintenance_request.clone()).await.map_err(|e| e.to_graphql_error())?;

        Ok(maintenance_request)
    }

    /// Remove work order from maintenance request
    async fn remove_work_order_from_maintenance_request(
        &self,
        ctx: &Context<'_>,
        input: RemoveWorkOrderFromRequestInput,
    ) -> Result<MaintenanceRequest, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        // Get the existing maintenance request
        let mut maintenance_request = repo
            .get::<MaintenanceRequest>(input.maintenance_request_id.clone()).await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| {
                AppError::NotFound(format!(
                    "Maintenance request with id {} not found", 
                    input.maintenance_request_id
                )).to_graphql_error()
            })?;

        // Remove the work order ID from the list
        maintenance_request.work_order_ids.retain(|id| id != &input.work_order_id);

        // Update the timestamp
        maintenance_request.updated_at = chrono::Utc::now();

        repo.update(maintenance_request.clone()).await.map_err(|e| e.to_graphql_error())?;

        Ok(maintenance_request)
    }

    /// Delete maintenance request
    async fn delete_maintenance_request(
        &self,
        ctx: &Context<'_>,
        id: String,
    ) -> Result<bool, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        // Check if the maintenance request exists
        let maintenance_request = repo
            .get::<MaintenanceRequest>(id.clone()).await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| {
                AppError::NotFound(format!("Maintenance request with id {} not found", id))
                    .to_graphql_error()
            })?;

        // Only allow deletion of requests that haven't been accepted or have work orders
        if maintenance_request.status == MaintenanceRequestStatus::Accepted && 
           !maintenance_request.work_order_ids.is_empty() {
            return Err(AppError::ValidationError(
                "Cannot delete maintenance request that has been accepted and has associated work orders".to_string()
            ).to_graphql_error());
        }

        repo.delete::<MaintenanceRequest>(id).await.map_err(|e| e.to_graphql_error())?;

        Ok(true)
    }

    /// Accept maintenance request
    async fn accept_maintenance_request(
        &self,
        ctx: &Context<'_>,
        id: String,
    ) -> Result<MaintenanceRequest, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        // Get the existing maintenance request
        let mut maintenance_request = repo
            .get::<MaintenanceRequest>(id.clone()).await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| {
                AppError::NotFound(format!("Maintenance request with id {} not found", id))
                    .to_graphql_error()
            })?;

        // Only allow accepting requests that are in Read status
        if maintenance_request.status != MaintenanceRequestStatus::Read {
            return Err(AppError::ValidationError(
                "Only read maintenance requests can be accepted".to_string()
            ).to_graphql_error());
        }

        maintenance_request.status = MaintenanceRequestStatus::Accepted;
        maintenance_request.updated_at = chrono::Utc::now();

        repo.update(maintenance_request.clone()).await.map_err(|e| e.to_graphql_error())?;

        Ok(maintenance_request)
    }

    /// Deny maintenance request
    async fn deny_maintenance_request(
        &self,
        ctx: &Context<'_>,
        id: String,
    ) -> Result<MaintenanceRequest, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        // Get the existing maintenance request
        let mut maintenance_request = repo
            .get::<MaintenanceRequest>(id.clone()).await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| {
                AppError::NotFound(format!("Maintenance request with id {} not found", id))
                    .to_graphql_error()
            })?;

        // Only allow denying requests that are in Read status
        if maintenance_request.status != MaintenanceRequestStatus::Read {
            return Err(AppError::ValidationError(
                "Only read maintenance requests can be denied".to_string()
            ).to_graphql_error());
        }

        maintenance_request.status = MaintenanceRequestStatus::Denied;
        maintenance_request.updated_at = chrono::Utc::now();

        repo.update(maintenance_request.clone()).await.map_err(|e| e.to_graphql_error())?;

        Ok(maintenance_request)
    }
}
