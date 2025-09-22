use std::collections::HashMap;

use async_graphql::Enum;
use aws_sdk_dynamodb::types::AttributeValue;
use chrono::{ DateTime, Utc };
use serde::{ Deserialize, Serialize };

use crate::{ repository::DynamoDbEntity, AppError };

#[derive(Enum, Copy, Clone, PartialEq, Eq, Debug, Deserialize, Serialize, Hash)]
#[serde(rename_all = "snake_case")]
pub enum AssetTypeCategory {
    FoodProduction,
    BeverageProduction,
    Bar,
    Refrigeration,
    Storage,
    Office,
    Garage,
    Entertainment,
    Infrastructure,
    Utilities,
    Lab,
    Miscellaneous,
}

impl AssetTypeCategory {
    pub(crate) fn to_string(&self) -> String {
        self.to_str().to_string()
    }
    pub(crate) fn to_str(&self) -> &str {
        match self {
            &AssetTypeCategory::FoodProduction => "food-production",
            &AssetTypeCategory::BeverageProduction => "beverage-production",
            &AssetTypeCategory::Bar => "bar",
            &AssetTypeCategory::Refrigeration => "refrigeration",
            &AssetTypeCategory::Storage => "storage",
            &AssetTypeCategory::Office => "office",
            &AssetTypeCategory::Garage => "garage",
            &AssetTypeCategory::Entertainment => "entertainment",
            &AssetTypeCategory::Infrastructure => "infrastructure",
            &AssetTypeCategory::Utilities => "utilities",
            &AssetTypeCategory::Lab => "lab",
            &AssetTypeCategory::Miscellaneous => "miscellaneous",
        }
    }
    pub(crate) fn from_string(s: &str) -> Result<AssetTypeCategory, AppError> {
        match s {
            "food-production" => Ok(Self::FoodProduction),
            "beverage-production" => Ok(Self::BeverageProduction),
            "bar" => Ok(Self::Bar),
            "refrigeration" => Ok(Self::Refrigeration),
            "storage" => Ok(Self::Storage),
            "office" => Ok(Self::Office),
            "garage" => Ok(Self::Garage),
            "entertainment" => Ok(Self::Entertainment),
            "infrastructure" => Ok(Self::Infrastructure),
            "utilities" => Ok(Self::Utilities),
            "lab" => Ok(Self::Lab),
            "miscellaneous" => Ok(Self::Miscellaneous),
            _ => {
                return Err(
                    AppError::DatabaseError(
                        "Invalid category string string for asset type".to_string()
                    )
                );
            }
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AssetType {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: AssetTypeCategory,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl AssetType {
    pub fn new(
        id: String,
        name: String,
        description: String,
        category: String
    ) -> Result<Self, AppError> {
        let now = Utc::now();

        let cat = AssetTypeCategory::from_string(&category.to_lowercase())?;

        Ok(Self {
            id,
            name,
            description,
            category: cat,
            created_at: now,
            updated_at: now,
        })
    }

    pub(crate) fn validate(&self) -> Result<(), String> {
        if self.name.trim().is_empty() || self.description.trim().is_empty() {
            return Err("Name and Description cannot be empty".to_string());
        }

        Ok(())
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

        let category_str = item.get("category")?.as_s().ok()?;
        let category = AssetTypeCategory::from_string(&category_str)
            .map_err(|e| e)
            .ok()?;

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
            category,
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
        item.insert("category".to_string(), AttributeValue::S(self.category.to_string()));
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
            "High-pressure industrial water pump".to_string(),
            "garage".to_string()
        ).unwrap()
    }

    #[test]
    fn test_new_asset_type() {
        let asset_type = AssetType::new(
            "type-456".to_string(),
            "HVAC System".to_string(),
            "Commercial heating and cooling system".to_string(),
            "garage".to_string()
        ).unwrap();

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

    #[test]
    fn test_asset_type_category_from_string_invalid() {
        let result = AssetTypeCategory::from_string("invalid-category");
        assert!(result.is_err());
        let err = result.unwrap_err();
        match err {
            AppError::DatabaseError(msg) => assert!(msg.contains("Invalid category string")),
            _ => panic!("Unexpected error type"),
        }
    }

    #[test]
    fn test_asset_type_category_round_trip() {
        for cat in [
            AssetTypeCategory::FoodProduction,
            AssetTypeCategory::BeverageProduction,
            AssetTypeCategory::Bar,
            AssetTypeCategory::Refrigeration,
            AssetTypeCategory::Storage,
            AssetTypeCategory::Office,
            AssetTypeCategory::Garage,
            AssetTypeCategory::Entertainment,
            AssetTypeCategory::Infrastructure,
            AssetTypeCategory::Utilities,
            AssetTypeCategory::Lab,
            AssetTypeCategory::Miscellaneous,
        ] {
            let s = cat.to_string();
            let parsed = AssetTypeCategory::from_string(&s).unwrap();
            assert_eq!(cat, parsed);
        }
    }

    #[test]
    fn test_dynamodb_entity_table_name() {
        assert_eq!(AssetType::table_name(), "AssetTypes");
    }

    #[test]
    fn test_dynamodb_entity_primary_key() {
        let asset_type = create_valid_asset_type();
        assert_eq!(asset_type.primary_key(), asset_type.id);
    }

    #[test]
    fn test_dynamodb_entity_to_item_and_from_item() {
        let asset_type = create_valid_asset_type();
        let item = asset_type.to_item();
        let parsed = AssetType::from_item(&item).unwrap();
        assert_eq!(asset_type.id, parsed.id);
        assert_eq!(asset_type.name, parsed.name);
        assert_eq!(asset_type.description, parsed.description);
        assert_eq!(asset_type.category, parsed.category);
        // Allow some leeway for timestamps, but they should be equal for a round-trip
        assert_eq!(asset_type.created_at, parsed.created_at);
        assert_eq!(asset_type.updated_at, parsed.updated_at);
    }
}
