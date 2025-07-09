use async_graphql::*;
use tracing::warn;

use crate::{
    error::AppError,
    models::{
        manufacturer::Manufacturer,
        asset::Asset,
    },
    DbClient,
    Repository,
};

#[derive(Debug, Default)]
pub(crate) struct ManufacturerQuery;

#[Object]
impl ManufacturerQuery {
    async fn manufacturer_by_id(
        &self,
        ctx: &Context<'_>,
        id: String,
    ) -> Result<Option<Manufacturer>, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        repo.get::<Manufacturer>(id)
            .await
            .map_err(|e| e.to_graphql_error())
    }

    async fn manufacturers(
        &self,
        ctx: &Context<'_>,
        limit: Option<i32>,
        active_only: Option<bool>,
    ) -> Result<Vec<Manufacturer>, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        let mut manufacturers = repo
            .list::<Manufacturer>(limit)
            .await
            .map_err(|e| e.to_graphql_error())?;

        if let Some(true) = active_only {
            manufacturers = manufacturers
                .into_iter()
                .filter(|m| m.active)
                .collect();
        }

        manufacturers.sort_by(|a, b| a.name.cmp(&b.name));

        Ok(manufacturers)
    }

    async fn manufacturers_by_name(
        &self,
        ctx: &Context<'_>,
        name: String,
        exact_match: Option<bool>,
        active_only: Option<bool>,
        limit: Option<i32>,
    ) -> Result<Vec<Manufacturer>, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        let mut manufacturers = repo
            .list::<Manufacturer>(None)
            .await
            .map_err(|e| e.to_graphql_error())?;

        let exact_match = exact_match.unwrap_or(false);

        manufacturers = if exact_match {
            manufacturers
                .into_iter()
                .filter(|m| m.name.to_lowercase() == name.to_lowercase())
                .collect()
        } else {
            manufacturers
                .into_iter()
                .filter(|m| m.name.to_lowercase().contains(&name.to_lowercase()))
                .collect()
        };

        if let Some(true) = active_only {
            manufacturers = manufacturers
                .into_iter()
                .filter(|m| m.active)
                .collect();
        }

        manufacturers.sort_by(|a, b| a.name.cmp(&b.name));

        if let Some(limit_val) = limit {
            manufacturers.truncate(limit_val as usize);
        }

        Ok(manufacturers)
    }

    async fn manufacturers_by_country(
        &self,
        ctx: &Context<'_>,
        country: String,
        active_only: Option<bool>,
        limit: Option<i32>,
    ) -> Result<Vec<Manufacturer>, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        let mut manufacturers = repo
            .list::<Manufacturer>(None)
            .await
            .map_err(|e| e.to_graphql_error())?;

        manufacturers = manufacturers
            .into_iter()
            .filter(|m| m.address.country.to_lowercase() == country.to_lowercase())
            .collect();

        if let Some(true) = active_only {
            manufacturers = manufacturers
                .into_iter()
                .filter(|m| m.active)
                .collect();
        }

        manufacturers.sort_by(|a, b| a.name.cmp(&b.name));

        if let Some(limit_val) = limit {
            manufacturers.truncate(limit_val as usize);
        }

        Ok(manufacturers)
    }

    async fn manufacturers_by_state(
        &self,
        ctx: &Context<'_>,
        state: String,
        active_only: Option<bool>,
        limit: Option<i32>,
    ) -> Result<Vec<Manufacturer>, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        let mut manufacturers = repo
            .list::<Manufacturer>(None)
            .await
            .map_err(|e| e.to_graphql_error())?;

        manufacturers = manufacturers
            .into_iter()
            .filter(|m| m.address.state.to_lowercase() == state.to_lowercase())
            .collect();

        if let Some(true) = active_only {
            manufacturers = manufacturers
                .into_iter()
                .filter(|m| m.active)
                .collect();
        }

        manufacturers.sort_by(|a, b| a.name.cmp(&b.name));

        if let Some(limit_val) = limit {
            manufacturers.truncate(limit_val as usize);
        }

        Ok(manufacturers)
    }

    async fn manufacturers_by_city(
        &self,
        ctx: &Context<'_>,
        city: String,
        active_only: Option<bool>,
        limit: Option<i32>,
    ) -> Result<Vec<Manufacturer>, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        let mut manufacturers = repo
            .list::<Manufacturer>(None)
            .await
            .map_err(|e| e.to_graphql_error())?;

        manufacturers = manufacturers
            .into_iter()
            .filter(|m| m.address.city.to_lowercase() == city.to_lowercase())
            .collect();

        if let Some(true) = active_only {
            manufacturers = manufacturers
                .into_iter()
                .filter(|m| m.active)
                .collect();
        }

        manufacturers.sort_by(|a, b| a.name.cmp(&b.name));

        if let Some(limit_val) = limit {
            manufacturers.truncate(limit_val as usize);
        }

        Ok(manufacturers)
    }

    async fn manufacturers_with_support_contact(
        &self,
        ctx: &Context<'_>,
        active_only: Option<bool>,
        limit: Option<i32>,
    ) -> Result<Vec<Manufacturer>, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        let mut manufacturers = repo
            .list::<Manufacturer>(None)
            .await
            .map_err(|e| e.to_graphql_error())?;

        manufacturers = manufacturers
            .into_iter()
            .filter(|m| m.support_contact.is_some())
            .collect();

        if let Some(true) = active_only {
            manufacturers = manufacturers
                .into_iter()
                .filter(|m| m.active)
                .collect();
        }

        manufacturers.sort_by(|a, b| a.name.cmp(&b.name));

        if let Some(limit_val) = limit {
            manufacturers.truncate(limit_val as usize);
        }

        Ok(manufacturers)
    }

    async fn manufacturers_with_warranty_contact(
        &self,
        ctx: &Context<'_>,
        active_only: Option<bool>,
        limit: Option<i32>,
    ) -> Result<Vec<Manufacturer>, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        let mut manufacturers = repo
            .list::<Manufacturer>(None)
            .await
            .map_err(|e| e.to_graphql_error())?;

        manufacturers = manufacturers
            .into_iter()
            .filter(|m| m.warranty_contact.is_some())
            .collect();

        if let Some(true) = active_only {
            manufacturers = manufacturers
                .into_iter()
                .filter(|m| m.active)
                .collect();
        }

        manufacturers.sort_by(|a, b| a.name.cmp(&b.name));

        if let Some(limit_val) = limit {
            manufacturers.truncate(limit_val as usize);
        }

        Ok(manufacturers)
    }

    async fn manufacturers_with_websites(
        &self,
        ctx: &Context<'_>,
        active_only: Option<bool>,
        limit: Option<i32>,
    ) -> Result<Vec<Manufacturer>, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        let mut manufacturers = repo
            .list::<Manufacturer>(None)
            .await
            .map_err(|e| e.to_graphql_error())?;

        manufacturers = manufacturers
            .into_iter()
            .filter(|m| m.website.is_some())
            .collect();

        if let Some(true) = active_only {
            manufacturers = manufacturers
                .into_iter()
                .filter(|m| m.active)
                .collect();
        }

        manufacturers.sort_by(|a, b| a.name.cmp(&b.name));

        if let Some(limit_val) = limit {
            manufacturers.truncate(limit_val as usize);
        }

        Ok(manufacturers)
    }

    async fn manufacturer_statistics(
        &self,
        ctx: &Context<'_>,
    ) -> Result<ManufacturerStatistics, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        let manufacturers = repo
            .list::<Manufacturer>(None)
            .await
            .map_err(|e| e.to_graphql_error())?;

        let total_manufacturers = manufacturers.len() as i32;
        let active_manufacturers = manufacturers.iter().filter(|m| m.active).count() as i32;
        let with_support_contact = manufacturers.iter().filter(|m| m.support_contact.is_some()).count() as i32;
        let with_warranty_contact = manufacturers.iter().filter(|m| m.warranty_contact.is_some()).count() as i32;
        let with_websites = manufacturers.iter().filter(|m| m.website.is_some()).count() as i32;

        let unique_countries: std::collections::HashSet<_> = manufacturers
            .iter()
            .map(|m| &m.address.country)
            .collect();
        let countries_count = unique_countries.len() as i32;

        Ok(ManufacturerStatistics {
            total_manufacturers,
            active_manufacturers,
            with_support_contact,
            with_warranty_contact,
            with_websites,
            countries_count,
        })
    }

    async fn assets_by_manufacturer(
        &self,
        ctx: &Context<'_>,
        manufacturer_id: String,
        limit: Option<i32>,
    ) -> Result<Vec<Asset>, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        let _manufacturer = repo
            .get::<Manufacturer>(manufacturer_id.clone())
            .await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| {
                AppError::NotFound(format!("Manufacturer {} not found", manufacturer_id))
                    .to_graphql_error()
            })?;

        let mut assets = repo
            .list::<Asset>(None)
            .await
            .map_err(|e| e.to_graphql_error())?;

        assets = assets
            .into_iter()
            .filter(|a| a.manufacturer_id == manufacturer_id)
            .collect();

        assets.sort_by(|a, b| a.name.cmp(&b.name));

        if let Some(limit_val) = limit {
            assets.truncate(limit_val as usize);
        }

        Ok(assets)
    }

    async fn manufacturer_asset_count(
        &self,
        ctx: &Context<'_>,
        manufacturer_id: String,
    ) -> Result<i32, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        let _manufacturer = repo
            .get::<Manufacturer>(manufacturer_id.clone())
            .await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| {
                AppError::NotFound(format!("Manufacturer {} not found", manufacturer_id))
                    .to_graphql_error()
            })?;

        let assets = repo
            .list::<Asset>(None)
            .await
            .map_err(|e| e.to_graphql_error())?;

        let count = assets
            .iter()
            .filter(|a| a.manufacturer_id == manufacturer_id)
            .count() as i32;

        Ok(count)
    }
}

