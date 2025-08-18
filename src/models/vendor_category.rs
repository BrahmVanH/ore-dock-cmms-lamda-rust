use std::collections::HashMap;

use async_graphql::Enum;
use aws_sdk_dynamodb::types::AttributeValue;
use chrono::{ DateTime, Utc };
use serde::{ Deserialize, Serialize };
use tracing::info;

use crate::error::AppError;

#[derive(Enum, Copy, Clone, PartialEq, Eq, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CategoryType {
    Service, // Service providers
    Supplier, // Material/equipment suppliers
    Contractor, // Construction/maintenance contractors
    Consultant, // Professional services
    Technology, // IT/software vendors
    Equipment, // Equipment manufacturers/dealers
    Emergency, // Emergency service providers
    Utility, // Utility providers
    Other, // Other category types
}

impl CategoryType {
    pub(crate) fn to_str(&self) -> &str {
        match self {
            CategoryType::Service => "service",
            CategoryType::Supplier => "supplier",
            CategoryType::Contractor => "contractor",
            CategoryType::Consultant => "consultant",
            CategoryType::Technology => "technology",
            CategoryType::Equipment => "equipment",
            CategoryType::Emergency => "emergency",
            CategoryType::Utility => "utility",
            CategoryType::Other => "other",
        }
    }

    pub(crate) fn to_string(&self) -> String {
        self.to_str().to_string()
    }

    pub(crate) fn from_string(s: &str) -> Result<CategoryType, AppError> {
        match s {
            "service" => Ok(Self::Service),
            "supplier" => Ok(Self::Supplier),
            "contractor" => Ok(Self::Contractor),
            "consultant" => Ok(Self::Consultant),
            "technology" => Ok(Self::Technology),
            "equipment" => Ok(Self::Equipment),
            "emergency" => Ok(Self::Emergency),
            "utility" => Ok(Self::Utility),
            "other" => Ok(Self::Other),
            _ => Err(AppError::ValidationError("Invalid category type".to_string())),
        }
    }
}

#[derive(Enum, Copy, Clone, PartialEq, Eq, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CategoryStatus {
    Active, // Currently active category
    Inactive, // Temporarily inactive
    Deprecated, // Being phased out
    Archived, // Archived but kept for historical records
}

impl CategoryStatus {
    pub(crate) fn to_str(&self) -> &str {
        match self {
            CategoryStatus::Active => "active",
            CategoryStatus::Inactive => "inactive",
            CategoryStatus::Deprecated => "deprecated",
            CategoryStatus::Archived => "archived",
        }
    }

    pub(crate) fn to_string(&self) -> String {
        self.to_str().to_string()
    }

    pub(crate) fn from_string(s: &str) -> Result<CategoryStatus, AppError> {
        match s {
            "active" => Ok(Self::Active),
            "inactive" => Ok(Self::Inactive),
            "deprecated" => Ok(Self::Deprecated),
            "archived" => Ok(Self::Archived),
            _ => Err(AppError::ValidationError("Invalid category status".to_string())),
        }
    }
}

