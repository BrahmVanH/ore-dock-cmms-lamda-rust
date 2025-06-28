use async_graphql::*;
use tracing::{ info, warn };

use crate::{
    error::AppError,
    models::asset_type::AssetType,
    schema::resolvers::query::QueryRoot,
    DbClient,
    Repository,
};

#[Object]
impl QueryRoot {
    pub(crate) async fn get_asset_by_id(
        &self,
        ctx: &Context<'_>,
        id: String
    ) -> Result<AssetType, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        let asset_type = repo
            .get::<AssetType>(id.clone()).await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| AppError::NotFound(format!("Asset type {} not found", id)))?;

        Ok(asset_type)
    }
    pub(crate) async fn get_all(&self, ctx: &Context<'_>) -> Result<Vec<AssetType>, Error> {
        let table_name = "AssetTypes";
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let response = db_client
            .scan()
            .table_name(table_name)
            .send().await
            .map_err(|e| {
                warn!("Failed to get db_client from context: {:?}", e);
                AppError::DatabaseError(
                    "Failed to get all users from db".to_string()
                ).to_graphql_error()
            })?;

        info!("get all users response: {:?}", response);

        let asset_types = response
            .items()
            .iter()
            .filter_map(|item| AssetType::from_item(item))
            .collect::<Vec<AssetType>>();

        info!("asset_types from response items: {:?}", asset_types);

        Ok(asset_types)
    }
}
