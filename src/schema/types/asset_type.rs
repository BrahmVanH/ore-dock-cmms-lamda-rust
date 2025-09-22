use crate::models::{ asset_type::{ AssetType, AssetTypeCategory }, prelude::* };
#[Object]
impl AssetType {
    async fn id(&self) -> &str {
        &self.id
    }

    async fn name(&self) -> &str {
        &self.name
    }

    async fn description(&self) -> &str {
        &self.description
    }
    async fn category(&self) -> AssetTypeCategory {
        self.category
    }

    async fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }

    async fn updated_at(&self) -> &DateTime<Utc> {
        &self.updated_at
    }
}