#[derive(Debug)]
pub struct ManufacturerStatistics {
    pub total_manufacturers: i32,
    pub active_manufacturers: i32,
    pub with_support_contact: i32,
    pub with_warranty_contact: i32,
    pub with_websites: i32,
    pub countries_count: i32,
}

#[Object]
impl ManufacturerStatistics {
    async fn total_manufacturers(&self) -> i32 {
        self.total_manufacturers
    }

    async fn active_manufacturers(&self) -> i32 {
        self.active_manufacturers
    }

    async fn with_support_contact(&self) -> i32 {
        self.with_support_contact
    }

    async fn with_warranty_contact(&self) -> i32 {
        self.with_warranty_contact
    }

    async fn with_websites(&self) -> i32 {
        self.with_websites
    }

    async fn countries_count(&self) -> i32 {
        self.countries_count
    }

    async fn inactive_manufacturers(&self) -> i32 {
        self.total_manufacturers - self.active_manufacturers
    }

    async fn active_percentage(&self) -> f64 {
        if self.total_manufacturers == 0 {
            0.0
        } else {
            (self.active_manufacturers as f64 / self.total_manufacturers as f64) * 100.0
        }
    }

    async fn support_contact_percentage(&self) -> f64 {
        if self.total_manufacturers == 0 {
            0.0
        } else {
            (self.with_support_contact as f64 / self.total_manufacturers as f64) * 100.0
        }
    }

    async fn warranty_contact_percentage(&self) -> f64 {
        if self.total_manufacturers == 0 {
            0.0
        } else {
            (self.with_warranty_contact as f64 / self.total_manufacturers as f64) * 100.0
        }
    }
}