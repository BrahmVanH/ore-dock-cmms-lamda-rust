use std::collections::HashMap;

use async_graphql::InputObject;
use aws_sdk_dynamodb::types::AttributeValue;
use regex::Regex;
use serde::{ Deserialize, Serialize };

use crate::AppError;

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

#[derive(Clone, Debug, Serialize, Deserialize, InputObject)]
pub struct AddressInput {
    pub street: String,
    pub unit: Option<String>,
    pub city: String,
    pub state: String,
    pub country: String,
    pub zip: String,
}

impl From<AddressInput> for Address {
    fn from(input: AddressInput) -> Self {
        Self {
            street: input.street,
            unit: input.unit,
            city: input.city,
            state: input.state,
            country: input.country,
            zip: input.zip,
        }
    }
}

impl From<Address> for AddressInput {
    fn from(address: Address) -> Self {
        Self {
            street: address.street,
            unit: address.unit,
            city: address.city,
            state: address.state,
            country: address.country,
            zip: address.zip,
        }
    }
}

impl Address {
    pub fn new(
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

    pub(crate) fn validate(&self) -> Result<(), String> {
        let po_box_regex = Regex::new(
            r"(?i)^P\.?O\.?\s*Box\s+\d+|Post\s*Office\s*Box\s+\d+|Postal\s*Box\s+\d+"
        ).map_err(|e| {
            return AppError::InternalServerError(e.to_string()).to_string();
        })?;

        let street_addr_regex = Regex::new(r"^\d+\s+\w+.*").map_err(|e| {
            return AppError::InternalServerError(e.to_string()).to_string();
        })?;

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

    pub(crate) fn from_item(av: &AttributeValue) -> Option<Self> {
        if let AttributeValue::M(item) = av {
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
        } else {
            None
        }
    }
    pub(crate) fn to_item(&self) -> AttributeValue {
        let mut item = HashMap::new();

        item.insert("street".to_string(), AttributeValue::S(self.street.clone()));

        if let Some(unit) = &self.unit {
            item.insert("unit".to_string(), AttributeValue::S(unit.clone()));
        }

        item.insert("city".to_string(), AttributeValue::S(self.city.clone()));
        item.insert("state".to_string(), AttributeValue::S(self.state.clone()));
        item.insert("country".to_string(), AttributeValue::S(self.country.clone()));
        item.insert("zip".to_string(), AttributeValue::S(self.zip.clone()));
        AttributeValue::M(item)
    }
}

/// Tests

#[cfg(test)]
mod tests {
    use super::*;

    fn create_valid_address() -> Address {
        Address::new(
            "123 Main St".to_string(),
            Some("Apt 4B".to_string()),
            "Springfield".to_string(),
            "IL".to_string(),
            "United States".to_string(),
            "62701".to_string()
        )
    }

    #[test]
    fn test_new_address_with_unit() {
        let address = Address::new(
            "123 Main St".to_string(),
            Some("Apt 4B".to_string()),
            "Springfield".to_string(),
            "IL".to_string(),
            "United States".to_string(),
            "62701".to_string()
        );

        assert_eq!(address.street, "123 Main St");
        assert_eq!(address.unit, Some("Apt 4B".to_string()));
        assert_eq!(address.city, "Springfield");
        assert_eq!(address.state, "IL");
        assert_eq!(address.country, "United States");
        assert_eq!(address.zip, "62701");
    }

    #[test]
    fn test_new_address_without_unit() {
        let address = Address::new(
            "456 Oak Avenue".to_string(),
            None,
            "Chicago".to_string(),
            "IL".to_string(),
            "United States".to_string(),
            "60601".to_string()
        );

        assert_eq!(address.street, "456 Oak Avenue");
        assert_eq!(address.unit, None);
        assert_eq!(address.city, "Chicago");
    }

    #[test]
    fn test_validate_empty_street() {
        let mut address = create_valid_address();
        address.street = "".to_string();

        let result = address.validate();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Street field cannot be empty");
    }

    #[test]
    fn test_validate_whitespace_only_street() {
        let mut address = create_valid_address();
        address.street = "   ".to_string();

        let result = address.validate();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Street field cannot be empty");
    }

    #[test]
    fn test_validate_empty_city() {
        let mut address = create_valid_address();
        address.city = "".to_string();

        let result = address.validate();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "City field cannot be empty");
    }

    #[test]
    fn test_validate_empty_state() {
        let mut address = create_valid_address();
        address.state = "".to_string();

        let result = address.validate();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "State field cannot be empty");
    }

    #[test]
    fn test_validate_empty_country() {
        let mut address = create_valid_address();
        address.country = "".to_string();

        let result = address.validate();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Country field cannot be empty");
    }

    #[test]
    fn test_validate_empty_zip() {
        let mut address = create_valid_address();
        address.zip = "".to_string();

        let result = address.validate();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Zip field cannot be empty");
    }

    #[test]
    fn test_clone() {
        let address = create_valid_address();
        let cloned = address.clone();

        assert_eq!(address.street, cloned.street);
        assert_eq!(address.unit, cloned.unit);
        assert_eq!(address.city, cloned.city);
    }
}
