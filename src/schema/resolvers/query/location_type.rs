use async_graphql::*;
use tracing::warn;

use crate::{
    error::AppError,
    models::{ location_type::LocationType, location::Location },
    DbClient,
    Repository,
};

#[derive(Debug, Default)]
pub(crate) struct LocationTypeQuery;

#[Object]
impl LocationTypeQuery {
    /// Get location type by ID
    async fn location_type_by_id(
        &self,
        ctx: &Context<'_>,
        id: String
    ) -> Result<Option<LocationType>, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        repo.get::<LocationType>(id).await.map_err(|e| e.to_graphql_error())
    }

    /// Get all location types
    async fn location_types(
        &self,
        ctx: &Context<'_>,
        limit: Option<i32>
    ) -> Result<Vec<LocationType>, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        repo.list::<LocationType>(limit).await.map_err(|e| e.to_graphql_error())
    }

    /// Search location types by name
    async fn location_types_by_name(
        &self,
        ctx: &Context<'_>,
        name: String,
        exact_match: Option<bool>,
        limit: Option<i32>
    ) -> Result<Vec<LocationType>, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        let location_types = repo
            .list::<LocationType>(None).await
            .map_err(|e| e.to_graphql_error())?;

        let exact_match = exact_match.unwrap_or(false);

        let mut filtered_types: Vec<LocationType> = if exact_match {
            location_types
                .into_iter()
                .filter(|lt| lt.name.to_lowercase() == name.to_lowercase())
                .collect()
        } else {
            location_types
                .into_iter()
                .filter(|lt| lt.name.to_lowercase().contains(&name.to_lowercase()))
                .collect()
        };

        // Apply limit if provided
        if let Some(limit_val) = limit {
            filtered_types.truncate(limit_val as usize);
        }

        Ok(filtered_types)
    }

    /// Search location types by description
    async fn location_types_by_description(
        &self,
        ctx: &Context<'_>,
        description_search: String,
        limit: Option<i32>
    ) -> Result<Vec<LocationType>, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        let location_types = repo
            .list::<LocationType>(None).await
            .map_err(|e| e.to_graphql_error())?;

        let mut filtered_types: Vec<LocationType> = location_types
            .into_iter()
            .filter(|lt| {
                lt.description.to_lowercase().contains(&description_search.to_lowercase())
            })
            .collect();

        // Apply limit if provided
        if let Some(limit_val) = limit {
            filtered_types.truncate(limit_val as usize);
        }

        Ok(filtered_types)
    }

    /// Get locations that use a specific location type
    async fn locations_for_location_type(
        &self,
        ctx: &Context<'_>,
        location_type_id: String,
        active_only: Option<bool>,
        limit: Option<i32>
    ) -> Result<Vec<Location>, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        // Verify location type exists
        let _location_type = repo
            .get::<LocationType>(location_type_id.clone()).await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| {
                AppError::NotFound(
                    format!("Location type {} not found", location_type_id)
                ).to_graphql_error()
            })?;

        // Get all locations and filter by location_type_id
        let mut locations = repo.list::<Location>(None).await.map_err(|e| e.to_graphql_error())?;

        locations = locations
            .into_iter()
            .filter(|location| location.location_type_id == location_type_id)
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

        // Apply limit if provided
        if let Some(limit_val) = limit {
            locations.truncate(limit_val as usize);
        }

        Ok(locations)
    }

    /// Get location type usage statistics
    async fn location_type_usage_stats(
        &self,
        ctx: &Context<'_>,
        location_type_id: String
    ) -> Result<LocationTypeUsageStats, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        // Verify location type exists
        let location_type = repo
            .get::<LocationType>(location_type_id.clone()).await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| {
                AppError::NotFound(
                    format!("Location type {} not found", location_type_id)
                ).to_graphql_error()
            })?;

        // Get all locations for this type
        let locations = repo.list::<Location>(None).await.map_err(|e| e.to_graphql_error())?;

        let type_locations: Vec<Location> = locations
            .into_iter()
            .filter(|location| location.location_type_id == location_type_id)
            .collect();

        let total_locations = type_locations.len() as i32;
        let active_locations = type_locations
            .iter()
            .filter(|l| l.is_active)
            .count() as i32;
        let inactive_locations = total_locations - active_locations;

        Ok(LocationTypeUsageStats {
            location_type,
            total_locations,
            active_locations,
            inactive_locations,
        })
    }

    /// Get all location types with their usage counts
    async fn location_types_with_usage(
        &self,
        ctx: &Context<'_>,
        limit: Option<i32>
    ) -> Result<Vec<LocationTypeWithUsage>, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        let mut location_types = repo
            .list::<LocationType>(limit).await
            .map_err(|e| e.to_graphql_error())?;

        let all_locations = repo.list::<Location>(None).await.map_err(|e| e.to_graphql_error())?;

        let mut result = Vec::new();

        for location_type in location_types {
            let type_locations: Vec<&Location> = all_locations
                .iter()
                .filter(|location| location.location_type_id == location_type.id)
                .collect();

            let total_count = type_locations.len() as i32;
            let active_count = type_locations
                .iter()
                .filter(|l| l.is_active)
                .count() as i32;

            result.push(LocationTypeWithUsage {
                location_type,
                total_locations: total_count,
                active_locations: active_count,
            });
        }

        Ok(result)
    }

    /// Get unused location types (no locations assigned)
    async fn unused_location_types(
        &self,
        ctx: &Context<'_>,
        limit: Option<i32>
    ) -> Result<Vec<LocationType>, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        let location_types = repo
            .list::<LocationType>(None).await
            .map_err(|e| e.to_graphql_error())?;

        let all_locations = repo.list::<Location>(None).await.map_err(|e| e.to_graphql_error())?;

        // Get location type IDs that are in use
        let used_type_ids: std::collections::HashSet<String> = all_locations
            .iter()
            .map(|location| location.location_type_id.clone())
            .collect();

        let mut unused_types: Vec<LocationType> = location_types
            .into_iter()
            .filter(|lt| !used_type_ids.contains(&lt.id))
            .collect();

        // Apply limit if provided
        if let Some(limit_val) = limit {
            unused_types.truncate(limit_val as usize);
        }

        Ok(unused_types)
    }
}

/// Statistics for location type usage
#[derive(Debug)]
pub struct LocationTypeUsageStats {
    pub location_type: LocationType,
    pub total_locations: i32,
    pub active_locations: i32,
    pub inactive_locations: i32,
}

#[Object]
impl LocationTypeUsageStats {
    async fn location_type(&self) -> &LocationType {
        &self.location_type
    }

    async fn total_locations(&self) -> i32 {
        self.total_locations
    }

    async fn active_locations(&self) -> i32 {
        self.active_locations
    }

    async fn inactive_locations(&self) -> i32 {
        self.inactive_locations
    }

    async fn usage_percentage(&self) -> f64 {
        if self.total_locations == 0 {
            0.0
        } else {
            ((self.active_locations as f64) / (self.total_locations as f64)) * 100.0
        }
    }
}

/// Location type with usage information
#[derive(Debug)]
pub struct LocationTypeWithUsage {
    pub location_type: LocationType,
    pub total_locations: i32,
    pub active_locations: i32,
}

#[Object]
impl LocationTypeWithUsage {
    async fn location_type(&self) -> &LocationType {
        &self.location_type
    }

    async fn total_locations(&self) -> i32 {
        self.total_locations
    }

    async fn active_locations(&self) -> i32 {
        self.active_locations
    }

    async fn inactive_locations(&self) -> i32 {
        self.total_locations - self.active_locations
    }
}
