use async_graphql::*;
use tracing::warn;

use crate::{
    error::AppError,
    models::location::Location,
    DbClient,
    Repository,
};

#[derive(Debug, Default)]
pub(crate) struct LocationQuery;

#[Object]
impl LocationQuery {
    /// Get location by ID
    async fn location_by_id(
        &self,
        ctx: &Context<'_>,
        id: String
    ) -> Result<Option<Location>, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        repo.get::<Location>(id).await.map_err(|e| e.to_graphql_error())
    }

    /// Get all locations
    async fn locations(
        &self,
        ctx: &Context<'_>,
        limit: Option<i32>,
        active_only: Option<bool>
    ) -> Result<Vec<Location>, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        let mut locations = repo.list::<Location>(limit).await.map_err(|e| e.to_graphql_error())?;

        // Filter by active status if requested
        if let Some(true) = active_only {
            locations = locations
                .into_iter()
                .filter(|l| l.is_active)
                .collect();
        } else if let Some(false) = active_only {
            locations = locations
                .into_iter()
                .filter(|l| !l.is_active)
                .collect();
        }

        Ok(locations)
    }

    /// Get locations by type
    async fn locations_by_type(
        &self,
        ctx: &Context<'_>,
        location_type_id: String,
        active_only: Option<bool>
    ) -> Result<Vec<Location>, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        // This would ideally use a GSI on location_type_id
        // For now, we'll scan and filter (not ideal for production)
        let mut locations = repo.list::<Location>(None).await.map_err(|e| e.to_graphql_error())?;

        locations = locations
            .into_iter()
            .filter(|l| l.location_type_id == location_type_id)
            .collect();

        // Filter by active status if requested
        if let Some(true) = active_only {
            locations = locations
                .into_iter()
                .filter(|l| l.is_active)
                .collect();
        } else if let Some(false) = active_only {
            locations = locations
                .into_iter()
                .filter(|l| !l.is_active)
                .collect();
        }

        Ok(locations)
    }

    /// Get child locations of a parent
    async fn child_locations(
        &self,
        ctx: &Context<'_>,
        parent_id: String,
        active_only: Option<bool>
    ) -> Result<Vec<Location>, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        // This would ideally use a GSI on parent_location_id
        // For now, we'll scan and filter (not ideal for production)
        let mut locations = repo.list::<Location>(None).await.map_err(|e| e.to_graphql_error())?;

        locations = locations
            .into_iter()
            .filter(|l| l.parent_location_id.as_ref() == Some(&parent_id))
            .collect();

        // Filter by active status if requested
        if let Some(true) = active_only {
            locations = locations
                .into_iter()
                .filter(|l| l.is_active)
                .collect();
        } else if let Some(false) = active_only {
            locations = locations
                .into_iter()
                .filter(|l| !l.is_active)
                .collect();
        }

        Ok(locations)
    }
}
