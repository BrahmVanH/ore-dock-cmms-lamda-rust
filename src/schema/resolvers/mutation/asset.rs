use async_graphql::*;
use chrono::{ DateTime, Utc };
use tracing::{ info, warn };
use uuid::Uuid;

use crate::{
    DbClient,
    models::asset::{ Asset, AssetCurrentStatusOptions, MaintenanceFrequencyOptions },
    AppError,
    Repository,
};

#[derive(Debug, Default)]
pub(crate) struct Mutation;

#[Object]
impl Mutation {
    /// Create a new asset
    async fn create_asset(
        &self,
        ctx: &Context<'_>,
        name: String,
        type_id: String,
        serial_number: String,
        model_number: String,
        purchase_date: DateTime<Utc>,
        installation_date: DateTime<Utc>,
        location_id: String,
        manufacturer_id: String,
        maintenance_frequency: String,
        warranty_start_date: Option<DateTime<Utc>>,
        warranty_end_date: Option<DateTime<Utc>>
    ) -> Result<Asset, Error> {
        info!("Creating new asset: {}", name);

        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let id = format!("asset-{}", Uuid::new_v4());

        // Validate that dependencies exist
        let repo = Repository::new(db_client.clone());

        // Check if asset type exists
        repo
            .get::<crate::models::asset_type::AssetType>(type_id.clone()).await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| {
                AppError::ValidationError(
                    format!("Asset type {} not found", type_id)
                ).to_graphql_error()
            })?;

