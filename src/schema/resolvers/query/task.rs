use async_graphql::*;
use tracing::warn;

use crate::{
    error::AppError,
    models::task::{Task, TaskType},
    DbClient,
    Repository,
};

#[derive(Default, Debug)]
pub(crate) struct TaskQuery;

#[Object]
impl TaskQuery {
    /// Get task by ID
    async fn task_by_id(
        &self,
        ctx: &Context<'_>,
        id: String
    ) -> Result<Option<Task>, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        repo.get::<Task>(id).await.map_err(|e| e.to_graphql_error())
    }

    /// Get all tasks with optional filtering
    async fn tasks(
        &self,
        ctx: &Context<'_>,
        limit: Option<i32>,
        type_filter: Option<String>,
        completed_filter: Option<bool>,
        assigned_to_filter: Option<String>
    ) -> Result<Vec<Task>, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        let mut tasks = repo
            .list::<Task>(limit).await
            .map_err(|e| e.to_graphql_error())?;

        // Apply filters
        if let Some(task_type) = type_filter {
            let type_enum = TaskType::from_string(&task_type).map_err(|e| e.to_graphql_error())?;
            tasks = tasks
                .into_iter()
                .filter(|t| t.task_type == type_enum)
                .collect();
        }

        if let Some(completed) = completed_filter {
            tasks = tasks
                .into_iter()
                .filter(|t| t.completed == completed)
                .collect();
        }

        if let Some(assigned_to) = assigned_to_filter {
            tasks = tasks
                .into_iter()
                .filter(|t| t.assigned_to.as_deref() == Some(&assigned_to))
                .collect();
        }

        Ok(tasks)
    }

    /// Get tasks by assigned user
    async fn tasks_by_assigned_to(
        &self,
        ctx: &Context<'_>,
        user_id: String,
        limit: Option<i32>,
        completed_filter: Option<bool>
    ) -> Result<Vec<Task>, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        let mut tasks = repo.list::<Task>(None).await.map_err(|e| e.to_graphql_error())?;

        tasks = tasks
            .into_iter()
            .filter(|t| t.assigned_to.as_deref() == Some(&user_id))
            .collect();

        if let Some(completed) = completed_filter {
            tasks = tasks
                .into_iter()
                .filter(|t| t.completed == completed)
                .collect();
        }

        if let Some(limit_val) = limit {
            tasks.truncate(limit_val as usize);
        }

        Ok(tasks)
    }

    /// Get tasks by type
    async fn tasks_by_type(
        &self,
        ctx: &Context<'_>,
        task_type: String,
        limit: Option<i32>,
        completed_filter: Option<bool>
    ) -> Result<Vec<Task>, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        let type_enum = TaskType::from_string(&task_type).map_err(|e| e.to_graphql_error())?;

        let mut tasks = repo.list::<Task>(None).await.map_err(|e| e.to_graphql_error())?;

        tasks = tasks
            .into_iter()
            .filter(|t| t.task_type == type_enum)
            .collect();

        if let Some(completed) = completed_filter {
            tasks = tasks
                .into_iter()
                .filter(|t| t.completed == completed)
                .collect();
        }

        if let Some(limit_val) = limit {
            tasks.truncate(limit_val as usize);
        }

        Ok(tasks)
    }

    /// Get tasks by work order ID
    async fn tasks_by_work_order(
        &self,
        ctx: &Context<'_>,
        work_order_id: String,
        limit: Option<i32>,
        completed_filter: Option<bool>
    ) -> Result<Vec<Task>, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        let mut tasks = repo.list::<Task>(None).await.map_err(|e| e.to_graphql_error())?;

        tasks = tasks
            .into_iter()
            .filter(|t| t.work_order_id.as_deref() == Some(&work_order_id))
            .collect();

        if let Some(completed) = completed_filter {
            tasks = tasks
                .into_iter()
                .filter(|t| t.completed == completed)
                .collect();
        }

        if let Some(limit_val) = limit {
            tasks.truncate(limit_val as usize);
        }

        Ok(tasks)
    }
}