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
pub(crate) struct MaintenanceRequestQuery;

#[Object]
impl MaintenanceRequestQuery {
    /// Get maintenance request by ID
    async fn maintenance_request_by_id(
        &self,
        ctx: &Context<'_>,
        id: String
    ) -> Result<Option<MaintenanceRequest>, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        repo.get::<MaintenanceRequest>(id).await.map_err(|e| e.to_graphql_error())
    }

    /// Get all maintenance requests with filtering
    async fn maintenance_requests(
        &self,
        ctx: &Context<'_>,
        limit: Option<i32>,
        status_filter: Option<String>,
        severity_filter: Option<String>,
        submitted_by_filter: Option<String>
    ) -> Result<Vec<MaintenanceRequest>, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        let mut requests = repo
            .list::<MaintenanceRequest>(limit).await
            .map_err(|e| e.to_graphql_error())?;

        // Apply filters
        if let Some(status) = status_filter {
            let status_enum = MaintenanceRequestStatus::from_string(&status).map_err(|e|
                e.to_graphql_error()
            )?;
            requests = requests
                .into_iter()
                .filter(|req| req.status == status_enum)
                .collect();
        }

        if let Some(severity) = severity_filter {
            let severity_enum = WorkOrderSeverity::from_string(&severity).map_err(|e|
                e.to_graphql_error()
            )?;
            requests = requests
                .into_iter()
                .filter(|req| req.severity == severity_enum)
                .collect();
        }

        if let Some(submitted_by) = submitted_by_filter {
            requests = requests
                .into_iter()
                .filter(|req|
                    req.submitted_by.to_lowercase().contains(&submitted_by.to_lowercase())
                )
                .collect();
        }

        // Sort by creation date (newest first)
        requests.sort_by(|a, b| b.created_at.cmp(&a.created_at));

        Ok(requests)
    }

    /// Get maintenance requests by status
    async fn maintenance_requests_by_status(
        &self,
        ctx: &Context<'_>,
        status: String,
        limit: Option<i32>
    ) -> Result<Vec<MaintenanceRequest>, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        let status_enum = MaintenanceRequestStatus::from_string(&status).map_err(|e|
            e.to_graphql_error()
        )?;

        let mut requests = repo
            .list::<MaintenanceRequest>(None).await
            .map_err(|e| e.to_graphql_error())?;

        requests = requests
            .into_iter()
            .filter(|req| req.status == status_enum)
            .collect();

        requests.sort_by(|a, b| b.created_at.cmp(&a.created_at));

        if let Some(limit_val) = limit {
            requests.truncate(limit_val as usize);
        }

        Ok(requests)
    }

    /// Get maintenance requests by severity
    async fn maintenance_requests_by_severity(
        &self,
        ctx: &Context<'_>,
        severity: String,
        limit: Option<i32>,
        status_filter: Option<String>
    ) -> Result<Vec<MaintenanceRequest>, Error> {
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

        let mut requests = repo
            .list::<MaintenanceRequest>(None).await
            .map_err(|e| e.to_graphql_error())?;

        requests = requests
            .into_iter()
            .filter(|req| req.severity == severity_enum)
            .collect();

        // Apply status filter if provided
        if let Some(status) = status_filter {
            let status_enum = MaintenanceRequestStatus::from_string(&status).map_err(|e|
                e.to_graphql_error()
            )?;
            requests = requests
                .into_iter()
                .filter(|req| req.status == status_enum)
                .collect();
        }

        requests.sort_by(|a, b| b.created_at.cmp(&a.created_at));

        if let Some(limit_val) = limit {
            requests.truncate(limit_val as usize);
        }

        Ok(requests)
    }

    /// Get maintenance requests submitted by a specific user
    async fn maintenance_requests_by_submitter(
        &self,
        ctx: &Context<'_>,
        submitted_by: String,
        limit: Option<i32>,
        status_filter: Option<String>
    ) -> Result<Vec<MaintenanceRequest>, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        let mut requests = repo
            .list::<MaintenanceRequest>(None).await
            .map_err(|e| e.to_graphql_error())?;

        requests = requests
            .into_iter()
            .filter(|req| req.submitted_by == submitted_by)
            .collect();

        // Apply status filter if provided
        if let Some(status) = status_filter {
            let status_enum = MaintenanceRequestStatus::from_string(&status).map_err(|e|
                e.to_graphql_error()
            )?;
            requests = requests
                .into_iter()
                .filter(|req| req.status == status_enum)
                .collect();
        }

        requests.sort_by(|a, b| b.created_at.cmp(&a.created_at));

        if let Some(limit_val) = limit {
            requests.truncate(limit_val as usize);
        }

        Ok(requests)
    }

    /// Get unread maintenance requests
    async fn unread_maintenance_requests(
        &self,
        ctx: &Context<'_>,
        limit: Option<i32>
    ) -> Result<Vec<MaintenanceRequest>, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        let mut requests = repo
            .list::<MaintenanceRequest>(None).await
            .map_err(|e| e.to_graphql_error())?;

        requests = requests
            .into_iter()
            .filter(|req| req.status == MaintenanceRequestStatus::Submitted)
            .collect();

        requests.sort_by(|a, b| a.created_at.cmp(&b.created_at)); // Oldest first for unread

        if let Some(limit_val) = limit {
            requests.truncate(limit_val as usize);
        }

        Ok(requests)
    }

    /// Get pending maintenance requests (read but not acted upon)
    async fn pending_maintenance_requests(
        &self,
        ctx: &Context<'_>,
        limit: Option<i32>
    ) -> Result<Vec<MaintenanceRequest>, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        let mut requests = repo
            .list::<MaintenanceRequest>(None).await
            .map_err(|e| e.to_graphql_error())?;

        requests = requests
            .into_iter()
            .filter(|req| req.status == MaintenanceRequestStatus::Read)
            .collect();

        requests.sort_by(|a, b| a.created_at.cmp(&b.created_at)); // Oldest first for pending

        if let Some(limit_val) = limit {
            requests.truncate(limit_val as usize);
        }

        Ok(requests)
    }

    /// Get maintenance requests that have work orders created
    async fn maintenance_requests_with_work_orders(
        &self,
        ctx: &Context<'_>,
        limit: Option<i32>
    ) -> Result<Vec<MaintenanceRequest>, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        let mut requests = repo
            .list::<MaintenanceRequest>(None).await
            .map_err(|e| e.to_graphql_error())?;

        requests = requests
            .into_iter()
            .filter(|req| !req.work_order_ids.is_empty())
            .collect();

        requests.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));

        if let Some(limit_val) = limit {
            requests.truncate(limit_val as usize);
        }

        Ok(requests)
    }

    /// Get maintenance request statistics
    async fn maintenance_request_statistics(
        &self,
        ctx: &Context<'_>
    ) -> Result<MaintenanceRequestStatistics, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        let requests = repo
            .list::<MaintenanceRequest>(None).await
            .map_err(|e| e.to_graphql_error())?;

        let total_requests = requests.len() as i32;
        let submitted_count = requests
            .iter()
            .filter(|r| r.status == MaintenanceRequestStatus::Submitted)
            .count() as i32;
        let read_count = requests
            .iter()
            .filter(|r| r.status == MaintenanceRequestStatus::Read)
            .count() as i32;
        let accepted_count = requests
            .iter()
            .filter(|r| r.status == MaintenanceRequestStatus::Accepted)
            .count() as i32;
        let denied_count = requests
            .iter()
            .filter(|r| r.status == MaintenanceRequestStatus::Denied)
            .count() as i32;
        let archived_count = requests
            .iter()
            .filter(|r| r.status == MaintenanceRequestStatus::Archived)
            .count() as i32;
        let with_work_orders = requests
            .iter()
            .filter(|r| !r.work_order_ids.is_empty())
            .count() as i32;

        let critical_requests = requests
            .iter()
            .filter(|r| r.severity == WorkOrderSeverity::Critical)
            .count() as i32;
        let important_requests = requests
            .iter()
            .filter(|r| r.severity == WorkOrderSeverity::Important)
            .count() as i32;
        let valuable_requests = requests
            .iter()
            .filter(|r| r.severity == WorkOrderSeverity::Valuable)
            .count() as i32;
        let nice_requests = requests
            .iter()
            .filter(|r| r.severity == WorkOrderSeverity::Nice)
            .count() as i32;

        Ok(MaintenanceRequestStatistics {
            total_requests,
            submitted_count,
            read_count,
            accepted_count,
            denied_count,
            archived_count,
            with_work_orders,
            critical_requests,
            important_requests,
            valuable_requests,
            nice_requests,
        })
    }
}

