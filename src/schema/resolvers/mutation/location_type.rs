use async_graphql::*;
use tracing::{ info, warn };
use uuid::Uuid;

use crate::{ DbClient, models::location_type::LocationType, AppError, Repository };

#[derive(Debug, Default)]
pub(crate) struct LocationTypeMutation;

#[Object]
impl LocationTypeMutation {
    /// Create a new location type
    async fn create_location_type(
        &self,
        ctx: &Context<'_>,
        name: String,
        description: String
    ) -> Result<LocationType, Error> {
        info!("Creating new location_type: {}", name);

        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let id = Uuid::new_v4().to_string();

        info!("new location_type id: {:?}", &id);

        let location_type = LocationType::new(id, name, description);
        
        info!("new location_type : {:?}", &location_type);

        location_type.validate().map_err(|e| { AppError::ValidationError(e).to_graphql_error() })?;

        Repository::new(db_client.clone())
            .create(location_type).await
            .map_err(|e| e.to_graphql_error())
    }

    /// Update an existing location type
    async fn update_location_type(
        &self,
        ctx: &Context<'_>,
        id: String,
        name: Option<String>,
        description: Option<String>
    ) -> Result<LocationType, Error> {
        info!("Updating location_type: {}", id);

        let db_client = ctx
            .data::<DbClient>()
            .map_err(|_| {
                AppError::InternalServerError("Database client not available".to_string())
            })?;

        let repo = Repository::new(db_client.clone());

        let mut location_type = repo
            .get::<LocationType>(id.clone()).await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| AppError::NotFound(format!("Location type {} not found", id)))?;

        if let Some(name) = name {
            location_type.name = name;
        }
        if let Some(description) = description {
            location_type.description = description;
        }
        location_type.updated_at = chrono::Utc::now();

        location_type.validate().map_err(|e| AppError::ValidationError(e))?;

        repo.update(location_type).await.map_err(|e| e.to_graphql_error())
    }

    /// Delete a location type
    async fn delete_location_type(&self, ctx: &Context<'_>, id: String) -> Result<bool, Error> {
        info!("Deleting location_type: {}", id);

        let db_client = ctx
            .data::<DbClient>()
            .map_err(|_| {
                AppError::InternalServerError("Database client not available".to_string())
            })?;

        let repo = Repository::new(db_client.clone());

        // Verify location type exists
        let _ = repo
            .get::<LocationType>(id.clone()).await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| AppError::NotFound(format!("Location type {} not found", id)))?;

        // Optional: Check if any locations are using this location type
        // This would require a scan of the locations table
        // For now, we'll allow deletion - in production you might want to prevent this

        repo.delete::<LocationType>(id).await.map_err(|e| e.to_graphql_error())
    }
}
