use crate::models::prelude::*;

#[Object]
impl Location {
    async fn id(&self) -> &str {
        &self.id
    }

    async fn name(&self) -> &str {
        &self.name
    }

    async fn location_type(&self) -> &str {
        self.location_type_id.as_str()
    }

    async fn address(&self) -> &Address {
        &self.address
    }

    async fn parent_location_id(&self) -> Option<&str> {
        self.parent_location_id.as_deref()
    }

    async fn description(&self) -> &str {
        &self.description
    }

    async fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }

    async fn updated_at(&self) -> &DateTime<Utc> {
        &self.updated_at
    }
}