/// Represents a Vendor Category in the system
///
/// # Fields
///
/// * `id` - Unique identifier for the vendor category
/// * `name` - Name of the category
/// * `description` - Optional description of the category
/// * `category_type` - Type of category (service, supplier, contractor, etc.)
/// * `status` - Current status of the category
/// * `parent_category_id` - Optional parent category for hierarchical structure
/// * `level` - Hierarchy level (0 = root, 1 = first level, etc.)
/// * `sort_order` - Display order within the same level
/// * `code` - Optional short code for the category
/// * `color` - Optional color for UI display
/// * `icon` - Optional icon identifier for UI display
/// * `required_fields` - List of required fields for vendors in this category
/// * `default_approval_workflow` - Default approval workflow for vendors in this category
/// * `compliance_requirements` - List of compliance requirements for this category
/// * `tax_category` - Tax category for financial processing
/// * `risk_level` - Risk assessment level (low, medium, high, critical)
/// * `auto_approval_limit` - Automatic approval limit for purchases from vendors in this category
/// * `vendor_count` - Number of vendors currently in this category
/// * `active` - Whether this category is currently active
/// * `created_by` - User who created this category
/// * `created_at` - Date and time of creation
/// * `updated_at` - Date and time of last update
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VendorCategory {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub category_type: CategoryType,
    pub status: CategoryStatus,
    pub parent_category_id: Option<String>,
    pub level: i32,
    pub sort_order: i32,
    pub code: Option<String>,
    pub color: Option<String>,
    pub icon: Option<String>,
    pub required_fields: Vec<String>,
    pub default_approval_workflow: Option<String>,
    pub compliance_requirements: Vec<String>,
    pub tax_category: Option<String>,
    pub risk_level: String,
    pub auto_approval_limit: Option<f64>,
    pub vendor_count: i32,
    pub active: bool,
    pub created_by: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Defines methods for VendorCategory
impl VendorCategory {
    /// Creates new VendorCategory instance
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier
    /// * `name` - Name of the category
    /// * `description` - Optional description
    /// * `category_type` - Type of category as string
    /// * `status` - Status as string
    /// * `parent_category_id` - Optional parent category ID
    /// * `level` - Hierarchy level
    /// * `sort_order` - Display sort order
    /// * `code` - Optional category code
    /// * `color` - Optional UI color
    /// * `icon` - Optional UI icon
    /// * `required_fields` - List of required fields
    /// * `default_approval_workflow` - Optional default workflow
    /// * `compliance_requirements` - List of compliance requirements
    /// * `tax_category` - Optional tax category
    /// * `risk_level` - Risk assessment level
    /// * `auto_approval_limit` - Optional auto-approval limit
    /// * `active` - Whether category is active
    /// * `created_by` - Optional creator user ID
    ///
    /// # Returns
    ///
    /// New VendorCategory instance
    pub fn new(
        id: String,
        name: String,
        description: Option<String>,
        category_type: String,
        status: String,
        parent_category_id: Option<String>,
        level: i32,
        sort_order: i32,
        code: Option<String>,
        color: Option<String>,
        icon: Option<String>,
        required_fields: Vec<String>,
        default_approval_workflow: Option<String>,
        compliance_requirements: Vec<String>,
        tax_category: Option<String>,
        risk_level: String,
        auto_approval_limit: Option<f64>,
        active: bool,
        created_by: Option<String>
    ) -> Result<Self, AppError> {
        let now = Utc::now();

        if name.trim().is_empty() {
            return Err(AppError::ValidationError("Category name cannot be empty".to_string()));
        }

        let category_type_enum = CategoryType::from_string(&category_type)?;
        let status_enum = CategoryStatus::from_string(&status)?;

        // Validate level
        if level < 0 {
            return Err(AppError::ValidationError("Level cannot be negative".to_string()));
        }

        // Validate sort order
        if sort_order < 0 {
            return Err(AppError::ValidationError("Sort order cannot be negative".to_string()));
        }

        // Validate auto approval limit
        if let Some(limit) = auto_approval_limit {
            if limit < 0.0 {
                return Err(
                    AppError::ValidationError("Auto approval limit cannot be negative".to_string())
                );
            }
        }

        // Validate risk level
        let valid_risk_levels = ["low", "medium", "high", "critical"];
        if !valid_risk_levels.contains(&risk_level.as_str()) {
            return Err(
                AppError::ValidationError(
                    "Risk level must be one of: low, medium, high, critical".to_string()
                )
            );
        }

        Ok(Self {
            id,
            name,
            description,
            category_type: category_type_enum,
            status: status_enum,
            parent_category_id,
            level,
            sort_order,
            code,
            color,
            icon,
            required_fields,
            default_approval_workflow,
            compliance_requirements,
            tax_category,
            risk_level,
            auto_approval_limit,
            vendor_count: 0, // Initialize to 0
            active,
            created_by,
            created_at: now,
            updated_at: now,
        })
    }

    /// Creates VendorCategory instance from DynamoDB item
    ///
    /// # Arguments
    ///
    /// * `item` - The dynamo db item
    ///
    /// # Returns
    ///
    /// 'Some' VendorCategory if item fields match, 'None' otherwise
    pub fn from_item(item: &HashMap<String, AttributeValue>) -> Option<Self> {
        info!("calling from_item with: {:?}", &item);

        let id = item.get("id")?.as_s().ok()?.to_string();
        let name = item.get("name")?.as_s().ok()?.to_string();

        let description = item
            .get("description")
            .and_then(|v| v.as_s().ok())
            .map(|s| s.to_string());

        let category_type_str = item.get("category_type")?.as_s().ok()?;
        let category_type = CategoryType::from_string(&category_type_str)
            .map_err(|e| e)
            .ok()?;

        let status_str = item.get("status")?.as_s().ok()?;
        let status = CategoryStatus::from_string(&status_str)
            .map_err(|e| e)
            .ok()?;

        let parent_category_id = item
            .get("parent_category_id")
            .and_then(|v| v.as_s().ok())
            .map(|s| s.to_string());

        let level = item
            .get("level")
            .and_then(|v| v.as_n().ok())
            .and_then(|s| s.parse::<i32>().ok())
            .unwrap_or(0);

        let sort_order = item
            .get("sort_order")
            .and_then(|v| v.as_n().ok())
            .and_then(|s| s.parse::<i32>().ok())
            .unwrap_or(0);

        let code = item
            .get("code")
            .and_then(|v| v.as_s().ok())
            .map(|s| s.to_string());

        let color = item
            .get("color")
            .and_then(|v| v.as_s().ok())
            .map(|s| s.to_string());

        let icon = item
            .get("icon")
            .and_then(|v| v.as_s().ok())
            .map(|s| s.to_string());

        let required_fields = item
            .get("required_fields")
            .and_then(|v| v.as_ss().ok())
            .map(|fields| {
                fields
                    .iter()
                    .map(|s| s.to_string())
                    .collect()
            })
            .unwrap_or_default();

        let default_approval_workflow = item
            .get("default_approval_workflow")
            .and_then(|v| v.as_s().ok())
            .map(|s| s.to_string());

        let compliance_requirements = item
            .get("compliance_requirements")
            .and_then(|v| v.as_ss().ok())
            .map(|reqs| {
                reqs.iter()
                    .map(|s| s.to_string())
                    .collect()
            })
            .unwrap_or_default();

        let tax_category = item
            .get("tax_category")
            .and_then(|v| v.as_s().ok())
            .map(|s| s.to_string());

        let risk_level = item
            .get("risk_level")
            .and_then(|v| v.as_s().ok())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "medium".to_string());

        let auto_approval_limit = item
            .get("auto_approval_limit")
            .and_then(|v| v.as_n().ok())
            .and_then(|s| s.parse::<f64>().ok());

        let vendor_count = item
            .get("vendor_count")
            .and_then(|v| v.as_n().ok())
            .and_then(|s| s.parse::<i32>().ok())
            .unwrap_or(0);

        let active = item
            .get("active")
            .and_then(|v| v.as_bool().ok())
            .unwrap_or(&true);

        let created_by = item
            .get("created_by")
            .and_then(|v| v.as_s().ok())
            .map(|s| s.to_string());

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
            category_type,
            status,
            parent_category_id,
            level,
            sort_order,
            code,
            color,
            icon,
            required_fields,
            default_approval_workflow,
            compliance_requirements,
            tax_category,
            risk_level,
            auto_approval_limit,
            vendor_count,
            active: *active,
            created_by,
            created_at,
            updated_at,
        });

        info!("result of from_item on vendor_category: {:?}", res);
        res
    }

    /// Creates DynamoDB item from VendorCategory instance
    ///
    /// # Arguments
    ///
    /// * `self` - borrowed instance of self
    ///
    /// # Returns
    ///
    /// HashMap representing DB item for VendorCategory instance
    pub fn to_item(&self) -> HashMap<String, AttributeValue> {
        let mut item = HashMap::new();

        item.insert("id".to_string(), AttributeValue::S(self.id.clone()));
        item.insert("name".to_string(), AttributeValue::S(self.name.clone()));

        if let Some(desc) = &self.description {
            item.insert("description".to_string(), AttributeValue::S(desc.clone()));
        }

        item.insert("category_type".to_string(), AttributeValue::S(self.category_type.to_string()));
        item.insert("status".to_string(), AttributeValue::S(self.status.to_string()));

        if let Some(parent_id) = &self.parent_category_id {
            item.insert("parent_category_id".to_string(), AttributeValue::S(parent_id.clone()));
        }

        item.insert("level".to_string(), AttributeValue::N(self.level.to_string()));
        item.insert("sort_order".to_string(), AttributeValue::N(self.sort_order.to_string()));

        if let Some(code) = &self.code {
            item.insert("code".to_string(), AttributeValue::S(code.clone()));
        }

        if let Some(color) = &self.color {
            item.insert("color".to_string(), AttributeValue::S(color.clone()));
        }

        if let Some(icon) = &self.icon {
            item.insert("icon".to_string(), AttributeValue::S(icon.clone()));
        }

        // Store required fields as string set
        if !self.required_fields.is_empty() {
            item.insert(
                "required_fields".to_string(),
                AttributeValue::Ss(self.required_fields.clone())
            );
        }

        if let Some(workflow) = &self.default_approval_workflow {
            item.insert(
                "default_approval_workflow".to_string(),
                AttributeValue::S(workflow.clone())
            );
        }

        // Store compliance requirements as string set
        if !self.compliance_requirements.is_empty() {
            item.insert(
                "compliance_requirements".to_string(),
                AttributeValue::Ss(self.compliance_requirements.clone())
            );
        }

        if let Some(tax_cat) = &self.tax_category {
            item.insert("tax_category".to_string(), AttributeValue::S(tax_cat.clone()));
        }

        item.insert("risk_level".to_string(), AttributeValue::S(self.risk_level.clone()));

        if let Some(limit) = &self.auto_approval_limit {
            item.insert("auto_approval_limit".to_string(), AttributeValue::N(limit.to_string()));
        }

        item.insert("vendor_count".to_string(), AttributeValue::N(self.vendor_count.to_string()));
        item.insert("active".to_string(), AttributeValue::Bool(self.active));

        if let Some(creator) = &self.created_by {
            item.insert("created_by".to_string(), AttributeValue::S(creator.clone()));
        }

        item.insert("created_at".to_string(), AttributeValue::S(self.created_at.to_string()));
        item.insert("updated_at".to_string(), AttributeValue::S(self.updated_at.to_string()));

        item
    }

    /// Checks if this category is at the root level
    pub fn is_root_category(&self) -> bool {
        self.parent_category_id.is_none() && self.level == 0
    }

    /// Checks if this category has a parent
    pub fn has_parent(&self) -> bool {
        self.parent_category_id.is_some()
    }

    /// Checks if this category is currently usable
    pub fn is_usable(&self) -> bool {
        self.active && matches!(self.status, CategoryStatus::Active)
    }

    /// Increments the vendor count
    pub fn increment_vendor_count(&mut self) {
        self.vendor_count += 1;
        self.updated_at = Utc::now();
    }

    /// Decrements the vendor count
    pub fn decrement_vendor_count(&mut self) {
        if self.vendor_count > 0 {
            self.vendor_count -= 1;
            self.updated_at = Utc::now();
        }
    }

    /// Updates the vendor count to a specific value
    pub fn update_vendor_count(&mut self, count: i32) -> Result<(), AppError> {
        if count < 0 {
            return Err(AppError::ValidationError("Vendor count cannot be negative".to_string()));
        }
        self.vendor_count = count;
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Adds a required field to this category
    pub fn add_required_field(&mut self, field: String) {
        if !self.required_fields.contains(&field) {
            self.required_fields.push(field);
            self.updated_at = Utc::now();
        }
    }

    /// Removes a required field from this category
    pub fn remove_required_field(&mut self, field: &str) {
        if let Some(pos) = self.required_fields.iter().position(|x| x == field) {
            self.required_fields.remove(pos);
            self.updated_at = Utc::now();
        }
    }

    /// Adds a compliance requirement to this category
    pub fn add_compliance_requirement(&mut self, requirement: String) {
        if !self.compliance_requirements.contains(&requirement) {
            self.compliance_requirements.push(requirement);
            self.updated_at = Utc::now();
        }
    }

    /// Removes a compliance requirement from this category
    pub fn remove_compliance_requirement(&mut self, requirement: &str) {
        if let Some(pos) = self.compliance_requirements.iter().position(|x| x == requirement) {
            self.compliance_requirements.remove(pos);
            self.updated_at = Utc::now();
        }
    }

    /// Updates the category status
    pub fn update_status(&mut self, new_status: String) -> Result<(), AppError> {
        let status_enum = CategoryStatus::from_string(&new_status)?;
        self.status = status_enum;
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Archives the category
    pub fn archive(&mut self) {
        self.status = CategoryStatus::Archived;
        self.active = false;
        self.updated_at = Utc::now();
    }

    /// Reactivates the category
    pub fn reactivate(&mut self) -> Result<(), AppError> {
        if matches!(self.status, CategoryStatus::Archived) {
            return Err(
                AppError::ValidationError("Cannot reactivate archived category".to_string())
            );
        }
        self.status = CategoryStatus::Active;
        self.active = true;
        self.updated_at = Utc::now();
        Ok(())
    }
}
