//! GraphQL schema implementation for Task entity.

use async_graphql::*;
use chrono::{DateTime, Utc};
use crate::models::task::{Task, TaskType};

/// GraphQL Object implementation for Task.
#[Object]
impl Task {
    /// Task unique identifier.
    async fn id(&self) -> &str {
        &self.id
    }

    /// Human-readable task number.
    async fn task_number(&self) -> &str {
        &self.task_number
    }

    /// Task title.
    async fn title(&self) -> &str {
        &self.title
    }

    /// Detailed task description.
    async fn description(&self) -> &str {
        &self.description
    }

    /// Associated work order ID, if applicable.
    async fn work_order_id(&self) -> Option<&str> {
        self.work_order_id.as_deref()
    }

    /// Task type as string representation.
    async fn task_type(&self) -> TaskType {
        self.task_type
    }

    /// Whether the task is private.
    async fn private(&self) -> bool {
        self.private
    }

    /// Whether the task is completed.
    async fn completed(&self) -> bool {
        self.completed
    }

    /// User ID assigned to the task.
    async fn assigned_to(&self) -> Option<&str> {
        self.assigned_to.as_deref()
    }

    /// User ID who completed the task.
    async fn completed_by(&self) -> Option<&str> {
        self.completed_by.as_deref()
    }

    /// Creation timestamp.
    async fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }

    /// Last modification timestamp.
    async fn updated_at(&self) -> &DateTime<Utc> {
        &self.updated_at
    }

    /// Computed field: checks if task is completed.
    #[graphql(name = "is_completed")]
    async fn check_is_completed(&self) -> bool {
        self.is_completed()
    }
}