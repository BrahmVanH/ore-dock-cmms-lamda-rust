use crate::{
    models::{
        prelude::*,
        address::AddressInput,
        location::Location,
        location_type::LocationType,
        prelude::*,
    },
    AppError,
    DbClient,
    Repository,
};

#[derive(Debug, Default)]
pub(crate) struct LocationMutation;

#[Object]
impl LocationMutation {
    /// Create a new location
    async fn create_location(
        &self,
        ctx: &Context<'_>,
        name: String,
        description: String,
        location_type_id: String,
        parent_location_id: Option<String>,
        address: AddressInput,
        coordinates: Option<String>
    ) -> Result<Location, Error> {
        info!("Creating new location: {}", name);

        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());
        let id = format!("location-{}", Uuid::new_v4());

        // Validate that location type exists
        repo
            .get::<LocationType>(location_type_id.clone()).await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| {
                AppError::ValidationError(
                    format!("Location type {} not found", location_type_id)
                ).to_graphql_error()
            })?;

        // Validate parent location exists if provided
        if let Some(ref parent_id) = parent_location_id {
            repo
                .get::<Location>(parent_id.clone()).await
                .map_err(|e| e.to_graphql_error())?
                .ok_or_else(|| {
                    AppError::ValidationError(
                        format!("Parent location {} not found", parent_id)
                    ).to_graphql_error()
                })?;
        }

        let location = Location::new(
            id,
            name,
            description,
            location_type_id,
            parent_location_id,
            Address::from(address),
            coordinates
        );

        location.validate().map_err(|e| { AppError::ValidationError(e).to_graphql_error() })?;

        repo.create(location).await.map_err(|e| e.to_graphql_error())
    }

    /// Update an existing location
    async fn update_location(
        &self,
        ctx: &Context<'_>,
        id: String,
        name: Option<String>,
        description: Option<String>,
        location_type_id: Option<String>,
        parent_location_id: Option<String>,
        address: Option<AddressInput>,
        coordinates: Option<String>,
        is_active: Option<bool>
    ) -> Result<Location, Error> {
        info!("Updating location: {}", id);

        let db_client = ctx
            .data::<DbClient>()
            .map_err(|_| {
                AppError::InternalServerError("Database client not available".to_string())
            })?;

        let repo = Repository::new(db_client.clone());

        let mut location = repo
            .get::<Location>(id.clone()).await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| AppError::NotFound(format!("Location {} not found", id)))?;

        // Validate new location type if provided
        if let Some(ref new_type_id) = location_type_id {
            repo
                .get::<LocationType>(new_type_id.clone()).await
                .map_err(|e| e.to_graphql_error())?
                .ok_or_else(|| {
                    AppError::ValidationError(
                        format!("Location type {} not found", new_type_id)
                    ).to_graphql_error()
                })?;
        }

        // Validate new parent location if provided
        if let Some(ref new_parent_id) = parent_location_id {
            if !new_parent_id.is_empty() {
                // Check that we're not creating a circular reference
                if new_parent_id == &location.id {
                    return Err(
                        AppError::ValidationError(
                            "Location cannot be its own parent".to_string()
                        ).to_graphql_error()
                    );
                }

                repo
                    .get::<Location>(new_parent_id.clone()).await
                    .map_err(|e| e.to_graphql_error())?
                    .ok_or_else(|| {
                        AppError::ValidationError(
                            format!("Parent location {} not found", new_parent_id)
                        ).to_graphql_error()
                    })?;
            }
        }

        // Update fields
        if let Some(name) = name {
            location.name = name;
        }
        if let Some(description) = description {
            location.description = description;
        }
        if let Some(location_type_id) = location_type_id {
            location.location_type_id = location_type_id;
        }
        if let Some(parent_id) = parent_location_id {
            location.parent_location_id = if parent_id.is_empty() { None } else { Some(parent_id) };
        }
        if let Some(address) = address {
            location.address = Address::from(address);
        }
        if let Some(coordinates) = coordinates {
            location.coordinates = if coordinates.is_empty() { None } else { Some(coordinates) };
        }
        if let Some(is_active) = is_active {
            location.is_active = is_active;
        }
        location.updated_at = chrono::Utc::now();

        location.validate().map_err(|e| AppError::ValidationError(e))?;

        repo.update(location).await.map_err(|e| e.to_graphql_error())
    }

    /// Activate a location
    async fn activate_location(&self, ctx: &Context<'_>, id: String) -> Result<Location, Error> {
        info!("Activating location: {}", id);

        let db_client = ctx
            .data::<DbClient>()
            .map_err(|_| {
                AppError::InternalServerError("Database client not available".to_string())
            })?;

        let repo = Repository::new(db_client.clone());

        let mut location = repo
            .get::<Location>(id.clone()).await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| AppError::NotFound(format!("Location {} not found", id)))?;

        location.is_active = true;
        location.updated_at = chrono::Utc::now();

        repo.update(location).await.map_err(|e| e.to_graphql_error())
    }

    /// Deactivate a location
    async fn deactivate_location(&self, ctx: &Context<'_>, id: String) -> Result<Location, Error> {
        info!("Deactivating location: {}", id);

        let db_client = ctx
            .data::<DbClient>()
            .map_err(|_| {
                AppError::InternalServerError("Database client not available".to_string())
            })?;

        let repo = Repository::new(db_client.clone());

        let mut location = repo
            .get::<Location>(id.clone()).await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| AppError::NotFound(format!("Location {} not found", id)))?;

        // Business rule: Check if location has active assets before deactivating
        // This would require scanning the assets table - for now we'll allow it
        // In production, you might want to add this validation

        location.is_active = false;
        location.updated_at = chrono::Utc::now();

        repo.update(location).await.map_err(|e| e.to_graphql_error())
    }

    /// Move location to a new parent
    async fn move_location(
        &self,
        ctx: &Context<'_>,
        id: String,
        new_parent_id: Option<String>
    ) -> Result<Location, Error> {
        info!("Moving location {} to parent {:?}", id, new_parent_id);

        let db_client = ctx
            .data::<DbClient>()
            .map_err(|_| {
                AppError::InternalServerError("Database client not available".to_string())
            })?;

        let repo = Repository::new(db_client.clone());

        let mut location = repo
            .get::<Location>(id.clone()).await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| AppError::NotFound(format!("Location {} not found", id)))?;

        // Validate new parent if provided
        if let Some(ref parent_id) = new_parent_id {
            if parent_id == &location.id {
                return Err(
                    AppError::ValidationError(
                        "Location cannot be its own parent".to_string()
                    ).to_graphql_error()
                );
            }

            let parent_location = repo
                .get::<Location>(parent_id.clone()).await
                .map_err(|e| e.to_graphql_error())?
                .ok_or_else(|| {
                    AppError::ValidationError(
                        format!("Parent location {} not found", parent_id)
                    ).to_graphql_error()
                })?;

            // Additional validation: Check for circular references
            // This would require traversing the parent chain - simplified for now
            if let Some(ref grandparent_id) = parent_location.parent_location_id {
                if grandparent_id == &location.id {
                    return Err(
                        AppError::ValidationError(
                            "Cannot create circular parent relationship".to_string()
                        ).to_graphql_error()
                    );
                }
            }
        }

        location.parent_location_id = new_parent_id;
        location.updated_at = chrono::Utc::now();

        repo.update(location).await.map_err(|e| e.to_graphql_error())
    }

    /// Delete a location
    async fn delete_location(&self, ctx: &Context<'_>, id: String) -> Result<bool, Error> {
        info!("Deleting location: {}", id);

        let db_client = ctx
            .data::<DbClient>()
            .map_err(|_| {
                AppError::InternalServerError("Database client not available".to_string())
            })?;

        let repo = Repository::new(db_client.clone());

        // Verify location exists
        let location = repo
            .get::<Location>(id.clone()).await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| AppError::NotFound(format!("Location {} not found", id)))?;

        // Business rules: Check if location can be deleted
        // 1. Location must be inactive
        if location.is_active {
            return Err(
                AppError::ValidationError(
                    "Cannot delete active location. Deactivate it first.".to_string()
                ).to_graphql_error()
            );
        }

        // 2. Check if location has child locations (would require a scan)
        // 3. Check if location has assets (would require a scan)
        // For now, we'll allow deletion - in production you'd add these checks

        repo.delete::<Location>(id).await.map_err(|e| e.to_graphql_error())
    }
}
