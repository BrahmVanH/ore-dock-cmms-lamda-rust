use std::collections::HashMap;

use aws_sdk_dynamodb::types::AttributeValue;
use chrono::{ DateTime, Utc };
use serde::{ Deserialize, Serialize };

use crate::DynamoDbEntity;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Location {
    pub id: String,
    pub name: String,
    pub description: String,
    pub location_type_id: String,
    pub parent_location_id: Option<String>,
    pub address: Option<String>,
    pub coordinates: Option<String>, // Could be lat,lng or more complex
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Location {
    pub fn new(
        id: String,
        name: String,
        description: String,
        location_type_id: String,
        parent_location_id: Option<String>,
        address: Option<String>,
        coordinates: Option<String>
    ) -> Self {
        let now = Utc::now();

        Self {
            id,
            name,
            description,
            location_type_id,
            parent_location_id,
            address,
            coordinates,
            is_active: true,
            created_at: now,
            updated_at: now,
        }
    }

    pub(crate) fn validate(&self) -> Result<(), String> {
        if self.name.trim().is_empty() || self.description.trim().is_empty() {
            return Err("Name and Description cannot be empty".to_string());
        }

        if self.location_type_id.trim().is_empty() {
            return Err("Location type ID cannot be empty".to_string());
        }

        // Validate coordinates format if provided (basic validation)
        if let Some(ref coords) = self.coordinates {
            if !coords.trim().is_empty() && !coords.contains(',') {
                return Err("Coordinates must be in format 'latitude,longitude'".to_string());
            }
        }

        Ok(())
    }
}

impl DynamoDbEntity for Location {
    fn table_name() -> &'static str {
        "Locations"
    }

    fn primary_key(&self) -> String {
        self.id.clone()
    }

    fn from_item(item: &HashMap<String, AttributeValue>) -> Option<Self> {
        let id = item.get("id")?.as_s().ok()?.to_string();
        let name = item.get("name")?.as_s().ok()?.to_string();
        let description = item.get("description")?.as_s().ok()?.to_string();
        let location_type_id = item.get("location_type_id")?.as_s().ok()?.to_string();

        let parent_location_id = item
            .get("parent_location_id")
            .and_then(|v| v.as_s().ok())
            .map(|s| s.to_string());

        let address = item
            .get("address")
            .and_then(|v| v.as_s().ok())
            .map(|s| s.to_string());

        let coordinates = item
            .get("coordinates")
            .and_then(|v| v.as_s().ok())
            .map(|s| s.to_string());

        let is_active = item
            .get("is_active")
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

        Some(Self {
            id,
            name,
            description,
            location_type_id,
            parent_location_id,
            address,
            coordinates,
            is_active,
            created_at,
            updated_at,
        })
    }

    fn to_item(&self) -> HashMap<String, AttributeValue> {
        let mut item = HashMap::new();

        item.insert("id".to_string(), AttributeValue::S(self.id.clone()));
        item.insert("name".to_string(), AttributeValue::S(self.name.clone()));
        item.insert("description".to_string(), AttributeValue::S(self.description.clone()));
        item.insert(
            "location_type_id".to_string(),
            AttributeValue::S(self.location_type_id.clone())
        );
        item.insert("is_active".to_string(), AttributeValue::Bool(self.is_active));
        item.insert("created_at".to_string(), AttributeValue::S(self.created_at.to_string()));
        item.insert("updated_at".to_string(), AttributeValue::S(self.updated_at.to_string()));

        if let Some(ref parent_id) = self.parent_location_id {
            item.insert("parent_location_id".to_string(), AttributeValue::S(parent_id.clone()));
        }

        if let Some(ref addr) = self.address {
            item.insert("address".to_string(), AttributeValue::S(addr.clone()));
        }

        if let Some(ref coords) = self.coordinates {
            item.insert("coordinates".to_string(), AttributeValue::S(coords.clone()));
        }

        item
    }
}
