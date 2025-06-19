use std::collections::HashMap;

use async_graphql::{ Object };
use aws_sdk_dynamodb::types::AttributeValue;
use chrono::{ DateTime, Utc };
use regex::Regex;
use serde::{ Deserialize, Serialize };
use tracing::info;

use crate::error::AppError;

/// Represents a street address for a location
///
/// # Fields
///
/// * `street` - building street and number
/// * `unit` - unit specifier for address
/// * `city` - city
/// * `state` - US state
/// * `country` - country
/// * `zip` - zip code

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Address {
    pub street: String,
    pub unit: Option<String>,
    pub city: String,
    pub state: String,
    pub country: String,
    pub zip: String,
}

impl Address {
    fn new(
        street: String,
        unit: Option<String>,
        city: String,
        state: String,
        country: String,
        zip: String
    ) -> Self {
        Self {
            street,
            unit,
            city,
            state,
            country,
            zip,
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        let po_box_regex = Regex::new(r"P([.]?(O|0)[.]?|ost|ostal).((O|0)ffice.)?Box{1}\b/i");
        let street_addr_regex = Regex::new(r"/\d{1,}(\s{1}\w{1,})(\s{1}?\w{1,})+)/g");
        if self.street.trim().is_empty() {
            return Err("Street field cannot be empty".to_string());
        }

        if
            !street_addr_regex.is_match(self.street.trim()) &&
            !po_box_regex.is_match(self.street.trim())
        {
            return Err("Street value invalid".to_string());
        }
        if self.city.trim().is_empty() {
            return Err("City field cannot be empty".to_string());
        }
        if self.state.trim().is_empty() {
            return Err("State field cannot be empty".to_string());
        }
        if self.country.trim().is_empty() {
            return Err("Country field cannot be empty".to_string());
        }
        if self.zip.trim().is_empty() {
            return Err("Zip field cannot be empty".to_string());
        }

        Ok(())
    }

    fn from_item(item: &HashMap<String, AttributeValue>) -> Option<Self> {
        let street = item.get("street")?.as_s().ok()?.to_string();
        let unit = item.get("unit").and_then(|v| {
            match v {
                AttributeValue::S(s) => Some(s.clone()),
                AttributeValue::Null(_) => None,
                _ => None,
            }
        });
        let city = item.get("city")?.as_s().ok()?.to_string();
        let state = item.get("state")?.as_s().ok()?.to_string();
        let country = item.get("country")?.as_s().ok()?.to_string();
        let zip = item.get("zip")?.as_s().ok()?.to_string();

        Some(Self {
            street,
            unit,
            city,
            state,
            country,
            zip,
        })
    }
    fn to_item(&self) -> HashMap<String, AttributeValue> {
        let mut item = HashMap::new();

        item.insert("street".to_string(), AttributeValue::S(self.id.clone()));

        if let Some(unit) = &self.unit {
            item.insert("unit".to_string(), unit.clone());
        }

        item.insert("city".to_string(), AttributeValue::S(self.name.clone()));
        item.insert("state".to_string(), AttributeValue::S(self.name.clone()));
        item.insert("country".to_string(), AttributeValue::S(self.name.clone()));
        item.insert("zip".to_string(), AttributeValue::S(self.name.clone()));
    }
}

/// Represents a Location in the system
///
/// # Fields
///
/// * `id` - Unique identifier for the location
/// * `name` - Name of the location
/// * `type_id` - Type of location (building, floor, room, etc.)
/// * `parent_location_id` - Optional ID of parent location for hierarchical structure
/// * `description` - Optional description of the location
/// * `address` - Optional physical address
/// * `coordinates` - Optional GPS coordinates
/// * `created_at` - Date and time of creation
/// * `updated_at` - Date and time of last update
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Location {
    pub id: String,
    pub name: String,
    pub r#type_id: String,
    pub parent_location_id: Option<String>,
    pub description: Option<String>,
    pub address: Address,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Defines methods for Location
impl Location {
    /// Creates new Location instance
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier
    /// * `name` - Name of the location
    /// * `location_type` - Type of location as string
    /// * `parent_location_id` - Optional parent location ID
    /// * `description` - Optional description
    /// * `address` - Optional physical address
    /// * `coordinates` - Optional GPS coordinates
    ///
    /// # Returns
    ///
    /// New Location instance
    pub fn new(
        id: String,
        name: String,
        r#type_id: String,
        parent_location_id: Option<String>,
        description: Option<String>,
        address: Address,
        coordinates: Option<String>
    ) -> Result<Self, AppError> {
        let now = Utc::now();

        Ok(Self {
            id,
            name,
            r#type_id,
            parent_location_id,
            address,
            description,
            created_at: now,
            updated_at: now,
        })
    }

    /// Creates Location instance from DynamoDB item
    ///
    /// # Arguments
    ///
    /// * `item` - The dynamo db item
    ///
    /// # Returns
    ///
    /// 'Some' Location if item fields match, 'None' otherwise
    pub fn from_item(item: &HashMap<String, AttributeValue>) -> Option<Self> {
        info!("calling from_item with: {:?}", &item);

        let id = item.get("id")?.as_s().ok()?.to_string();
        let name = item.get("name")?.as_s().ok()?.to_string();

        let r#type_id = item.get("r#type_id")?.as_s().ok()?;

        let parent_location_id = item
            .get("parent_location_id")
            .and_then(|v| v.as_s().ok())
            .map(|s| s.to_string());

        let description = item
            .get("description")
            .and_then(|v| v.as_s().ok())
            .map(|s| s.to_string());

        let address = item
            .get("address")
            .and_then(|v| v.as_m().ok())
            .and_then(|address_map| Address::from_item(address_map))?;

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
            r#type_id,
            parent_location_id,
            description,
            address,
            created_at,
            updated_at,
        });

        info!("result of from_item on location: {:?}", res);
        res
    }

