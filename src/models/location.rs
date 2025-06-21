use std::collections::HashMap;

use aws_sdk_dynamodb::types::AttributeValue;
use chrono::{ DateTime, Utc };
use serde::{ Deserialize, Serialize };
use tracing::info;

use crate::{ error::AppError, models::address::Address };

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
    pub(crate)  fn from_item(item: &HashMap<String, AttributeValue>) -> Option<Self> {
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
    pub(crate)  fn to_item(&self) -> HashMap<String, AttributeValue> {
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
