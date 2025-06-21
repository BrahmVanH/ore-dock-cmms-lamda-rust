use crate::models::{ prelude::*, vendor::VendorTier };

#[Object]
impl Vendor {
    async fn id(&self) -> &str {
        &self.id
    }

    async fn name(&self) -> &str {
        &self.name
    }

    async fn legal_name(&self) -> Option<&str> {
        self.legal_name.as_deref()
    }

    async fn vendor_code(&self) -> Option<&str> {
        self.vendor_code.as_deref()
    }

    async fn vendor_category_id(&self) -> &str {
        &self.vendor_category_id
    }

    async fn status(&self) -> &str {
        self.status.to_str()
    }

    async fn tier(&self) -> &str {
        self.tier.to_str()
    }

    async fn phone_number(&self) -> &str {
        &self.phone_number
    }

    async fn secondary_phone(&self) -> Option<&str> {
        self.secondary_phone.as_deref()
    }

    async fn email_address(&self) -> &str {
        &self.email_address
    }

    async fn secondary_email(&self) -> Option<&str> {
        self.secondary_email.as_deref()
    }

    async fn website(&self) -> Option<&str> {
        self.website.as_deref()
    }

    async fn tax_id(&self) -> &str {
        &self.tax_id
    }

    async fn registration_number(&self) -> Option<&str> {
        self.registration_number.as_deref()
    }

    async fn payment_terms(&self) -> &str {
        &self.payment_terms
    }

    async fn currency(&self) -> &str {
        &self.currency
    }

    async fn credit_limit(&self) -> Option<f64> {
        self.credit_limit
    }

    async fn primary_contact_name(&self) -> &str {
        &self.primary_contact_name
    }

    async fn primary_contact_title(&self) -> &str {
        &self.primary_contact_title
    }

    async fn primary_contact_email(&self) -> Option<&str> {
        self.primary_contact_email.as_deref()
    }

    async fn primary_contact_phone(&self) -> Option<&str> {
        self.primary_contact_phone.as_deref()
    }

    async fn billing_address(&self) -> Option<String> {
        self.billing_address.as_ref().and_then(|addr| serde_json::to_string(addr).ok())
    }

    async fn shipping_address(&self) -> Option<String> {
        self.shipping_address.as_ref().and_then(|addr| serde_json::to_string(addr).ok())
    }

    async fn bank_details(&self) -> Option<String> {
        self.bank_details.as_ref().and_then(|bank| serde_json::to_string(bank).ok())
    }

    async fn certifications(&self) -> &Vec<String> {
        &self.certifications
    }

    async fn compliance_status(&self) -> &str {
        &self.compliance_status
    }

    async fn insurance_info(&self) -> Option<String> {
        self.insurance_info.as_ref().and_then(|ins| serde_json::to_string(ins).ok())
    }

    async fn contract_start_date(&self) -> Option<&DateTime<Utc>> {
        self.contract_start_date.as_ref()
    }

    async fn contract_end_date(&self) -> Option<&DateTime<Utc>> {
        self.contract_end_date.as_ref()
    }

    async fn last_order_date(&self) -> Option<&DateTime<Utc>> {
        self.last_order_date.as_ref()
    }

    async fn total_orders(&self) -> i32 {
        self.total_orders
    }

    async fn total_spent(&self) -> f64 {
        self.total_spent
    }

    async fn average_rating(&self) -> Option<f64> {
        self.average_rating
    }

    async fn performance_notes(&self) -> Option<&str> {
        self.performance_notes.as_deref()
    }

    async fn emergency_contact(&self) -> Option<String> {
        self.emergency_contact.as_ref().and_then(|ec| serde_json::to_string(ec).ok())
    }

    async fn preferred_communication(&self) -> &str {
        &self.preferred_communication
    }

    async fn time_zone(&self) -> Option<&str> {
        self.time_zone.as_deref()
    }

    async fn business_hours(&self) -> Option<String> {
        self.business_hours.as_ref().and_then(|bh| serde_json::to_string(bh).ok())
    }

    async fn tags(&self) -> &Vec<String> {
        &self.tags
    }

    async fn custom_fields(&self) -> Option<String> {
        self.custom_fields.as_ref().and_then(|cf| serde_json::to_string(cf).ok())
    }

    async fn attachments(&self) -> &Vec<String> {
        &self.attachments
    }

    async fn approval_required(&self) -> bool {
        self.approval_required
    }

    async fn auto_approval_limit(&self) -> Option<f64> {
        self.auto_approval_limit
    }

    async fn notes(&self) -> Option<&str> {
        self.notes.as_deref()
    }

    async fn created_by(&self) -> Option<&str> {
        self.created_by.as_deref()
    }

    async fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }

    async fn updated_at(&self) -> &DateTime<Utc> {
        &self.updated_at
    }

    #[graphql(name = "is_active")]
    async fn check_is_active(&self) -> bool {
        self.is_active()
    }

    #[graphql(name = "is_contract_expired")]
    async fn check_is_contract_expired(&self) -> bool {
        self.is_contract_expired()
    }

    #[graphql(name = "can_place_orders")]
    async fn check_can_place_orders(&self) -> bool {
        self.can_place_orders()
    }

    #[graphql(name = "average_order_value")]
    async fn check_average_order_value(&self) -> f64 {
        self.average_order_value()
    }

    async fn certification_count(&self) -> i32 {
        self.certifications.len() as i32
    }

    async fn tag_count(&self) -> i32 {
        self.tags.len() as i32
    }

    async fn attachment_count(&self) -> i32 {
        self.attachments.len() as i32
    }

    async fn days_since_last_order(&self) -> Option<i64> {
        self.last_order_date.map(|last_order| {
            let duration = Utc::now() - last_order;
            duration.num_days()
        })
    }

    async fn contract_days_remaining(&self) -> Option<i64> {
        self.contract_end_date.map(|end_date| {
            let duration = end_date - Utc::now();
            duration.num_days().max(0)
        })
    }

    async fn is_preferred_vendor(&self) -> bool {
        matches!(self.tier, VendorTier::Preferred)
    }

    async fn has_credit_limit(&self) -> bool {
        self.credit_limit.is_some()
    }

    async fn has_auto_approval(&self) -> bool {
        self.auto_approval_limit.is_some()
    }

    async fn display_name(&self) -> String {
        if let Some(code) = &self.vendor_code {
            format!("{} ({})", self.name, code)
        } else {
            self.name.clone()
        }
    }
}