#[derive(Debug)]
pub struct MaintenanceRequestStatistics {
    pub total_requests: i32,
    pub submitted_count: i32,
    pub read_count: i32,
    pub accepted_count: i32,
    pub denied_count: i32,
    pub archived_count: i32,
    pub with_work_orders: i32,
    pub critical_requests: i32,
    pub important_requests: i32,
    pub valuable_requests: i32,
    pub nice_requests: i32,
}

#[Object]
impl MaintenanceRequestStatistics {
    async fn total_requests(&self) -> i32 {
        self.total_requests
    }

    async fn submitted_count(&self) -> i32 {
        self.submitted_count
    }

    async fn read_count(&self) -> i32 {
        self.read_count
    }

    async fn accepted_count(&self) -> i32 {
        self.accepted_count
    }

    async fn denied_count(&self) -> i32 {
        self.denied_count
    }

    async fn archived_count(&self) -> i32 {
        self.archived_count
    }

    async fn with_work_orders(&self) -> i32 {
        self.with_work_orders
    }

    async fn critical_requests(&self) -> i32 {
        self.critical_requests
    }

    async fn important_requests(&self) -> i32 {
        self.important_requests
    }

    async fn valuable_requests(&self) -> i32 {
        self.valuable_requests
    }

    async fn nice_requests(&self) -> i32 {
        self.nice_requests
    }

    // Computed fields
    async fn pending_action_count(&self) -> i32 {
        self.submitted_count + self.read_count
    }

    async fn resolution_rate(&self) -> f64 {
        if self.total_requests == 0 {
            0.0
        } else {
            (((self.accepted_count + self.denied_count) as f64) / (self.total_requests as f64)) *
                100.0
        }
    }

    async fn work_order_conversion_rate(&self) -> f64 {
        if self.accepted_count == 0 {
            0.0
        } else {
            ((self.with_work_orders as f64) / (self.accepted_count as f64)) * 100.0
        }
    }

    async fn critical_percentage(&self) -> f64 {
        if self.total_requests == 0 {
            0.0
        } else {
            ((self.critical_requests as f64) / (self.total_requests as f64)) * 100.0
        }
    }
}
