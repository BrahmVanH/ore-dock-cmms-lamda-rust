use async_graphql::*;
use chrono::{ DateTime, Utc };
use tracing::warn;

use crate::{
    error::AppError,
    models::{ maintenance_schedule::{ MaintenanceSchedule, CadenceUnit }, asset::Asset },
    DbClient,
    Repository,
};

#[derive(Debug, Default)]
pub(crate) struct MaintenanceScheduleQuery;

#[Object]
impl MaintenanceScheduleQuery {
    async fn maintenance_schedule_by_id(
        &self,
        ctx: &Context<'_>,
        id: String
    ) -> Result<Option<MaintenanceSchedule>, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        repo.get::<MaintenanceSchedule>(id).await.map_err(|e| e.to_graphql_error())
    }

    async fn maintenance_schedules(
        &self,
        ctx: &Context<'_>,
        limit: Option<i32>,
        active_only: Option<bool>,
        recurring_only: Option<bool>
    ) -> Result<Vec<MaintenanceSchedule>, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        let mut schedules = repo
            .list::<MaintenanceSchedule>(limit).await
            .map_err(|e| e.to_graphql_error())?;

        if let Some(true) = active_only {
            schedules = schedules
                .into_iter()
                .filter(|s| s.active)
                .collect();
        }

        if let Some(true) = recurring_only {
            schedules = schedules
                .into_iter()
                .filter(|s| s.recurring)
                .collect();
        }

        Ok(schedules)
    }

    async fn maintenance_schedules_for_asset(
        &self,
        ctx: &Context<'_>,
        asset_id: String,
        active_only: Option<bool>,
        limit: Option<i32>
    ) -> Result<Vec<MaintenanceSchedule>, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        let _asset = repo
            .get::<Asset>(asset_id.clone()).await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| {
                AppError::NotFound(format!("Asset {} not found", asset_id)).to_graphql_error()
            })?;

        let mut schedules = repo
            .list::<MaintenanceSchedule>(None).await
            .map_err(|e| e.to_graphql_error())?;

        schedules = schedules
            .into_iter()
            .filter(|s| s.asset_id == asset_id)
            .collect();

        if let Some(true) = active_only {
            schedules = schedules
                .into_iter()
                .filter(|s| s.active)
                .collect();
        }

        schedules.sort_by(|a, b| a.next_due_at.cmp(&b.next_due_at));

        if let Some(limit_val) = limit {
            schedules.truncate(limit_val as usize);
        }

        Ok(schedules)
    }

    async fn overdue_maintenance_schedules(
        &self,
        ctx: &Context<'_>,
        limit: Option<i32>
    ) -> Result<Vec<MaintenanceSchedule>, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        let mut schedules = repo
            .list::<MaintenanceSchedule>(None).await
            .map_err(|e| e.to_graphql_error())?;

        let now = Utc::now();

        schedules = schedules
            .into_iter()
            .filter(|s| s.active && s.next_due_at < now)
            .collect();

        schedules.sort_by(|a, b| a.next_due_at.cmp(&b.next_due_at));

        if let Some(limit_val) = limit {
            schedules.truncate(limit_val as usize);
        }

        Ok(schedules)
    }

    async fn upcoming_maintenance_schedules(
        &self,
        ctx: &Context<'_>,
        days_ahead: Option<i32>,
        limit: Option<i32>
    ) -> Result<Vec<MaintenanceSchedule>, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        let mut schedules = repo
            .list::<MaintenanceSchedule>(None).await
            .map_err(|e| e.to_graphql_error())?;

        let days_ahead = days_ahead.unwrap_or(30);
        let cutoff_date = Utc::now() + chrono::Duration::days(days_ahead as i64);
        let now = Utc::now();

        schedules = schedules
            .into_iter()
            .filter(|s| s.active && s.next_due_at >= now && s.next_due_at <= cutoff_date)
            .collect();

        schedules.sort_by(|a, b| a.next_due_at.cmp(&b.next_due_at));

        if let Some(limit_val) = limit {
            schedules.truncate(limit_val as usize);
        }

        Ok(schedules)
    }

    async fn recurring_maintenance_schedules(
        &self,
        ctx: &Context<'_>,
        active_only: Option<bool>,
        limit: Option<i32>
    ) -> Result<Vec<MaintenanceSchedule>, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        let mut schedules = repo
            .list::<MaintenanceSchedule>(None).await
            .map_err(|e| e.to_graphql_error())?;

        schedules = schedules
            .into_iter()
            .filter(|s| s.recurring)
            .collect();

        if let Some(true) = active_only {
            schedules = schedules
                .into_iter()
                .filter(|s| s.active)
                .collect();
        }

        if let Some(limit_val) = limit {
            schedules.truncate(limit_val as usize);
        }

        Ok(schedules)
    }

    async fn maintenance_schedules_by_cadence_unit(
        &self,
        ctx: &Context<'_>,
        cadence_unit: String,
        active_only: Option<bool>,
        limit: Option<i32>
    ) -> Result<Vec<MaintenanceSchedule>, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        let cadence_unit_enum = CadenceUnit::from_string(&cadence_unit).map_err(|e|
            e.to_graphql_error()
        )?;

        let mut schedules = repo
            .list::<MaintenanceSchedule>(None).await
            .map_err(|e| e.to_graphql_error())?;

        schedules = schedules
            .into_iter()
            .filter(|s|
                s.cadences
                    .iter()
                    .any(
                        |c|
                            std::mem::discriminant(&c.unit) ==
                            std::mem::discriminant(&cadence_unit_enum)
                    )
            )
            .collect();

        if let Some(true) = active_only {
            schedules = schedules
                .into_iter()
                .filter(|s| s.active)
                .collect();
        }

        if let Some(limit_val) = limit {
            schedules.truncate(limit_val as usize);
        }

        Ok(schedules)
    }

    async fn never_completed_maintenance_schedules(
        &self,
        ctx: &Context<'_>,
        active_only: Option<bool>,
        limit: Option<i32>
    ) -> Result<Vec<MaintenanceSchedule>, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        let mut schedules = repo
            .list::<MaintenanceSchedule>(None).await
            .map_err(|e| e.to_graphql_error())?;

        schedules = schedules
            .into_iter()
            .filter(|s| s.last_completed_at.is_none())
            .collect();

        if let Some(true) = active_only {
            schedules = schedules
                .into_iter()
                .filter(|s| s.active)
                .collect();
        }

        schedules.sort_by(|a, b| a.created_at.cmp(&b.created_at));

        if let Some(limit_val) = limit {
            schedules.truncate(limit_val as usize);
        }

        Ok(schedules)
    }

    async fn recently_completed_maintenance_schedules(
        &self,
        ctx: &Context<'_>,
        days_back: Option<i32>,
        limit: Option<i32>
    ) -> Result<Vec<MaintenanceSchedule>, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        let mut schedules = repo
            .list::<MaintenanceSchedule>(None).await
            .map_err(|e| e.to_graphql_error())?;

        let days_back = days_back.unwrap_or(30);
        let cutoff_date = Utc::now() - chrono::Duration::days(days_back as i64);

        schedules = schedules
            .into_iter()
            .filter(|s| {
                if let Some(completed_at) = s.last_completed_at {
                    completed_at >= cutoff_date
                } else {
                    false
                }
            })
            .collect();

        schedules.sort_by(|a, b| {
            b.last_completed_at
                .unwrap_or_else(|| Utc::now())
                .cmp(&a.last_completed_at.unwrap_or_else(|| Utc::now()))
        });

        if let Some(limit_val) = limit {
            schedules.truncate(limit_val as usize);
        }

        Ok(schedules)
    }

    async fn maintenance_schedules_by_duration_range(
        &self,
        ctx: &Context<'_>,
        min_duration: Option<i32>,
        max_duration: Option<i32>,
        active_only: Option<bool>,
        limit: Option<i32>
    ) -> Result<Vec<MaintenanceSchedule>, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        let mut schedules = repo
            .list::<MaintenanceSchedule>(None).await
            .map_err(|e| e.to_graphql_error())?;

        schedules = schedules
            .into_iter()
            .filter(|s| {
                if let Some(duration) = s.duration_estimate {
                    let meets_min = min_duration.map_or(true, |min| duration >= min);
                    let meets_max = max_duration.map_or(true, |max| duration <= max);
                    meets_min && meets_max
                } else {
                    min_duration.is_none() && max_duration.is_none()
                }
            })
            .collect();

        if let Some(true) = active_only {
            schedules = schedules
                .into_iter()
                .filter(|s| s.active)
                .collect();
        }

        schedules.sort_by(|a, b| {
            a.duration_estimate.unwrap_or(0).cmp(&b.duration_estimate.unwrap_or(0))
        });

        if let Some(limit_val) = limit {
            schedules.truncate(limit_val as usize);
        }

        Ok(schedules)
    }

    async fn maintenance_schedule_statistics(
        &self,
        ctx: &Context<'_>
    ) -> Result<MaintenanceScheduleStatistics, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        let schedules = repo
            .list::<MaintenanceSchedule>(None).await
            .map_err(|e| e.to_graphql_error())?;

        let now = Utc::now();
        let total_schedules = schedules.len() as i32;
        let active_schedules = schedules
            .iter()
            .filter(|s| s.active)
            .count() as i32;
        let recurring_schedules = schedules
            .iter()
            .filter(|s| s.recurring)
            .count() as i32;
        let overdue_schedules = schedules
            .iter()
            .filter(|s| s.active && s.next_due_at < now)
            .count() as i32;
        let never_completed = schedules
            .iter()
            .filter(|s| s.last_completed_at.is_none())
            .count() as i32;
        let completed_schedules = schedules
            .iter()
            .filter(|s| s.last_completed_at.is_some())
            .count() as i32;

        Ok(MaintenanceScheduleStatistics {
            total_schedules,
            active_schedules,
            recurring_schedules,
            overdue_schedules,
            never_completed,
            completed_schedules,
        })
    }

    async fn maintenance_workload_by_date_range(
        &self,
        ctx: &Context<'_>,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
        active_only: Option<bool>
    ) -> Result<Vec<MaintenanceSchedule>, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        let mut schedules = repo
            .list::<MaintenanceSchedule>(None).await
            .map_err(|e| e.to_graphql_error())?;

        schedules = schedules
            .into_iter()
            .filter(|s| s.next_due_at >= start_date && s.next_due_at <= end_date)
            .collect();

        if let Some(true) = active_only {
            schedules = schedules
                .into_iter()
                .filter(|s| s.active)
                .collect();
        }

        schedules.sort_by(|a, b| a.next_due_at.cmp(&b.next_due_at));

        Ok(schedules)
    }
}

