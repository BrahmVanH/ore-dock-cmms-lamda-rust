use std::collections::HashMap;

use aws_sdk_dynamodb::types::AttributeValue;
use chrono::{ DateTime, Utc };
use serde::{ Deserialize, Serialize };

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LocationType {
    pub id: String,
    pub name: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl LocationType {
    pub fn new(id: String, name: String, description: String) -> Self {
        let now = Utc::now();

        Self {
            id,
            name,
            description,
            created_at: now,
            updated_at: now,
        }
    }

    pub(crate) fn validate(&self) -> Result<(), String> {
        if self.name.trim().is_empty() || self.description.trim().is_empty() {
            return Err("Name and Description cannot be empty".to_string());
        }

        Ok(())
    }

    pub(crate) fn from_item(item: &HashMap<String, AttributeValue>) -> Option<Self> {
        let id = item.get("id")?.as_s().ok()?.to_string();
        let name = item.get("name")?.as_s().ok()?.to_string();
        let description = item.get("description")?.as_s().ok()?.to_string();

        let created_at: DateTime<Utc> = item
            .get("created_at")
            .and_then(|v| v.as_s().ok())
            .and_then(|s| s.parse::<DateTime<Utc>>().ok())
            .unwrap_or_else(|| Utc::now());

        let updated_at: DateTime<Utc> = item
            .get("updated_at")
            .and_then(|v| v.as_s().ok())
            .and_then(|s| s.parse::<DateTime<Utc>>().ok())
            .unwrap_or_else(|| Utc::now());

        let res = Some(Self {
            id,
            name,
            description,
            created_at,
            updated_at,
        });

        res
    }

    pub(crate) fn to_item(&self) -> HashMap<String, AttributeValue> {
        let mut item = HashMap::new();

        item.insert("id".to_string(), AttributeValue::S(self.id.clone()));
        item.insert("name".to_string(), AttributeValue::S(self.name.clone()));
        item.insert("description".to_string(), AttributeValue::S(self.description.clone()));
        item.insert("created_at".to_string(), AttributeValue::S(self.created_at.to_string()));
        item.insert("updated_at".to_string(), AttributeValue::S(self.updated_at.to_string()));

        item
    }
}
