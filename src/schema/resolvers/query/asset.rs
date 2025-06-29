use async_graphql::*;
use chrono::{ DateTime, Utc };
use tracing::warn;

use crate::{
    error::AppError,
    models::{
        asset::{ Asset, AssetCurrentStatusOptions },
        work_order::WorkOrder,
    },
    DbClient,
    Repository,
};
#[derive(Debug, Default)]
pub(crate) struct Query;

#[Object]
impl Query {
    /// Get asset by ID
    async fn asset_by_id(&self, ctx: &Context<'_>, id: String) -> Result<Option<Asset>, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        repo.get::<Asset>(id).await.map_err(|e| e.to_graphql_error())
    }

    /// Get all assets with optional filtering
    async fn assets(
        &self,
        ctx: &Context<'_>,
        limit: Option<i32>,
        status_filter: Option<String>,
        type_filter: Option<String>,
        location_filter: Option<String>,
        manufacturer_filter: Option<String>
    ) -> Result<Vec<Asset>, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        let mut assets = repo.list::<Asset>(limit).await.map_err(|e| e.to_graphql_error())?;

        // Apply status filter
        if let Some(status) = status_filter {
            let status_enum = AssetCurrentStatusOptions::from_string(&status).map_err(|e|
                e.to_graphql_error()
            )?;
            assets = assets
                .into_iter()
                .filter(|asset| asset.current_status.to_string() == status_enum.to_string())
                .collect();
        }

        // Apply type filter
        if let Some(type_id) = type_filter {
            assets = assets
                .into_iter()
                .filter(|asset| asset.asset_type_id == type_id)
                .collect();
        }

        // Apply location filter
        if let Some(location_id) = location_filter {
            assets = assets
                .into_iter()
                .filter(|asset| asset.location_id == location_id)
                .collect();
        }

        // Apply manufacturer filter
        if let Some(manufacturer_id) = manufacturer_filter {
            assets = assets
                .into_iter()
                .filter(|asset| asset.manufacturer_id == manufacturer_id)
                .collect();
        }

