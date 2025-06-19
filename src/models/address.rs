use std::collections::HashMap;

use async_graphql::Object;
use aws_sdk_dynamodb::types::AttributeValue;
use regex::Regex;
use serde::{ Deserialize, Serialize };

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

    pub fn from_item(item: &HashMap<String, AttributeValue>) -> Option<Self> {
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
    pub fn to_item(&self) -> HashMap<String, AttributeValue> {
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
