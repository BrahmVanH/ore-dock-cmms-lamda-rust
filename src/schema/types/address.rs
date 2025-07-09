use crate::models::prelude::*;
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