        Ok(assets)
    }

    /// Get assets by location
    async fn assets_by_location(
        &self,
        ctx: &Context<'_>,
        location_id: String,
        limit: Option<i32>,
        status_filter: Option<String>
    ) -> Result<Vec<Asset>, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        let mut assets = repo.list::<Asset>(None).await.map_err(|e| e.to_graphql_error())?;

        // Filter by location
        assets = assets
            .into_iter()
            .filter(|asset| asset.location_id == location_id)
            .collect();

        // Apply status filter if provided
        if let Some(status) = status_filter {
            let status_enum = AssetCurrentStatusOptions::from_string(&status).map_err(|e|
                e.to_graphql_error()
            )?;
            assets = assets
                .into_iter()
                .filter(|asset| asset.current_status.to_string() == status_enum.to_string())
                .collect();
        }

        // Apply limit if provided
        if let Some(limit_val) = limit {
            assets.truncate(limit_val as usize);
        }

        Ok(assets)
    }

    /// Get assets by type
    async fn assets_by_type(
        &self,
        ctx: &Context<'_>,
        asset_type_id: String,
        limit: Option<i32>,
        status_filter: Option<String>
    ) -> Result<Vec<Asset>, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        let mut assets = repo.list::<Asset>(None).await.map_err(|e| e.to_graphql_error())?;

        // Filter by asset type
        assets = assets
            .into_iter()
            .filter(|asset| asset.asset_type_id == asset_type_id)
            .collect();

        // Apply status filter if provided
        if let Some(status) = status_filter {
            let status_enum = AssetCurrentStatusOptions::from_string(&status).map_err(|e|
                e.to_graphql_error()
            )?;
            assets = assets
                .into_iter()
                .filter(|asset| asset.current_status.to_string() == status_enum.to_string())
                .collect();
        }

        // Apply limit if provided
        if let Some(limit_val) = limit {
            assets.truncate(limit_val as usize);
        }

        Ok(assets)
    }

    /// Get assets by manufacturer
    async fn assets_by_manufacturer(
        &self,
        ctx: &Context<'_>,
        manufacturer_id: String,
        limit: Option<i32>,
        status_filter: Option<String>
    ) -> Result<Vec<Asset>, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        let mut assets = repo.list::<Asset>(None).await.map_err(|e| e.to_graphql_error())?;

        // Filter by manufacturer
        assets = assets
            .into_iter()
            .filter(|asset| asset.manufacturer_id == manufacturer_id)
            .collect();

        // Apply status filter if provided
        if let Some(status) = status_filter {
            let status_enum = AssetCurrentStatusOptions::from_string(&status).map_err(|e|
                e.to_graphql_error()
            )?;
            assets = assets
                .into_iter()
                .filter(|asset| asset.current_status.to_string() == status_enum.to_string())
                .collect();
        }

        // Apply limit if provided
        if let Some(limit_val) = limit {
            assets.truncate(limit_val as usize);
        }

        Ok(assets)
    }

    /// Get assets by status
    async fn assets_by_status(
        &self,
        ctx: &Context<'_>,
        status: String,
        limit: Option<i32>
    ) -> Result<Vec<Asset>, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        let status_enum = AssetCurrentStatusOptions::from_string(&status).map_err(|e|
            e.to_graphql_error()
        )?;

        let mut assets = repo.list::<Asset>(None).await.map_err(|e| e.to_graphql_error())?;

        // Filter by status
        assets = assets
            .into_iter()
            .filter(|asset| asset.current_status.to_string() == status_enum.to_string())
            .collect();

        // Apply limit if provided
        if let Some(limit_val) = limit {
            assets.truncate(limit_val as usize);
        }

        Ok(assets)
    }

    /// Get assets that are down (not operational)
    async fn assets_down(
        &self,
        ctx: &Context<'_>,
        limit: Option<i32>
    ) -> Result<Vec<Asset>, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        let mut assets = repo.list::<Asset>(None).await.map_err(|e| e.to_graphql_error())?;

        // Filter for down assets
        assets = assets
            .into_iter()
            .filter(|asset| matches!(asset.current_status, AssetCurrentStatusOptions::Down))
            .collect();

        // Apply limit if provided
        if let Some(limit_val) = limit {
            assets.truncate(limit_val as usize);
        }

        Ok(assets)
    }

    /// Get assets due for maintenance
    async fn assets_due_for_maintenance(
        &self,
        ctx: &Context<'_>,
        days_ahead: Option<i32>,
        limit: Option<i32>
    ) -> Result<Vec<Asset>, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        let mut assets = repo.list::<Asset>(None).await.map_err(|e| e.to_graphql_error())?;

        let days_ahead = days_ahead.unwrap_or(30); // Default to 30 days ahead
        let cutoff_date = Utc::now() + chrono::Duration::days(days_ahead as i64);

        // Filter for assets due for maintenance
        assets = assets
            .into_iter()
            .filter(|asset| {
                if let Some(next_maintenance) = asset.next_maintenance_due {
                    next_maintenance <= cutoff_date
                } else {
                    false
                }
            })
            .collect();

        // Apply limit if provided
        if let Some(limit_val) = limit {
            assets.truncate(limit_val as usize);
        }

        Ok(assets)
    }

    /// Get assets overdue for maintenance
    async fn assets_overdue_for_maintenance(
        &self,
        ctx: &Context<'_>,
        limit: Option<i32>
    ) -> Result<Vec<Asset>, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        let mut assets = repo.list::<Asset>(None).await.map_err(|e| e.to_graphql_error())?;

        let now = Utc::now();

        // Filter for overdue assets
        assets = assets
            .into_iter()
            .filter(|asset| {
                if let Some(next_maintenance) = asset.next_maintenance_due {
                    next_maintenance < now
                } else {
                    false
                }
            })
            .collect();

        // Apply limit if provided
        if let Some(limit_val) = limit {
            assets.truncate(limit_val as usize);
        }

        Ok(assets)
    }

    /// Get assets with work orders
    async fn assets_with_work_orders(
        &self,
        ctx: &Context<'_>,
        limit: Option<i32>,
        status_filter: Option<String>
    ) -> Result<Vec<Asset>, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        let mut assets = repo.list::<Asset>(None).await.map_err(|e| e.to_graphql_error())?;

        // Filter for assets with work orders
        assets = assets
            .into_iter()
            .filter(|asset| !asset.work_order_ids.is_empty())
            .collect();

        // Apply status filter if provided
        if let Some(status) = status_filter {
            let status_enum = AssetCurrentStatusOptions::from_string(&status).map_err(|e|
                e.to_graphql_error()
            )?;
            assets = assets
                .into_iter()
                .filter(|asset| asset.current_status.to_string() == status_enum.to_string())
                .collect();
        }

        // Apply limit if provided
        if let Some(limit_val) = limit {
            assets.truncate(limit_val as usize);
        }

        Ok(assets)
    }

    /// Get assets under warranty
    async fn assets_under_warranty(
        &self,
        ctx: &Context<'_>,
        limit: Option<i32>
    ) -> Result<Vec<Asset>, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        let mut assets = repo.list::<Asset>(None).await.map_err(|e| e.to_graphql_error())?;

        let now = Utc::now();

        // Filter for assets under warranty
        assets = assets
            .into_iter()
            .filter(|asset| {
                if let Some(warranty_end) = asset.warranty_end_date {
                    warranty_end > now
                } else {
                    false
                }
            })
            .collect();

        // Apply limit if provided
        if let Some(limit_val) = limit {
            assets.truncate(limit_val as usize);
        }

        Ok(assets)
    }

    /// Get assets with expiring warranties
    async fn assets_with_expiring_warranties(
        &self,
        ctx: &Context<'_>,
        days_ahead: Option<i32>,
        limit: Option<i32>
    ) -> Result<Vec<Asset>, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        let mut assets = repo.list::<Asset>(None).await.map_err(|e| e.to_graphql_error())?;

        let days_ahead = days_ahead.unwrap_or(90); // Default to 90 days ahead
        let cutoff_date = Utc::now() + chrono::Duration::days(days_ahead as i64);
        let now = Utc::now();

        // Filter for assets with warranties expiring soon
        assets = assets
            .into_iter()
            .filter(|asset| {
                if let Some(warranty_end) = asset.warranty_end_date {
                    warranty_end > now && warranty_end <= cutoff_date
                } else {
                    false
                }
            })
            .collect();

        // Apply limit if provided
        if let Some(limit_val) = limit {
            assets.truncate(limit_val as usize);
        }

        Ok(assets)
    }

    /// Get work orders for a specific asset
    async fn work_orders_for_asset(
        &self,
        ctx: &Context<'_>,
        asset_id: String,
        limit: Option<i32>
    ) -> Result<Vec<WorkOrder>, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        // Verify asset exists
        let _asset = repo
            .get::<Asset>(asset_id.clone()).await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| AppError::NotFound(format!("Asset {} not found", asset_id)))?;

        // Get all work orders and filter by asset_id
        let mut work_orders = repo.list::<WorkOrder>(None).await.map_err(|e| e.to_graphql_error())?;

        work_orders = work_orders
            .into_iter()
            .filter(|wo| wo.asset_id == asset_id)
            .collect();

        // Apply limit if provided
        if let Some(limit_val) = limit {
            work_orders.truncate(limit_val as usize);
        }

        Ok(work_orders)
    }

    /// Search assets by serial number
    async fn assets_by_serial_number(
        &self,
        ctx: &Context<'_>,
        serial_number: String,
        exact_match: Option<bool>
    ) -> Result<Vec<Asset>, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        let assets = repo.list::<Asset>(None).await.map_err(|e| e.to_graphql_error())?;

        let exact_match = exact_match.unwrap_or(false);

        let filtered_assets = if exact_match {
            assets
                .into_iter()
                .filter(|asset| asset.serial_number == serial_number)
                .collect()
        } else {
            assets
                .into_iter()
                .filter(|asset|
                    asset.serial_number.to_lowercase().contains(&serial_number.to_lowercase())
                )
                .collect()
        };

        Ok(filtered_assets)
    }

    /// Search assets by model number
    async fn assets_by_model_number(
        &self,
        ctx: &Context<'_>,
        model_number: String,
        exact_match: Option<bool>
    ) -> Result<Vec<Asset>, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        let assets = repo.list::<Asset>(None).await.map_err(|e| e.to_graphql_error())?;

        let exact_match = exact_match.unwrap_or(false);

        let filtered_assets = if exact_match {
            assets
                .into_iter()
                .filter(|asset| asset.model_number == model_number)
                .collect()
        } else {
            assets
                .into_iter()
                .filter(|asset|
                    asset.model_number.to_lowercase().contains(&model_number.to_lowercase())
                )
                .collect()
        };

        Ok(filtered_assets)
    }

    /// Get asset maintenance history (placeholder - would need maintenance records model)
    async fn asset_maintenance_history(
        &self,
        ctx: &Context<'_>,
        asset_id: String,
        limit: Option<i32>
    ) -> Result<Vec<WorkOrder>, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        // Verify asset exists
        let _asset = repo
            .get::<Asset>(asset_id.clone()).await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| AppError::NotFound(format!("Asset {} not found", asset_id)))?;

        // Get completed work orders for this asset as maintenance history
        let mut work_orders = repo.list::<WorkOrder>(None).await.map_err(|e| e.to_graphql_error())?;

        work_orders = work_orders
            .into_iter()
            .filter(|wo| wo.asset_id == asset_id && wo.is_completed())
            .collect();

        // Sort by completion date (most recent first)
        work_orders.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));

        // Apply limit if provided
        if let Some(limit_val) = limit {
            work_orders.truncate(limit_val as usize);
        }

        Ok(work_orders)
    }
}
