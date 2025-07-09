use async_graphql::*;
use chrono::{ DateTime, Utc };
use tracing::{ info, warn };
use uuid::Uuid;

use crate::{
    DbClient,
    models::{
        maintenance_schedule::{ MaintenanceSchedule, MaintenanceCadence, CadenceUnit },
        asset::Asset,
        user::User,
    },
    AppError,
    Repository,
};

#[derive(Debug, Default)]
pub struct MaintenanceScheduleMutation;

#[Object]
impl MaintenanceScheduleMutation {
    async fn create_maintenance_schedule(
        &self,
        ctx: &Context<'_>,
        asset_id: String,
        cadence_intervals: Vec<i32>,
        cadence_units: Vec<String>,
        next_due_at: DateTime<Utc>,
        duration_estimate: Option<i32>,
        recurring: Option<bool>,
        active: Option<bool>
    ) -> Result<MaintenanceSchedule, Error> {
        info!("Creating new maintenance schedule for asset: {}", asset_id);

        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());
        let id = Uuid::new_v4().to_string();

        let _asset = repo
            .get::<Asset>(asset_id.clone()).await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| {
                AppError::ValidationError(
                    format!("Asset {} not found", asset_id)
                ).to_graphql_error()
            })?;

        if cadence_intervals.len() != cadence_units.len() {
            return Err(
                AppError::ValidationError(
                    "Cadence intervals and units must have the same length".to_string()
                ).to_graphql_error()
            );
        }

        let mut cadences = Vec::new();
        for (interval, unit_str) in cadence_intervals.iter().zip(cadence_units.iter()) {
            let unit = CadenceUnit::from_string(unit_str).map_err(|e| e.to_graphql_error())?;
            cadences.push(MaintenanceCadence {
                interval: *interval,
                unit,
            });
        }

        let schedule = MaintenanceSchedule::new(
            id,
            asset_id,
            cadences,
            None,
            None,
            next_due_at,
            duration_estimate,
            recurring.unwrap_or(true),
            active.unwrap_or(true)
        ).map_err(|e| e.to_graphql_error())?;

        repo.create(schedule).await.map_err(|e| e.to_graphql_error())
    }

    async fn update_maintenance_schedule(
        &self,
        ctx: &Context<'_>,
        id: String,
        cadence_intervals: Option<Vec<i32>>,
        cadence_units: Option<Vec<String>>,
        next_due_at: Option<DateTime<Utc>>,
        duration_estimate: Option<i32>,
        recurring: Option<bool>,
        active: Option<bool>
    ) -> Result<MaintenanceSchedule, Error> {
        info!("Updating maintenance schedule: {}", id);

        let db_client = ctx
            .data::<DbClient>()
            .map_err(|_| {
                AppError::InternalServerError("Database client not available".to_string())
            })?;

        let repo = Repository::new(db_client.clone());

        let mut schedule = repo
            .get::<MaintenanceSchedule>(id.clone()).await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| AppError::NotFound(format!("Maintenance schedule {} not found", id)))?;

        if let (Some(intervals), Some(units)) = (cadence_intervals, cadence_units) {
            if intervals.len() != units.len() {
                return Err(
                    AppError::ValidationError(
                        "Cadence intervals and units must have the same length".to_string()
                    ).to_graphql_error()
                );
            }

            let mut cadences = Vec::new();
            for (interval, unit_str) in intervals.iter().zip(units.iter()) {
                let unit = CadenceUnit::from_string(unit_str).map_err(|e| e.to_graphql_error())?;
                cadences.push(MaintenanceCadence {
                    interval: *interval,
                    unit,
                });
            }
            schedule.cadences = cadences;
        }

        if let Some(due_at) = next_due_at {
            schedule.next_due_at = due_at;
        }

        if let Some(duration) = duration_estimate {
            schedule.duration_estimate = if duration <= 0 { None } else { Some(duration) };
        }

        if let Some(is_recurring) = recurring {
            schedule.recurring = is_recurring;
        }

        if let Some(is_active) = active {
            schedule.active = is_active;
        }

        schedule.updated_at = Utc::now();

        repo.update(schedule).await.map_err(|e| e.to_graphql_error())
    }

    async fn complete_maintenance_schedule(
        &self,
        ctx: &Context<'_>,
        id: String,
        completed_by_user_id: String,
        completed_at: Option<DateTime<Utc>>
    ) -> Result<MaintenanceSchedule, Error> {
        info!("Completing maintenance schedule: {}", id);

        let db_client = ctx
            .data::<DbClient>()
            .map_err(|_| {
                AppError::InternalServerError("Database client not available".to_string())
            })?;

        let repo = Repository::new(db_client.clone());

        let mut schedule = repo
            .get::<MaintenanceSchedule>(id.clone()).await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| AppError::NotFound(format!("Maintenance schedule {} not found", id)))?;

        let _user = repo
            .get::<User>(completed_by_user_id.clone()).await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| {
                AppError::ValidationError(
                    format!("User {} not found", completed_by_user_id)
                ).to_graphql_error()
            })?;

        let completion_time = completed_at.unwrap_or_else(|| Utc::now());

        schedule.last_completed_at = Some(completion_time);
        schedule.last_completed_by_user_id = Some(completed_by_user_id);

        if schedule.recurring {
            if let Some(primary_cadence) = schedule.cadences.first() {
                if let Ok(days_to_add) = primary_cadence.to_days() {
                    schedule.next_due_at =
                        completion_time + chrono::Duration::days(days_to_add as i64);
                } else {
                    schedule.next_due_at = completion_time + chrono::Duration::days(30);
                }
            }
        } else {
            schedule.active = false;
        }

        schedule.updated_at = Utc::now();

        repo.update(schedule).await.map_err(|e| e.to_graphql_error())
    }

    async fn reschedule_maintenance(
        &self,
        ctx: &Context<'_>,
        id: String,
        new_due_date: DateTime<Utc>
    ) -> Result<MaintenanceSchedule, Error> {
        info!("Rescheduling maintenance schedule: {}", id);

        let db_client = ctx
            .data::<DbClient>()
            .map_err(|_| {
                AppError::InternalServerError("Database client not available".to_string())
            })?;

        let repo = Repository::new(db_client.clone());

        let mut schedule = repo
            .get::<MaintenanceSchedule>(id.clone()).await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| AppError::NotFound(format!("Maintenance schedule {} not found", id)))?;

        schedule.next_due_at = new_due_date;
        schedule.updated_at = Utc::now();

        repo.update(schedule).await.map_err(|e| e.to_graphql_error())
    }

    async fn activate_maintenance_schedule(
        &self,
        ctx: &Context<'_>,
        id: String
    ) -> Result<MaintenanceSchedule, Error> {
        info!("Activating maintenance schedule: {}", id);

        let db_client = ctx
            .data::<DbClient>()
            .map_err(|_| {
                AppError::InternalServerError("Database client not available".to_string())
            })?;

        let repo = Repository::new(db_client.clone());

        let mut schedule = repo
            .get::<MaintenanceSchedule>(id.clone()).await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| AppError::NotFound(format!("Maintenance schedule {} not found", id)))?;

        schedule.active = true;
        schedule.updated_at = Utc::now();

        repo.update(schedule).await.map_err(|e| e.to_graphql_error())
    }

    async fn deactivate_maintenance_schedule(
        &self,
        ctx: &Context<'_>,
        id: String
    ) -> Result<MaintenanceSchedule, Error> {
        info!("Deactivating maintenance schedule: {}", id);

        let db_client = ctx
            .data::<DbClient>()
            .map_err(|_| {
                AppError::InternalServerError("Database client not available".to_string())
            })?;

        let repo = Repository::new(db_client.clone());

        let mut schedule = repo
            .get::<MaintenanceSchedule>(id.clone()).await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| AppError::NotFound(format!("Maintenance schedule {} not found", id)))?;

        schedule.active = false;
        schedule.updated_at = Utc::now();

        repo.update(schedule).await.map_err(|e| e.to_graphql_error())
    }

    async fn add_cadence_to_schedule(
        &self,
        ctx: &Context<'_>,
        id: String,
        interval: i32,
        unit: String
    ) -> Result<MaintenanceSchedule, Error> {
        info!("Adding cadence to maintenance schedule: {}", id);

        let db_client = ctx
            .data::<DbClient>()
            .map_err(|_| {
                AppError::InternalServerError("Database client not available".to_string())
            })?;

        let repo = Repository::new(db_client.clone());

        let mut schedule = repo
            .get::<MaintenanceSchedule>(id.clone()).await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| AppError::NotFound(format!("Maintenance schedule {} not found", id)))?;

        let cadence_unit = CadenceUnit::from_string(&unit).map_err(|e| e.to_graphql_error())?;

        let new_cadence = MaintenanceCadence {
            interval,
            unit: cadence_unit,
        };

        schedule.cadences.push(new_cadence);
        schedule.updated_at = Utc::now();

        repo.update(schedule).await.map_err(|e| e.to_graphql_error())
    }

    async fn clone_maintenance_schedule(
        &self,
        ctx: &Context<'_>,
        source_schedule_id: String,
        target_asset_id: String,
        next_due_at: Option<DateTime<Utc>>
    ) -> Result<MaintenanceSchedule, Error> {
        info!("Cloning maintenance schedule {} to asset {}", source_schedule_id, target_asset_id);

        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        let source_schedule = repo
            .get::<MaintenanceSchedule>(source_schedule_id.clone()).await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(||
                AppError::NotFound(format!("Source schedule {} not found", source_schedule_id))
            )?;

        let _target_asset = repo
            .get::<Asset>(target_asset_id.clone()).await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| {
                AppError::ValidationError(
                    format!("Target asset {} not found", target_asset_id)
                ).to_graphql_error()
            })?;

        let new_id = Uuid::new_v4().to_string();
        let due_date = next_due_at.unwrap_or(source_schedule.next_due_at);

        let cloned_schedule = MaintenanceSchedule::new(
            new_id,
            target_asset_id,
            source_schedule.cadences.clone(),
            None,
            None,
            due_date,
            source_schedule.duration_estimate,
            source_schedule.recurring,
            source_schedule.active
        ).map_err(|e| e.to_graphql_error())?;

        repo.create(cloned_schedule).await.map_err(|e| e.to_graphql_error())
    }

    async fn bulk_reschedule_maintenance(
        &self,
        ctx: &Context<'_>,
        schedule_ids: Vec<String>,
        days_to_add: i32
    ) -> Result<Vec<MaintenanceSchedule>, Error> {
        info!("Bulk rescheduling {} maintenance schedules", schedule_ids.len());

        let db_client = ctx
            .data::<DbClient>()
            .map_err(|_| {
                AppError::InternalServerError("Database client not available".to_string())
            })?;

        let repo = Repository::new(db_client.clone());

        let mut results = Vec::new();

        for schedule_id in schedule_ids {
            let schedule_result = repo.get::<MaintenanceSchedule>(schedule_id.clone()).await;

            if let Ok(Some(mut schedule)) = schedule_result {
                schedule.next_due_at =
                    schedule.next_due_at + chrono::Duration::days(days_to_add as i64);
                schedule.updated_at = Utc::now();

                if let Ok(updated_schedule) = repo.update(schedule).await {
                    results.push(updated_schedule);
                }
            }
        }

        Ok(results)
    }

    async fn bulk_update_maintenance_schedules(
        &self,
        ctx: &Context<'_>,
        schedule_ids: Vec<String>,
        active: Option<bool>,
        recurring: Option<bool>,
        duration_estimate: Option<i32>
    ) -> Result<Vec<MaintenanceSchedule>, Error> {
        info!("Bulk updating {} maintenance schedules", schedule_ids.len());

        let db_client = ctx
            .data::<DbClient>()
            .map_err(|_| {
                AppError::InternalServerError("Database client not available".to_string())
            })?;

        let repo = Repository::new(db_client.clone());

        let mut results = Vec::new();

        for schedule_id in schedule_ids {
            let schedule_result = repo.get::<MaintenanceSchedule>(schedule_id.clone()).await;

            if let Ok(Some(mut schedule)) = schedule_result {
                let mut updated = false;

                if let Some(is_active) = active {
                    schedule.active = is_active;
                    updated = true;
                }

                if let Some(is_recurring) = recurring {
                    schedule.recurring = is_recurring;
                    updated = true;
                }

                if let Some(duration) = duration_estimate {
                    schedule.duration_estimate = if duration <= 0 { None } else { Some(duration) };
                    updated = true;
                }

                if updated {
                    schedule.updated_at = Utc::now();
                    if let Ok(updated_schedule) = repo.update(schedule).await {
                        results.push(updated_schedule);
                    }
                }
            }
        }

        Ok(results)
    }

    async fn copy_schedules_to_asset(
        &self,
        ctx: &Context<'_>,
        source_asset_id: String,
        target_asset_id: String,
        active_only: Option<bool>
    ) -> Result<Vec<MaintenanceSchedule>, Error> {
        info!(
            "Copying maintenance schedules from asset {} to asset {}",
            source_asset_id,
            target_asset_id
        );

        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        let _source_asset = repo
            .get::<Asset>(source_asset_id.clone()).await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| {
                AppError::ValidationError(
                    format!("Source asset {} not found", source_asset_id)
                ).to_graphql_error()
            })?;

        let _target_asset = repo
            .get::<Asset>(target_asset_id.clone()).await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| {
                AppError::ValidationError(
                    format!("Target asset {} not found", target_asset_id)
                ).to_graphql_error()
            })?;

        let all_schedules = repo
            .list::<MaintenanceSchedule>(None).await
            .map_err(|e| e.to_graphql_error())?;

        let mut source_schedules: Vec<MaintenanceSchedule> = all_schedules
            .into_iter()
            .filter(|s| s.asset_id == source_asset_id)
            .collect();

        if let Some(true) = active_only {
            source_schedules = source_schedules
                .into_iter()
                .filter(|s| s.active)
                .collect();
        }

        let mut results = Vec::new();

        for source_schedule in source_schedules {
            let new_id = Uuid::new_v4().to_string();

            let cloned_schedule = MaintenanceSchedule::new(
                new_id,
                target_asset_id.clone(),
                source_schedule.cadences,
                None,
                None,
                source_schedule.next_due_at,
                source_schedule.duration_estimate,
                source_schedule.recurring,
                source_schedule.active
            ).map_err(|e| e.to_graphql_error())?;

            if let Ok(created_schedule) = repo.create(cloned_schedule).await {
                results.push(created_schedule);
            }
        }

        Ok(results)
    }

    async fn delete_maintenance_schedule(
        &self,
        ctx: &Context<'_>,
        id: String
    ) -> Result<bool, Error> {
        info!("Deleting maintenance schedule: {}", id);

        let db_client = ctx
            .data::<DbClient>()
            .map_err(|_| {
                AppError::InternalServerError("Database client not available".to_string())
            })?;

        let repo = Repository::new(db_client.clone());

        let _schedule = repo
            .get::<MaintenanceSchedule>(id.clone()).await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| AppError::NotFound(format!("Maintenance schedule {} not found", id)))?;

        repo.delete::<MaintenanceSchedule>(id).await.map_err(|e| e.to_graphql_error())
    }

    async fn delete_schedules_for_asset(
        &self,
        ctx: &Context<'_>,
        asset_id: String
    ) -> Result<i32, Error> {
        info!("Deleting all maintenance schedules for asset: {}", asset_id);

        let db_client = ctx
            .data::<DbClient>()
            .map_err(|_| {
                AppError::InternalServerError("Database client not available".to_string())
            })?;

        let repo = Repository::new(db_client.clone());

        let all_schedules = repo
            .list::<MaintenanceSchedule>(None).await
            .map_err(|e| e.to_graphql_error())?;

        let asset_schedules: Vec<MaintenanceSchedule> = all_schedules
            .into_iter()
            .filter(|s| s.asset_id == asset_id)
            .collect();

        let mut deleted_count = 0;

        for schedule in asset_schedules {
            if repo.delete::<MaintenanceSchedule>(schedule.id).await.is_ok() {
                deleted_count += 1;
            }
        }

        Ok(deleted_count)
    }
}
