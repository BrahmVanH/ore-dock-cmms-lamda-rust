use chrono::{ DateTime, Utc };
use serde::{ Deserialize, Serialize };

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

    pub fn validate(&self) -> Result<(), String> {
        if self.name.trim().is_empty() || self.description.trim().is_empty() {
            return Err("Name and Description cannot be empty".to_string());
        }

        Ok(())
    }
}
