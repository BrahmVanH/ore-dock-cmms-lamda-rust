use crate::models::{ prelude::*, manufacturer::Manufacturer };
#[Object]
impl Manufacturer {
    async fn id(&self) -> &str {
        &self.id
    }

    async fn name(&self) -> &str {
        &self.name
    }

    async fn phone(&self) -> &str {
        &self.phone
    }

    async fn email(&self) -> &str {
        &self.email
    }

    async fn website(&self) -> Option<&str> {
        self.website.as_deref()
    }

    async fn notes(&self) -> Option<&str> {
        self.notes.as_deref()
    }

    async fn address(&self) -> &Address {
        &self.address
    }

    async fn support_contact(&self) -> Option<&str> {
        self.support_contact.as_deref()
    }

    async fn warranty_contact(&self) -> Option<&str> {
        self.warranty_contact.as_deref()
    }

    async fn active(&self) -> bool {
        self.active
    }

    async fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }

    async fn updated_at(&self) -> &DateTime<Utc> {
        &self.updated_at
    }
}
