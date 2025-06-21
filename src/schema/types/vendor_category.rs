use crate::models::{ prelude::*, vendor_category::{ VendorCategory, CategoryStatus } };

#[Object]
impl VendorCategory {
    async fn id(&self) -> &str {
        &self.id
    }

    async fn name(&self) -> &str {
        &self.name
    }

    async fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    async fn category_type(&self) -> &str {
        self.category_type.to_str()
    }

    async fn status(&self) -> &str {
        self.status.to_str()
    }

    async fn parent_category_id(&self) -> Option<&str> {
        self.parent_category_id.as_deref()
    }

    async fn level(&self) -> i32 {
        self.level
    }

    async fn sort_order(&self) -> i32 {
        self.sort_order
    }

    async fn code(&self) -> Option<&str> {
        self.code.as_deref()
    }

    async fn color(&self) -> Option<&str> {
        self.color.as_deref()
    }

    async fn icon(&self) -> Option<&str> {
        self.icon.as_deref()
    }

    async fn required_fields(&self) -> &Vec<String> {
        &self.required_fields
    }

    async fn default_approval_workflow(&self) -> Option<&str> {
        self.default_approval_workflow.as_deref()
    }

    async fn compliance_requirements(&self) -> &Vec<String> {
        &self.compliance_requirements
    }

    async fn tax_category(&self) -> Option<&str> {
        self.tax_category.as_deref()
    }

    async fn risk_level(&self) -> &str {
        &self.risk_level
    }

    async fn auto_approval_limit(&self) -> Option<f64> {
        self.auto_approval_limit
    }

    async fn vendor_count(&self) -> i32 {
        self.vendor_count
    }

    async fn active(&self) -> bool {
        self.active
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

    #[graphql(name = "is_root_category")]
    async fn check_is_root_category(&self) -> bool {
        self.is_root_category()
    }

    #[graphql(name = "has_parent")]
    async fn check_has_parent(&self) -> bool {
        self.has_parent()
    }

    #[graphql(name = "is_usable")]
    async fn check_is_usable(&self) -> bool {
        self.is_usable()
    }

    async fn required_fields_count(&self) -> i32 {
        self.required_fields.len() as i32
    }

    async fn compliance_requirements_count(&self) -> i32 {
        self.compliance_requirements.len() as i32
    }

    async fn has_auto_approval(&self) -> bool {
        self.auto_approval_limit.is_some()
    }

    async fn has_vendors(&self) -> bool {
        self.vendor_count > 0
    }

    async fn is_high_risk(&self) -> bool {
        matches!(self.risk_level.as_str(), "high" | "critical")
    }

    async fn is_archived(&self) -> bool {
        matches!(self.status, CategoryStatus::Archived)
    }

    async fn display_name(&self) -> String {
        if let Some(code) = &self.code {
            format!("{} ({})", self.name, code)
        } else {
            self.name.clone()
        }
    }
}
