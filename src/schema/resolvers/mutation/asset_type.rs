use crate::{
    models::{ asset_type::{ AssetType, AssetTypeCategory }, prelude::* },
    AppError,
    DbClient,
    Repository,
};

#[derive(Debug, Default)]
pub(crate) struct AssetTypeMutation;

#[Object]
impl AssetTypeMutation {
    /// Create a new asset type
    async fn create_asset_type(
        &self,
        ctx: &Context<'_>,
        name: String,
        description: String,
        category: String
    ) -> Result<AssetType, Error> {
        // info!("Creating new asset_type: {}", name);

        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let id = format!("asset_type-{}", Uuid::new_v4());

        let asset_type = AssetType::new(id, name, description, category).map_err(|e|
            e.to_graphql_error()
        )?;

        // info!("Creating new asset_type: {:?}", asset_type);

        asset_type.validate().map_err(|e| { AppError::ValidationError(e).to_graphql_error() })?;
        Repository::new(db_client.clone())
            .create(asset_type).await
            .map_err(|e| e.to_graphql_error())
        // Validate before saving
    }

    /// Update an existing asset type
    async fn update_asset_type(
        &self,
        ctx: &Context<'_>,
        id: String,
        name: Option<String>,
        description: Option<String>,
        category: Option<String>
    ) -> Result<AssetType, Error> {
        // info!("Updating asset_type: {}", id);

        let db_client = ctx
            .data::<DbClient>()
            .map_err(|_| {
                AppError::InternalServerError("Database client not available".to_string())
            })?;

        let repo = Repository::new(db_client.clone());

        let mut asset_type = repo
            .get::<AssetType>(id.clone()).await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| AppError::NotFound(format!("Asset type {} not found", id)))?;

        if let Some(name) = name {
            asset_type.name = name;
        }
        if let Some(description) = description {
            asset_type.description = description;
        }
        if let Some(category) = category {
            asset_type.category = AssetTypeCategory::from_string(&category)?;
        }

        asset_type.updated_at = chrono::Utc::now();

        asset_type.validate().map_err(|e| AppError::ValidationError(e))?;

        repo.update(asset_type).await.map_err(|e| e.to_graphql_error())
    }

    /// Delete an asset type
    async fn delete_asset_type(&self, ctx: &Context<'_>, id: String) -> Result<bool, Error> {
        // info!("Deleting asset_type: {}", id);

        let db_client = ctx
            .data::<DbClient>()
            .map_err(|_| {
                AppError::InternalServerError("Database client not available".to_string())
            })?;

        let repo = Repository::new(db_client.clone());

        let _ = repo
            .get::<AssetType>(id.clone()).await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| AppError::NotFound(format!("Asset type {} not found", id)))?;

        repo.delete::<AssetType>(id).await.map_err(|e| e.to_graphql_error())
    }
}
