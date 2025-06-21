use std::collections::HashMap;

use aws_sdk_dynamodb::types::AttributeValue;
use chrono::{ DateTime, Utc };
use serde::{ Deserialize, Serialize };
use tracing::info;

use crate::{ error::AppError, models::address::Address };

/// Represents a Manufacturer in the system
///
/// # Fields
///
/// * `id` - Unique identifier for the manufacturer
/// * `name` - Name of the manufacturer
/// * `phone` - Phone number for the manufacturer
/// * `email` - Email address for the manufacturer
/// * `website` - Website URL for the manufacturer
/// * `notes` - Optional notes about the manufacturer
/// * `address` - Physical address of the manufacturer
/// * `support_contact` - Optional support contact information
/// * `warranty_contact` - Optional warranty contact information
/// * `active` - Whether the manufacturer is currently active
/// * `created_at` - Date and time of creation
/// * `updated_at` - Date and time of last update
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Manufacturer {
    pub id: String,
    pub name: String,
    pub phone: String,
    pub email: String,
    pub website: Option<String>,
    pub notes: Option<String>,
    pub address: Address,
    pub support_contact: Option<String>,
    pub warranty_contact: Option<String>,
    pub active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Defines methods for Manufacturer
impl Manufacturer {
    /// Creates new Manufacturer instance
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier
    /// * `name` - Name of the manufacturer
    /// * `phone` - Phone number
    /// * `email` - Email address
    /// * `website` - Optional website URL
    /// * `notes` - Optional notes
    /// * `address` - Physical address
    /// * `support_contact` - Optional support contact
    /// * `warranty_contact` - Optional warranty contact
    /// * `active` - Whether manufacturer is active
    ///
    /// # Returns
    ///
    /// New Manufacturer instance
    pub fn new(
        id: String,
        name: String,
        phone: String,
        email: String,
        website: Option<String>,
        notes: Option<String>,
        address: Address,
        support_contact: Option<String>,
        warranty_contact: Option<String>,
        active: bool
    ) -> Result<Self, AppError> {
        let now = Utc::now();

        if name.trim().is_empty() {
            return Err(AppError::ValidationError("Manufacturer name cannot be empty".to_string()));
        }

        if email.trim().is_empty() {
            return Err(AppError::ValidationError("Email cannot be empty".to_string()));
        }

        Ok(Self {
            id,
            name,
            phone,
            email,
            website,
            notes,
            address,
            support_contact,
            warranty_contact,
            active,
            created_at: now,
            updated_at: now,
        })
    }

    /// Creates Manufacturer instance from DynamoDB item
    ///
    /// # Arguments
    ///
    /// * `item` - The dynamo db item
    ///
    /// # Returns
    ///
    /// 'Some' Manufacturer if item fields match, 'None' otherwise
    pub(crate)  fn from_item(item: &HashMap<String, AttributeValue>) -> Option<Self> {
        info!("calling from_item with: {:?}", &item);

        let id = item.get("id")?.as_s().ok()?.to_string();
        let name = item.get("name")?.as_s().ok()?.to_string();
        let phone = item.get("phone")?.as_s().ok()?.to_string();
        let email = item.get("email")?.as_s().ok()?.to_string();

        let website = item
            .get("website")
            .and_then(|v| v.as_s().ok())
            .map(|s| s.to_string());

        let notes = item
            .get("notes")
            .and_then(|v| v.as_s().ok())
            .map(|s| s.to_string());

        let address = item
            .get("address")
            .and_then(|v| v.as_m().ok())
            .and_then(|address_map| Address::from_item(address_map))?;

        let support_contact = item
            .get("support_contact")
            .and_then(|v| v.as_s().ok())
            .map(|s| s.to_string());

        let warranty_contact = item
            .get("warranty_contact")
            .and_then(|v| v.as_s().ok())
            .map(|s| s.to_string());

        let active = item
            .get("active")
            .and_then(|v| v.as_bool().ok())
            .unwrap_or(&true);

        let created_at = item
            .get("created_at")
            .and_then(|v| v.as_s().ok())
            .and_then(|s| s.parse::<DateTime<Utc>>().ok())
            .unwrap_or_else(|| Utc::now());

        let updated_at = item
            .get("updated_at")
            .and_then(|v| v.as_s().ok())
            .and_then(|s| s.parse::<DateTime<Utc>>().ok())
            .unwrap_or_else(|| Utc::now());

        let res = Some(Self {
            id,
            name,
            phone,
            email,
            website,
            notes,
            address,
            support_contact,
            warranty_contact,
            active,
            created_at,
            updated_at,
        });

        info!("result of from_item on manufacturer: {:?}", res);
        res
    }

    /// Creates DynamoDB item from Manufacturer instance
    ///
    /// # Arguments
    ///
    /// * `self` - borrowed instance of self
    ///
    /// # Returns
    ///
    /// HashMap representing DB item for Manufacturer instance
    pub(crate)  fn to_item(&self) -> HashMap<String, AttributeValue> {
        let mut item = HashMap::new();

        item.insert("id".to_string(), AttributeValue::S(self.id.clone()));
        item.insert("name".to_string(), AttributeValue::S(self.name.clone()));
        item.insert("phone".to_string(), AttributeValue::S(self.phone.clone()));
        item.insert("email".to_string(), AttributeValue::S(self.email.clone()));

        if let Some(website) = &self.website {
            item.insert("website".to_string(), AttributeValue::S(website.clone()));
        }

        if let Some(notes) = &self.notes {
            item.insert("notes".to_string(), AttributeValue::S(notes.clone()));
        }

        // Add the nested address as a Map
        let address_item = self.address.to_item();
        item.insert("address".to_string(), AttributeValue::M(address_item));

        if let Some(support) = &self.support_contact {
            item.insert("support_contact".to_string(), AttributeValue::S(support.clone()));
        }

        if let Some(warranty) = &self.warranty_contact {
            item.insert("warranty_contact".to_string(), AttributeValue::S(warranty.clone()));
        }

        item.insert("active".to_string(), AttributeValue::Bool(self.active));
        item.insert("created_at".to_string(), AttributeValue::S(self.created_at.to_string()));
        item.insert("updated_at".to_string(), AttributeValue::S(self.updated_at.to_string()));

        item
    }
}
