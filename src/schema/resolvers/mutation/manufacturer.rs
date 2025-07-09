use async_graphql::*;
use tracing::{ info, warn };
use uuid::Uuid;

use crate::{
    DbClient,
    models::{ manufacturer::Manufacturer, address::{ Address, AddressInput }, asset::Asset },
    AppError,
    Repository,
};

#[derive(Debug, Default)]
pub struct ManufacturerMutation;

#[Object]
impl ManufacturerMutation {
    async fn create_manufacturer(
        &self,
        ctx: &Context<'_>,
        name: String,
        phone: String,
        email: String,
        website: Option<String>,
        notes: Option<String>,
        address: AddressInput,
        support_contact: Option<String>,
        warranty_contact: Option<String>,
        active: Option<bool>
    ) -> Result<Manufacturer, Error> {
        info!("Creating new manufacturer: {}", name);

        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());
        let id = Uuid::new_v4().to_string();

        Address::from(address.clone())
            .validate()
            .map_err(|e| { AppError::ValidationError(e).to_graphql_error() })?;

        let existing_manufacturers = repo
            .list::<Manufacturer>(None).await
            .map_err(|e| e.to_graphql_error())?;
        if existing_manufacturers.iter().any(|m| m.name.to_lowercase() == name.to_lowercase()) {
            return Err(
                AppError::ValidationError(
                    "Manufacturer name already exists".to_string()
                ).to_graphql_error()
            );
        }

        if existing_manufacturers.iter().any(|m| m.email.to_lowercase() == email.to_lowercase()) {
            return Err(
                AppError::ValidationError(
                    "Email address already in use".to_string()
                ).to_graphql_error()
            );
        }

        let manufacturer = Manufacturer::new(
            id,
            name,
            phone,
            email,
            website,
            notes,
            Address::from(address.clone()),
            support_contact,
            warranty_contact,
            active.unwrap_or(true)
        ).map_err(|e| e.to_graphql_error())?;