        // Check if location exists
        repo
            .get::<crate::models::location::Location>(location_id.clone()).await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| {
                AppError::ValidationError(
                    format!("Location {} not found", location_id)
                ).to_graphql_error()
            })?;

        // Check if manufacturer exists
        repo
            .get::<crate::models::manufacturer::Manufacturer>(manufacturer_id.clone()).await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| {
                AppError::ValidationError(
                    format!("Manufacturer {} not found", manufacturer_id)
                ).to_graphql_error()
            })?;

        // Create the asset
        let asset = Asset::new(
            id,
            name,
            type_id,
            serial_number,
            model_number,
            purchase_date,
            installation_date,
            location_id,
            manufacturer_id,
            maintenance_frequency,
            warranty_start_date,
            warranty_end_date
        ).map_err(|e| e.to_graphql_error())?;

        repo.create(asset).await.map_err(|e| e.to_graphql_error())
    }

    /// Update an existing asset
    async fn update_asset(
        &self,
        ctx: &Context<'_>,
        id: String,
        name: Option<String>,
        type_id: Option<String>,
        serial_number: Option<String>,
        model_number: Option<String>,
        purchase_date: Option<DateTime<Utc>>,
        installation_date: Option<DateTime<Utc>>,
        location_id: Option<String>,
        manufacturer_id: Option<String>,
        maintenance_frequency: Option<String>,
        warranty_start_date: Option<DateTime<Utc>>,
        warranty_end_date: Option<DateTime<Utc>>
    ) -> Result<Asset, Error> {
        info!("Updating asset: {}", id);

        let db_client = ctx
            .data::<DbClient>()
            .map_err(|_| {
                AppError::InternalServerError("Database client not available".to_string())
            })?;

        let repo = Repository::new(db_client.clone());

        let mut asset = repo
            .get::<Asset>(id.clone()).await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| AppError::NotFound(format!("Asset {} not found", id)))?;

        // Validate dependencies if they're being changed
        if let Some(ref new_type_id) = type_id {
            repo
                .get::<crate::models::asset_type::AssetType>(new_type_id.clone()).await
                .map_err(|e| e.to_graphql_error())?
                .ok_or_else(|| {
                    AppError::ValidationError(
                        format!("Asset type {} not found", new_type_id)
                    ).to_graphql_error()
                })?;
        }

        if let Some(ref new_location_id) = location_id {
            repo
                .get::<crate::models::location::Location>(new_location_id.clone()).await
                .map_err(|e| e.to_graphql_error())?
                .ok_or_else(|| {
                    AppError::ValidationError(
                        format!("Location {} not found", new_location_id)
                    ).to_graphql_error()
                })?;
        }

        if let Some(ref new_manufacturer_id) = manufacturer_id {
            repo
                .get::<crate::models::manufacturer::Manufacturer>(new_manufacturer_id.clone()).await
                .map_err(|e| e.to_graphql_error())?
                .ok_or_else(|| {
                    AppError::ValidationError(
                        format!("Manufacturer {} not found", new_manufacturer_id)
                    ).to_graphql_error()
                })?;
        }

        // Update fields
        if let Some(name) = name {
            asset.name = name;
        }
        if let Some(type_id) = type_id {
            asset.asset_type_id = type_id;
        }
        if let Some(serial_number) = serial_number {
            asset.serial_number = serial_number;
        }
        if let Some(model_number) = model_number {
            asset.model_number = model_number;
        }
        if let Some(purchase_date) = purchase_date {
            asset.purchase_date = purchase_date;
        }
        if let Some(installation_date) = installation_date {
            asset.installation_date = installation_date;
        }
        if let Some(location_id) = location_id {
            asset.location_id = location_id;
        }
        if let Some(manufacturer_id) = manufacturer_id {
            asset.manufacturer_id = manufacturer_id;
        }
        if let Some(maintenance_frequency) = maintenance_frequency {
            asset.maintenance_frequency = MaintenanceFrequencyOptions::from_string(
                &maintenance_frequency
            ).map_err(|e| e.to_graphql_error())?;
            asset.interval_days = MaintenanceFrequencyOptions::to_days(
                &asset.maintenance_frequency
            ).map_err(|e| e.to_graphql_error())?;
        }
        asset.warranty_start_date = warranty_start_date.or(asset.warranty_start_date);
        asset.warranty_end_date = warranty_end_date.or(asset.warranty_end_date);
        asset.updated_at = Utc::now();

        repo.update(asset).await.map_err(|e| e.to_graphql_error())
    }

    /// Update asset status
    async fn update_asset_status(
        &self,
        ctx: &Context<'_>,
        id: String,
        status: String
    ) -> Result<Asset, Error> {
        info!("Updating asset status: {} to {}", id, status);

        let db_client = ctx
            .data::<DbClient>()
            .map_err(|_| {
                AppError::InternalServerError("Database client not available".to_string())
            })?;

        let repo = Repository::new(db_client.clone());

        let mut asset = repo
            .get::<Asset>(id.clone()).await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| AppError::NotFound(format!("Asset {} not found", id)))?;

        // Validate status
        let new_status = AssetCurrentStatusOptions::from_string(&status).map_err(|e|
            e.to_graphql_error()
        )?;

        // Update status and handle business logic
        let old_status = asset.current_status.clone();
        asset.current_status = new_status.clone();
        asset.updated_at = Utc::now();

        // Handle status-specific logic
        match new_status {
            AssetCurrentStatusOptions::Down => {
                asset.last_downtime_date = Utc::now();
            }
            AssetCurrentStatusOptions::Operational => {
                // If coming from Down status, could calculate downtime
                // This is where you'd add business logic for status changes
            }
            _ => {}
        }

        repo.update(asset).await.map_err(|e| e.to_graphql_error())
    }

    /// Add work order to asset
    async fn add_work_order_to_asset(
        &self,
        ctx: &Context<'_>,
        asset_id: String,
        work_order_id: String
    ) -> Result<Asset, Error> {
        info!("Adding work order {} to asset {}", work_order_id, asset_id);

        let db_client = ctx
            .data::<DbClient>()
            .map_err(|_| {
                AppError::InternalServerError("Database client not available".to_string())
            })?;

        let repo = Repository::new(db_client.clone());

        let mut asset = repo
            .get::<Asset>(asset_id.clone()).await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| AppError::NotFound(format!("Asset {} not found", asset_id)))?;

        // Verify work order exists
        repo
            .get::<crate::models::work_order::WorkOrder>(work_order_id.clone()).await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| {
                AppError::ValidationError(
                    format!("Work order {} not found", work_order_id)
                ).to_graphql_error()
            })?;

        // Add work order if not already present
        if !asset.work_order_ids.contains(&work_order_id) {
            asset.work_order_ids.push(work_order_id);
            asset.updated_at = Utc::now();

            repo.update(asset).await.map_err(|e| e.to_graphql_error())
        } else {
            Err(
                AppError::ValidationError(
                    format!(
                        "Work order {} already associated with asset {}",
                        work_order_id,
                        asset_id
                    )
                ).to_graphql_error()
            )
        }
    }

    /// Remove work order from asset
    async fn remove_work_order_from_asset(
        &self,
        ctx: &Context<'_>,
        asset_id: String,
        work_order_id: String
    ) -> Result<Asset, Error> {
        info!("Removing work order {} from asset {}", work_order_id, asset_id);

        let db_client = ctx
            .data::<DbClient>()
            .map_err(|_| {
                AppError::InternalServerError("Database client not available".to_string())
            })?;

        let repo = Repository::new(db_client.clone());

        let mut asset = repo
            .get::<Asset>(asset_id.clone()).await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| AppError::NotFound(format!("Asset {} not found", asset_id)))?;

        // Remove work order if present
        if let Some(pos) = asset.work_order_ids.iter().position(|x| *x == work_order_id) {
            asset.work_order_ids.remove(pos);
            asset.updated_at = Utc::now();

            repo.update(asset).await.map_err(|e| e.to_graphql_error())
        } else {
            Err(
                AppError::ValidationError(
                    format!("Work order {} not associated with asset {}", work_order_id, asset_id)
                ).to_graphql_error()
            )
        }
    }

    /// Add documentation key to asset
    async fn add_documentation_to_asset(
        &self,
        ctx: &Context<'_>,
        asset_id: String,
        documentation_key: String
    ) -> Result<Asset, Error> {
        info!("Adding documentation {} to asset {}", documentation_key, asset_id);

        let db_client = ctx
            .data::<DbClient>()
            .map_err(|_| {
                AppError::InternalServerError("Database client not available".to_string())
            })?;

        let repo = Repository::new(db_client.clone());

        let mut asset = repo
            .get::<Asset>(asset_id.clone()).await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| AppError::NotFound(format!("Asset {} not found", asset_id)))?;

        // Add documentation key if not already present
        if !asset.documentation_keys.contains(&documentation_key) {
            asset.documentation_keys.push(documentation_key);
            asset.updated_at = Utc::now();

            repo.update(asset).await.map_err(|e| e.to_graphql_error())
        } else {
            Err(
                AppError::ValidationError(
                    format!(
                        "Documentation key {} already associated with asset {}",
                        documentation_key,
                        asset_id
                    )
                ).to_graphql_error()
            )
        }
    }

    /// Remove documentation key from asset
    async fn remove_documentation_from_asset(
        &self,
        ctx: &Context<'_>,
        asset_id: String,
        documentation_key: String
    ) -> Result<Asset, Error> {
        info!("Removing documentation {} from asset {}", documentation_key, asset_id);

        let db_client = ctx
            .data::<DbClient>()
            .map_err(|_| {
                AppError::InternalServerError("Database client not available".to_string())
            })?;

        let repo = Repository::new(db_client.clone());

        let mut asset = repo
            .get::<Asset>(asset_id.clone()).await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| AppError::NotFound(format!("Asset {} not found", asset_id)))?;

        // Remove documentation key if present
        if let Some(pos) = asset.documentation_keys.iter().position(|x| *x == documentation_key) {
            asset.documentation_keys.remove(pos);
            asset.updated_at = Utc::now();

            repo.update(asset).await.map_err(|e| e.to_graphql_error())
        } else {
            Err(
                AppError::ValidationError(
                    format!(
                        "Documentation key {} not associated with asset {}",
                        documentation_key,
                        asset_id
                    )
                ).to_graphql_error()
            )
        }
    }

    /// Update asset maintenance schedule
    async fn update_asset_maintenance_schedule(
        &self,
        ctx: &Context<'_>,
        asset_id: String,
        maintenance_schedule_id: Option<String>
    ) -> Result<Asset, Error> {
        info!("Updating maintenance schedule for asset {}", asset_id);

        let db_client = ctx
            .data::<DbClient>()
            .map_err(|_| {
                AppError::InternalServerError("Database client not available".to_string())
            })?;

        let repo = Repository::new(db_client.clone());

        let mut asset = repo
            .get::<Asset>(asset_id.clone()).await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| AppError::NotFound(format!("Asset {} not found", asset_id)))?;

        // Validate maintenance schedule exists if provided
        if let Some(ref schedule_id) = maintenance_schedule_id {
            repo
                .get::<crate::models::maintenance_schedule::MaintenanceSchedule>(
                    schedule_id.clone()
                ).await
                .map_err(|e| e.to_graphql_error())?
                .ok_or_else(|| {
                    AppError::ValidationError(
                        format!("Maintenance schedule {} not found", schedule_id)
                    ).to_graphql_error()
                })?;
        }

        asset.maintenance_schedule_id = maintenance_schedule_id;
        asset.updated_at = Utc::now();

        repo.update(asset).await.map_err(|e| e.to_graphql_error())
    }

    /// Delete an asset
    async fn delete_asset(&self, ctx: &Context<'_>, id: String) -> Result<bool, Error> {
        info!("Deleting asset: {}", id);

        let db_client = ctx
            .data::<DbClient>()
            .map_err(|_| {
                AppError::InternalServerError("Database client not available".to_string())
            })?;

        let repo = Repository::new(db_client.clone());

        // Verify asset exists
        let asset = repo
            .get::<Asset>(id.clone()).await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| AppError::NotFound(format!("Asset {} not found", id)))?;

        // Business logic: Check if asset can be deleted
        // For example, don't allow deletion if there are active work orders
        if !asset.work_order_ids.is_empty() {
            return Err(
                AppError::ValidationError(
                    format!("Cannot delete asset {} - it has associated work orders", id)
                ).to_graphql_error()
            );
        }

        // Check if asset is not retired
        if !matches!(asset.current_status, AssetCurrentStatusOptions::Retired) {
            return Err(
                AppError::ValidationError(
                    format!("Cannot delete asset {} - asset must be retired before deletion", id)
                ).to_graphql_error()
            );
        }

        repo.delete::<Asset>(id).await.map_err(|e| e.to_graphql_error())
    }
}
