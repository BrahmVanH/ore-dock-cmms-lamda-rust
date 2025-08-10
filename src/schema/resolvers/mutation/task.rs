use crate::{ DbClient, models::{ prelude::*, task::{ Task, TaskType } }, AppError, Repository };

#[derive(Debug, Default)]
pub struct TaskMutation;

#[Object]
impl TaskMutation {
    /// Create a new task
    pub(crate) async fn create_task(
        &self,
        ctx: &Context<'_>,

        title: String,
        description: String,
        work_order_id: Option<String>,
        task_type: String,
        private: bool,
        assigned_to: Option<String>
    ) -> Result<Task, Error> {
        info!("Creating new task: {}", title);

        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());
        let id = Uuid::new_v4().to_string();
        let latest_tasks = repo
            .list::<Task>(Some(1)).await
            .map_err(|e| e.to_graphql_error())?;
        let next_number = if let Some(latest) = latest_tasks.first() {
            // Assuming work_order_number is numeric
            latest.task_number.parse::<u64>().unwrap_or(0) + 1
        } else {
            1
        };
        let task_number = format!("{:06}", next_number); // e.g., "000123"
        let task = Task::new(
            id,
            task_number,
            title,
            description,
            work_order_id,
            task_type,
            private,
            assigned_to
        ).map_err(|e| e.to_graphql_error())?;
        
        repo.create(task.clone()).await.map_err(|e| e.to_graphql_error())?;
        Ok(task)
    }

    /// Update an existing task
    async fn update_task(
        &self,
        ctx: &Context<'_>,
        id: String,
        task_number: Option<String>,
        title: Option<String>,
        description: Option<String>,
        work_order_id: Option<String>,
        task_type: Option<String>,
        private: Option<bool>,
        assigned_to: Option<String>,
        completed: Option<bool>
    ) -> Result<Task, Error> {
        info!("Updating task: {}", id);

        let db_client = ctx
            .data::<DbClient>()
            .map_err(|_| {
                AppError::InternalServerError("Database client not available".to_string())
            })?;

        let repo = Repository::new(db_client.clone());

        let mut task = repo
            .get::<Task>(id.clone()).await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(||
                AppError::NotFound(format!("Task {} not found", id)).to_graphql_error()
            )?;

        if let Some(number) = task_number {
            task.task_number = number;
        }
        if let Some(title) = title {
            task.title = title;
        }
        if let Some(description) = description {
            task.description = description;
        }

        task.update_work_order_id(work_order_id);

        if let Some(task_type_str) = task_type {
            task.task_type = TaskType::from_string(&task_type_str).map_err(|e|
                e.to_graphql_error()
            )?;
        }
        if let Some(private_val) = private {
            task.private = private_val;
        }
        if let Some(assigned_to_val) = assigned_to {
            task.assign_to(assigned_to_val);
        }
        if let Some(completed_val) = completed {
            if completed_val && !task.completed {
                task.complete_task(task.completed_by.clone());
            } else if !completed_val && task.completed {
                task.completed = false;
                task.completed_by = None;
                task.updated_at = Utc::now();
            }
        }
        task.updated_at = Utc::now();

        repo.update(task.clone()).await.map_err(|e| e.to_graphql_error())?;
        Ok(task)
    }

    /// Assign a task to a user
    async fn assign_task(
        &self,
        ctx: &Context<'_>,
        id: String,
        user_id: String
    ) -> Result<Task, Error> {
        info!("Assigning task {} to user {}", id, user_id);

        let db_client = ctx
            .data::<DbClient>()
            .map_err(|_| {
                AppError::InternalServerError("Database client not available".to_string())
            })?;
        let repo = Repository::new(db_client.clone());

        let mut task = repo
            .get::<Task>(id.clone()).await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(||
                AppError::NotFound(format!("Task {} not found", id)).to_graphql_error()
            )?;

        task.assign_to(user_id);
        repo.update(task.clone()).await.map_err(|e| e.to_graphql_error())?;
        Ok(task)
    }

    /// Unassign a task
    async fn unassign_task(&self, ctx: &Context<'_>, id: String) -> Result<Task, Error> {
        info!("Unassigning task {}", id);

        let db_client = ctx
            .data::<DbClient>()
            .map_err(|_| {
                AppError::InternalServerError("Database client not available".to_string())
            })?;
        let repo = Repository::new(db_client.clone());

        let mut task = repo
            .get::<Task>(id.clone()).await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(||
                AppError::NotFound(format!("Task {} not found", id)).to_graphql_error()
            )?;

        task.unassign();
        repo.update(task.clone()).await.map_err(|e| e.to_graphql_error())?;
        Ok(task)
    }

    /// Complete a task
    async fn complete_task(
        &self,
        ctx: &Context<'_>,
        id: String,
        completed_by: Option<String>
    ) -> Result<Task, Error> {
        info!("Completing task {}", id);

        let db_client = ctx
            .data::<DbClient>()
            .map_err(|_| {
                AppError::InternalServerError("Database client not available".to_string())
            })?;
        let repo = Repository::new(db_client.clone());

        let mut task = repo
            .get::<Task>(id.clone()).await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(||
                AppError::NotFound(format!("Task {} not found", id)).to_graphql_error()
            )?;

        task.complete_task(completed_by);
        repo.update(task.clone()).await.map_err(|e| e.to_graphql_error())?;
        Ok(task)
    }

    /// Delete a task
    async fn delete_task(&self, ctx: &Context<'_>, id: String) -> Result<bool, Error> {
        info!("Deleting task: {}", id);

        let db_client = ctx
            .data::<DbClient>()
            .map_err(|_| {
                AppError::InternalServerError("Database client not available".to_string())
            })?;
        let repo = Repository::new(db_client.clone());

        // Verify task exists
        let _task = repo
            .get::<Task>(id.clone()).await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(||
                AppError::NotFound(format!("Task {} not found", id)).to_graphql_error()
            )?;

        repo.delete::<Task>(id).await.map_err(|e| e.to_graphql_error())
    }
}