        repo.create(manufacturer).await.map_err(|e| e.to_graphql_error())
    }

    async fn update_manufacturer(
        &self,
        ctx: &Context<'_>,
        id: String,
        name: Option<String>,
        phone: Option<String>,
        email: Option<String>,
        website: Option<String>,
        notes: Option<String>,
        address: Option<AddressInput>,
        support_contact: Option<String>,
        warranty_contact: Option<String>,
        active: Option<bool>
    ) -> Result<Manufacturer, Error> {
        info!("Updating manufacturer: {}", id);

        let db_client = ctx
            .data::<DbClient>()
            .map_err(|_| {
                AppError::InternalServerError("Database client not available".to_string())
            })?;

        let repo = Repository::new(db_client.clone());

        let mut manufacturer = repo
            .get::<Manufacturer>(id.clone()).await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| AppError::NotFound(format!("Manufacturer {} not found", id)))?;

        if let Some(new_name) = name {
            if new_name.trim().is_empty() {
                return Err(
                    AppError::ValidationError(
                        "Manufacturer name cannot be empty".to_string()
                    ).to_graphql_error()
                );
            }
            let existing_manufacturers = repo
                .list::<Manufacturer>(None).await
                .map_err(|e| e.to_graphql_error())?;
            if
                existing_manufacturers
                    .iter()
                    .any(
                        |m|
                            m.id != manufacturer.id &&
                            m.name.to_lowercase() == new_name.to_lowercase()
                    )
            {
                return Err(
                    AppError::ValidationError(
                        "Manufacturer name already exists".to_string()
                    ).to_graphql_error()
                );
            }
            manufacturer.name = new_name;
        }

        if let Some(new_phone) = phone {
            manufacturer.phone = new_phone;
        }

        if let Some(new_email) = email {
            if new_email.trim().is_empty() {
                return Err(
                    AppError::ValidationError(
                        "Email cannot be empty".to_string()
                    ).to_graphql_error()
                );
            }
            let existing_manufacturers = repo
                .list::<Manufacturer>(None).await
                .map_err(|e| e.to_graphql_error())?;
            if
                existing_manufacturers
                    .iter()
                    .any(
                        |m|
                            m.id != manufacturer.id &&
                            m.email.to_lowercase() == new_email.to_lowercase()
                    )
            {
                return Err(
                    AppError::ValidationError(
                        "Email address already in use".to_string()
                    ).to_graphql_error()
                );
            }
            manufacturer.email = new_email;
        }

        if let Some(new_website) = website {
            manufacturer.website = if new_website.is_empty() { None } else { Some(new_website) };
        }

        if let Some(new_notes) = notes {
            manufacturer.notes = if new_notes.is_empty() { None } else { Some(new_notes) };
        }

        if let Some(new_address) = address {
            Address::from(new_address.clone())
                .validate()
                .map_err(|e| { AppError::ValidationError(e).to_graphql_error() })?;
            manufacturer.address = Address::from(new_address);
        }

        if let Some(new_support) = support_contact {
            manufacturer.support_contact = if new_support.is_empty() {
                None
            } else {
                Some(new_support)
            };
        }

        if let Some(new_warranty) = warranty_contact {
            manufacturer.warranty_contact = if new_warranty.is_empty() {
                None
            } else {
                Some(new_warranty)
            };
        }

        if let Some(is_active) = active {
            manufacturer.active = is_active;
        }

        manufacturer.updated_at = chrono::Utc::now();

        repo.update(manufacturer).await.map_err(|e| e.to_graphql_error())
    }

    async fn activate_manufacturer(
        &self,
        ctx: &Context<'_>,
        id: String
    ) -> Result<Manufacturer, Error> {
        info!("Activating manufacturer: {}", id);

        let db_client = ctx
            .data::<DbClient>()
            .map_err(|_| {
                AppError::InternalServerError("Database client not available".to_string())
            })?;

        let repo = Repository::new(db_client.clone());

        let mut manufacturer = repo
            .get::<Manufacturer>(id.clone()).await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| AppError::NotFound(format!("Manufacturer {} not found", id)))?;

        manufacturer.active = true;
        manufacturer.updated_at = chrono::Utc::now();

        repo.update(manufacturer).await.map_err(|e| e.to_graphql_error())
    }

    async fn deactivate_manufacturer(
        &self,
        ctx: &Context<'_>,
        id: String
    ) -> Result<Manufacturer, Error> {
        info!("Deactivating manufacturer: {}", id);

        let db_client = ctx
            .data::<DbClient>()
            .map_err(|_| {
                AppError::InternalServerError("Database client not available".to_string())
            })?;

        let repo = Repository::new(db_client.clone());

        let mut manufacturer = repo
            .get::<Manufacturer>(id.clone()).await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| AppError::NotFound(format!("Manufacturer {} not found", id)))?;

        manufacturer.active = false;
        manufacturer.updated_at = chrono::Utc::now();

        repo.update(manufacturer).await.map_err(|e| e.to_graphql_error())
    }

    async fn update_manufacturer_contact_info(
        &self,
        ctx: &Context<'_>,
        id: String,
        phone: Option<String>,
        email: Option<String>,
        support_contact: Option<String>,
        warranty_contact: Option<String>
    ) -> Result<Manufacturer, Error> {
        info!("Updating manufacturer contact info: {}", id);

        let db_client = ctx
            .data::<DbClient>()
            .map_err(|_| {
                AppError::InternalServerError("Database client not available".to_string())
            })?;

        let repo = Repository::new(db_client.clone());

        let mut manufacturer = repo
            .get::<Manufacturer>(id.clone()).await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| AppError::NotFound(format!("Manufacturer {} not found", id)))?;

        if let Some(new_phone) = phone {
            manufacturer.phone = new_phone;
        }

        if let Some(new_email) = email {
            if new_email.trim().is_empty() {
                return Err(
                    AppError::ValidationError(
                        "Email cannot be empty".to_string()
                    ).to_graphql_error()
                );
            }
            let existing_manufacturers = repo
                .list::<Manufacturer>(None).await
                .map_err(|e| e.to_graphql_error())?;
            if
                existing_manufacturers
                    .iter()
                    .any(
                        |m|
                            m.id != manufacturer.id &&
                            m.email.to_lowercase() == new_email.to_lowercase()
                    )
            {
                return Err(
                    AppError::ValidationError(
                        "Email address already in use".to_string()
                    ).to_graphql_error()
                );
            }
            manufacturer.email = new_email;
        }

        if let Some(new_support) = support_contact {
            manufacturer.support_contact = if new_support.is_empty() {
                None
            } else {
                Some(new_support)
            };
        }

        if let Some(new_warranty) = warranty_contact {
            manufacturer.warranty_contact = if new_warranty.is_empty() {
                None
            } else {
                Some(new_warranty)
            };
        }

        manufacturer.updated_at = chrono::Utc::now();

        repo.update(manufacturer).await.map_err(|e| e.to_graphql_error())
    }

    async fn clone_manufacturer(
        &self,
        ctx: &Context<'_>,
        source_id: String,
        new_name: String,
        new_email: String
    ) -> Result<Manufacturer, Error> {
        info!("Cloning manufacturer {} with new name: {}", source_id, new_name);

        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        let source_manufacturer = repo
            .get::<Manufacturer>(source_id.clone()).await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(||
                AppError::NotFound(format!("Source manufacturer {} not found", source_id))
            )?;

        let existing_manufacturers = repo
            .list::<Manufacturer>(None).await
            .map_err(|e| e.to_graphql_error())?;
        if existing_manufacturers.iter().any(|m| m.name.to_lowercase() == new_name.to_lowercase()) {
            return Err(
                AppError::ValidationError(
                    "Manufacturer name already exists".to_string()
                ).to_graphql_error()
            );
        }

        if
            existing_manufacturers
                .iter()
                .any(|m| m.email.to_lowercase() == new_email.to_lowercase())
        {
            return Err(
                AppError::ValidationError(
                    "Email address already in use".to_string()
                ).to_graphql_error()
            );
        }

        let new_id = Uuid::new_v4().to_string();

        let cloned_manufacturer = Manufacturer::new(
            new_id,
            new_name,
            source_manufacturer.phone,
            new_email,
            source_manufacturer.website,
            source_manufacturer.notes,
            source_manufacturer.address,
            source_manufacturer.support_contact,
            source_manufacturer.warranty_contact,
            source_manufacturer.active
        ).map_err(|e| e.to_graphql_error())?;

        repo.create(cloned_manufacturer).await.map_err(|e| e.to_graphql_error())
    }

    async fn bulk_update_manufacturers(
        &self,
        ctx: &Context<'_>,
        manufacturer_ids: Vec<String>,
        active: Option<bool>,
        phone: Option<String>,
        support_contact: Option<String>,
        warranty_contact: Option<String>
    ) -> Result<Vec<Manufacturer>, Error> {
        info!("Bulk updating {} manufacturers", manufacturer_ids.len());

        let db_client = ctx
            .data::<DbClient>()
            .map_err(|_| {
                AppError::InternalServerError("Database client not available".to_string())
            })?;

        let repo = Repository::new(db_client.clone());

        let mut results = Vec::new();

        for manufacturer_id in manufacturer_ids {
            let manufacturer_result = repo.get::<Manufacturer>(manufacturer_id.clone()).await;

            if let Ok(Some(mut manufacturer)) = manufacturer_result {
                let mut updated = false;

                if let Some(is_active) = active {
                    manufacturer.active = is_active;
                    updated = true;
                }

                if let Some(ref new_phone) = phone {
                    manufacturer.phone = new_phone.clone();
                    updated = true;
                }

                if let Some(ref new_support) = support_contact {
                    manufacturer.support_contact = if new_support.is_empty() {
                        None
                    } else {
                        Some(new_support.clone())
                    };
                    updated = true;
                }

                if let Some(ref new_warranty) = warranty_contact {
                    manufacturer.warranty_contact = if new_warranty.is_empty() {
                        None
                    } else {
                        Some(new_warranty.clone())
                    };
                    updated = true;
                }

                if updated {
                    manufacturer.updated_at = chrono::Utc::now();
                    if let Ok(updated_manufacturer) = repo.update(manufacturer).await {
                        results.push(updated_manufacturer);
                    }
                }
            }
        }

        Ok(results)
    }

    async fn delete_manufacturer(
        &self,
        ctx: &Context<'_>,
        id: String,
        force: Option<bool>
    ) -> Result<bool, Error> {
        info!("Deleting manufacturer: {}", id);

        let db_client = ctx
            .data::<DbClient>()
            .map_err(|_| {
                AppError::InternalServerError("Database client not available".to_string())
            })?;

        let repo = Repository::new(db_client.clone());

        let _manufacturer = repo
            .get::<Manufacturer>(id.clone()).await
            .map_err(|e| e.to_graphql_error())?
            .ok_or_else(|| AppError::NotFound(format!("Manufacturer {} not found", id)))?;

        let assets = repo.list::<Asset>(None).await.map_err(|e| e.to_graphql_error())?;
        let has_assets = assets.iter().any(|a| a.manufacturer_id == id);

        if has_assets && !force.unwrap_or(false) {
            return Err(
                AppError::ValidationError(
                    "Cannot delete manufacturer with existing assets. Use force=true to override.".to_string()
                ).to_graphql_error()
            );
        }

        repo.delete::<Manufacturer>(id).await.map_err(|e| e.to_graphql_error())
    }
}