    /// Creates DynamoDB item from Location instance
    ///
    /// # Arguments
    ///
    /// * `self` - borrowed instance of self
    ///
    /// # Returns
    ///
    /// HashMap representing DB item for Location instance
    pub fn to_item(&self) -> HashMap<String, AttributeValue> {
        let mut item = HashMap::new();

        item.insert("id".to_string(), AttributeValue::S(self.id.clone()));
        item.insert("name".to_string(), AttributeValue::S(self.name.clone()));
        item.insert("location_type".to_string(), AttributeValue::S(self.r#type_id.clone()));

        if let Some(parent_id) = &self.parent_location_id {
            item.insert("parent_location_id".to_string(), AttributeValue::S(parent_id.clone()));
        }

        if let Some(desc) = &self.description {
            item.insert("description".to_string(), AttributeValue::S(desc.clone()));
        }

        let address_item = self.address.to_item();
        item.insert("address".to_string(), AttributeValue::M(address_item));

        item.insert("created_at".to_string(), AttributeValue::S(self.created_at.to_string()));
        item.insert("updated_at".to_string(), AttributeValue::S(self.updated_at.to_string()));

        item
    }
}

#[Object]
impl Location {
    async fn id(&self) -> &str {
        &self.id
    }

    async fn name(&self) -> &str {
        &self.name
    }

    async fn location_type(&self) -> &str {
        self.r#type_id.as_str()
    }

    async fn address(&self) -> &Address {
        &self.address
    }

    async fn parent_location_id(&self) -> Option<&str> {
        self.parent_location_id.as_deref()
    }

    async fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    async fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }

    async fn updated_at(&self) -> &DateTime<Utc> {
        &self.updated_at
    }
}

#[Object]
impl Address {
    async fn street(&self) -> &str {
        &self.street
    }

    async fn unit(&self) -> &str {
        self.unit.as_deref().unwrap_or(" ")
    }
    async fn city(&self) -> &str {
        &self.city
    }

    async fn state(&self) -> &str {
        &self.state
    }
    async fn country(&self) -> &str {
        &self.country
    }

    async fn zip(&self) -> &str {
        &self.zip
    }
}