#[derive(Debug)]
pub struct MaintenanceScheduleStatistics {
    pub total_schedules: i32,
    pub active_schedules: i32,
    pub recurring_schedules: i32,
    pub overdue_schedules: i32,
    pub never_completed: i32,
    pub completed_schedules: i32,
}

#[Object]
impl MaintenanceScheduleStatistics {
    async fn total_schedules(&self) -> i32 {
        self.total_schedules
    }

    async fn active_schedules(&self) -> i32 {
        self.active_schedules
    }

    async fn recurring_schedules(&self) -> i32 {
        self.recurring_schedules
    }

    async fn overdue_schedules(&self) -> i32 {
        self.overdue_schedules
    }

    async fn never_completed(&self) -> i32 {
        self.never_completed
    }

    async fn completed_schedules(&self) -> i32 {
        self.completed_schedules
    }

    async fn inactive_schedules(&self) -> i32 {
        self.total_schedules - self.active_schedules
    }

    async fn completion_rate(&self) -> f64 {
        if self.total_schedules == 0 {
            0.0
        } else {
            ((self.completed_schedules as f64) / (self.total_schedules as f64)) * 100.0
        }
    }

    async fn overdue_percentage(&self) -> f64 {
        if self.active_schedules == 0 {
            0.0
        } else {
            ((self.overdue_schedules as f64) / (self.active_schedules as f64)) * 100.0
        }
    }
}
