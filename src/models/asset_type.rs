use std::collections::HashMap;

use aws_sdk_dynamodb::types::AttributeValue;
use chrono::{ DateTime, Utc };
use serde::{ Deserialize, Serialize };

use crate::repository::DynamoDbEntity;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AssetType {
    pub id: String,
    pub name: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl AssetType {
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
            description,
            created_at,
            updated_at,
        });

        res
    }
}

impl DynamoDbEntity for AssetType {
    fn table_name() -> &'static str {
        "AssetTypes"
    }

    fn primary_key(&self) -> String {
        self.id.clone()
    }

    fn from_item(item: &HashMap<String, AttributeValue>) -> Option<Self> {
        let id = item.get("id")?.as_s().ok()?.to_string();
        let name = item.get("name")?.as_s().ok()?.to_string();
        let description = item.get("description")?.as_s().ok()?.to_string();

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
            description,
            created_at,
            updated_at,
        });

        res
    }

    fn to_item(&self) -> HashMap<String, AttributeValue> {
        let mut item = HashMap::new();

        item.insert("id".to_string(), AttributeValue::S(self.id.clone()));
        item.insert("name".to_string(), AttributeValue::S(self.name.clone()));
        item.insert("description".to_string(), AttributeValue::S(self.description.clone()));
        item.insert("created_at".to_string(), AttributeValue::S(self.created_at.to_string()));
        item.insert("updated_at".to_string(), AttributeValue::S(self.updated_at.to_string()));

        item
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    fn create_valid_asset_type() -> AssetType {
        AssetType::new(
            "type-123".to_string(),
            "Industrial Pump".to_string(),
            "High-pressure industrial water pump".to_string()
        )
    }

    #[test]
    fn test_new_asset_type() {
        let asset_type = AssetType::new(
            "type-456".to_string(),
            "HVAC System".to_string(),
            "Commercial heating and cooling system".to_string()
        );

        assert_eq!(asset_type.id, "type-456");
        assert_eq!(asset_type.name, "HVAC System");
        assert_eq!(asset_type.description, "Commercial heating and cooling system");
        assert!(asset_type.created_at <= Utc::now());
        assert!(asset_type.updated_at <= Utc::now());
        assert_eq!(asset_type.created_at, asset_type.updated_at);
    }

    #[test]
    fn test_new_asset_type_timestamps_are_current() {
        let before = Utc::now();
        let asset_type = create_valid_asset_type();
        let after = Utc::now();

        assert!(asset_type.created_at >= before);
        assert!(asset_type.created_at <= after);
        assert!(asset_type.updated_at >= before);
        assert!(asset_type.updated_at <= after);
    }

    #[test]
    fn test_validate_valid_asset_type() {
        let asset_type = create_valid_asset_type();
        let result = asset_type.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_empty_name() {
        let mut asset_type = create_valid_asset_type();
        asset_type.name = "".to_string();

        let result = asset_type.validate();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Name and Description cannot be empty");
    }

    #[test]
    fn test_validate_whitespace_only_name() {
        let mut asset_type = create_valid_asset_type();
        asset_type.name = "   ".to_string();

        let result = asset_type.validate();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Name and Description cannot be empty");
    }

    #[test]
    fn test_validate_empty_description() {
        let mut asset_type = create_valid_asset_type();
        asset_type.description = "".to_string();

        let result = asset_type.validate();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Name and Description cannot be empty");
    }

    #[test]
    fn test_validate_whitespace_only_description() {
        let mut asset_type = create_valid_asset_type();
        asset_type.description = "   ".to_string();

        let result = asset_type.validate();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Name and Description cannot be empty");
    }

    #[test]
    fn test_validate_both_empty() {
        let mut asset_type = create_valid_asset_type();
        asset_type.name = "".to_string();
        asset_type.description = "".to_string();

        let result = asset_type.validate();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Name and Description cannot be empty");
    }

    #[test]
    fn test_clone() {
        let asset_type = create_valid_asset_type();
        let cloned = asset_type.clone();

        assert_eq!(asset_type.id, cloned.id);
        assert_eq!(asset_type.name, cloned.name);
        assert_eq!(asset_type.description, cloned.description);
        assert_eq!(asset_type.created_at, cloned.created_at);
        assert_eq!(asset_type.updated_at, cloned.updated_at);
    }

    #[test]
    fn test_debug_format() {
        let asset_type = create_valid_asset_type();
        let debug_str = format!("{:?}", asset_type);

        assert!(debug_str.contains("type-123"));
        assert!(debug_str.contains("Industrial Pump"));
        assert!(debug_str.contains("High-pressure industrial water pump"));
    }
}
