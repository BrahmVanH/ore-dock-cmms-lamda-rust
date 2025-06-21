use std::collections::HashMap;

use aws_sdk_dynamodb::types::AttributeValue;
use chrono::{ DateTime, Utc };
use serde::{ Deserialize, Serialize };
use serde_json::Value as Json;
use tracing::info;

use crate::error::AppError;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum VendorStatus {
    Active, // Currently active vendor
    Inactive, // Temporarily inactive
    Pending, // Pending approval/verification
    Suspended, // Suspended due to issues
    Blacklisted, // Blacklisted - cannot do business
    Terminated, // Permanently terminated
}

impl VendorStatus {
    pub fn to_str(&self) -> &str {
        match self {
            VendorStatus::Active => "active",
            VendorStatus::Inactive => "inactive",
            VendorStatus::Pending => "pending",
            VendorStatus::Suspended => "suspended",
            VendorStatus::Blacklisted => "blacklisted",
            VendorStatus::Terminated => "terminated",
        }
    }

    pub fn to_string(&self) -> String {
        self.to_str().to_string()
    }

    pub fn from_string(s: &str) -> Result<VendorStatus, AppError> {
        match s {
            "active" => Ok(Self::Active),
            "inactive" => Ok(Self::Inactive),
            "pending" => Ok(Self::Pending),
            "suspended" => Ok(Self::Suspended),
            "blacklisted" => Ok(Self::Blacklisted),
            "terminated" => Ok(Self::Terminated),
            _ => Err(AppError::ValidationError("Invalid vendor status".to_string())),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum VendorTier {
    Preferred, // Preferred vendor with best terms
    Standard, // Standard vendor
    Occasional, // Occasional use vendor
    Trial, // Trial period vendor
}

impl VendorTier {
    pub fn to_str(&self) -> &str {
        match self {
            VendorTier::Preferred => "preferred",
            VendorTier::Standard => "standard",
            VendorTier::Occasional => "occasional",
            VendorTier::Trial => "trial",
        }
    }

    pub fn to_string(&self) -> String {
        self.to_str().to_string()
    }

    pub fn from_string(s: &str) -> Result<VendorTier, AppError> {
        match s {
            "preferred" => Ok(Self::Preferred),
            "standard" => Ok(Self::Standard),
            "occasional" => Ok(Self::Occasional),
            "trial" => Ok(Self::Trial),
            _ => Err(AppError::ValidationError("Invalid vendor tier".to_string())),
        }
    }
}

/// Represents a Vendor in the system
///
/// # Fields
///
/// * `id` - Unique identifier for the vendor
/// * `name` - Company/vendor name
/// * `legal_name` - Official legal business name
/// * `vendor_code` - Internal vendor code/identifier
/// * `vendor_category_id` - ID of the vendor category
/// * `status` - Current status of the vendor
/// * `tier` - Vendor tier (preferred, standard, occasional, trial)
/// * `phone_number` - Primary phone number
/// * `secondary_phone` - Optional secondary phone number
/// * `email_address` - Primary email address
/// * `secondary_email` - Optional secondary email address
/// * `website` - Company website URL
/// * `tax_id` - Tax identification number
/// * `registration_number` - Business registration number
/// * `payment_terms` - Payment terms (e.g., "Net 30", "COD")
/// * `currency` - Preferred currency for transactions
/// * `credit_limit` - Optional credit limit
/// * `primary_contact_name` - Name of primary contact person
/// * `primary_contact_title` - Job title of primary contact
/// * `primary_contact_email` - Email of primary contact
/// * `primary_contact_phone` - Phone of primary contact
/// * `billing_address` - Billing address as JSON
/// * `shipping_address` - Shipping address as JSON
/// * `certifications` - List of certifications
/// * `compliance_status` - Compliance verification status
/// * `insurance_info` - Insurance information as JSON
/// * `contract_start_date` - Contract start date
/// * `contract_end_date` - Contract end date
/// * `last_order_date` - Date of last order
/// * `total_orders` - Total number of orders placed
/// * `total_spent` - Total amount spent with vendor
/// * `average_rating` - Average performance rating
/// * `performance_notes` - Performance evaluation notes
/// * `emergency_contact` - Emergency contact information as JSON
/// * `preferred_communication` - Preferred communication method
/// * `time_zone` - Vendor's time zone
/// * `business_hours` - Business hours as JSON
/// * `tags` - List of tags for categorization
/// * `custom_fields` - Custom fields as JSON
/// * `attachments` - List of attachment URLs/IDs
/// * `approval_required` - Whether purchases require approval
/// * `auto_approval_limit` - Automatic approval limit
/// * `notes` - Internal notes about the vendor
/// * `created_by` - User who created this vendor record
/// * `created_at` - Date and time of creation
/// * `updated_at` - Date and time of last update
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Vendor {
    pub id: String,
    pub name: String,
    pub legal_name: Option<String>,
    pub vendor_code: Option<String>,
    pub vendor_category_id: String,
    pub status: VendorStatus,
    pub tier: VendorTier,
    pub phone_number: String,
    pub secondary_phone: Option<String>,
    pub email_address: String,
    pub secondary_email: Option<String>,
    pub website: Option<String>,
    pub tax_id: String,
    pub registration_number: Option<String>,
    pub payment_terms: String,
    pub currency: String,
    pub credit_limit: Option<f64>,
    pub primary_contact_name: String,
    pub primary_contact_title: String,
    pub primary_contact_email: Option<String>,
    pub primary_contact_phone: Option<String>,
    pub billing_address: Option<Json>,
    pub shipping_address: Option<Json>,
    pub certifications: Vec<String>,
    pub compliance_status: String,
    pub insurance_info: Option<Json>,
    pub contract_start_date: Option<DateTime<Utc>>,
    pub contract_end_date: Option<DateTime<Utc>>,
    pub last_order_date: Option<DateTime<Utc>>,
    pub total_orders: i32,
    pub total_spent: f64,
    pub average_rating: Option<f64>,
    pub performance_notes: Option<String>,
    pub emergency_contact: Option<Json>,
    pub preferred_communication: String,
    pub time_zone: Option<String>,
    pub business_hours: Option<Json>,
    pub tags: Vec<String>,
    pub custom_fields: Option<Json>,
    pub attachments: Vec<String>,
    pub approval_required: bool,
    pub auto_approval_limit: Option<f64>,
    pub notes: Option<String>,
    pub created_by: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Defines methods for Vendor
impl Vendor {
    /// Creates new Vendor instance
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier
    /// * `name` - Vendor name
    /// * `legal_name` - Optional legal name
    /// * `vendor_code` - Optional vendor code
    /// * `vendor_category_id` - Category ID
    /// * `status` - Status as string
    /// * `tier` - Tier as string
    /// * `phone_number` - Phone number
    /// * `secondary_phone` - Optional secondary phone
    /// * `email_address` - Email address
    /// * `secondary_email` - Optional secondary email
    /// * `website` - Optional website
    /// * `tax_id` - Tax ID
    /// * `registration_number` - Optional registration number
    /// * `payment_terms` - Payment terms
    /// * `currency` - Currency code
    /// * `credit_limit` - Optional credit limit
    /// * `primary_contact_name` - Primary contact name
    /// * `primary_contact_title` - Primary contact title
    /// * `primary_contact_email` - Optional primary contact email
    /// * `primary_contact_phone` - Optional primary contact phone
    /// * `billing_address` - Optional billing address as JSON
    /// * `shipping_address` - Optional shipping address as JSON
    /// * `certifications` - List of certifications
    /// * `compliance_status` - Compliance status
    /// * `insurance_info` - Optional insurance info as JSON
    /// * `contract_start_date` - Optional contract start date
    /// * `contract_end_date` - Optional contract end date
    /// * `emergency_contact` - Optional emergency contact as JSON
    /// * `preferred_communication` - Preferred communication method
    /// * `time_zone` - Optional timezone
    /// * `business_hours` - Optional business hours as JSON
    /// * `tags` - List of tags
    /// * `custom_fields` - Optional custom fields as JSON
    /// * `attachments` - List of attachment IDs
    /// * `approval_required` - Whether approval is required
    /// * `auto_approval_limit` - Optional auto approval limit
    /// * `notes` - Optional notes
    /// * `created_by` - Optional creator user ID
    ///
    /// # Returns
    ///
    /// New Vendor instance
    pub fn new(
        id: String,
        name: String,
        legal_name: Option<String>,
        vendor_code: Option<String>,
        vendor_category_id: String,
        status: String,
        tier: String,
        phone_number: String,
        secondary_phone: Option<String>,
        email_address: String,
        secondary_email: Option<String>,
        website: Option<String>,
        tax_id: String,
        registration_number: Option<String>,
        payment_terms: String,
        currency: String,
        credit_limit: Option<f64>,
        primary_contact_name: String,
        primary_contact_title: String,
        primary_contact_email: Option<String>,
        primary_contact_phone: Option<String>,
        billing_address: Option<Json>,
        shipping_address: Option<Json>,
        certifications: Vec<String>,
        compliance_status: String,
        insurance_info: Option<Json>,
        contract_start_date: Option<DateTime<Utc>>,
        contract_end_date: Option<DateTime<Utc>>,
        emergency_contact: Option<Json>,
        preferred_communication: String,
        time_zone: Option<String>,
        business_hours: Option<Json>,
        tags: Vec<String>,
        custom_fields: Option<Json>,
        attachments: Vec<String>,
        approval_required: &bool,
        auto_approval_limit: Option<f64>,
        notes: Option<String>,
        created_by: Option<String>
    ) -> Result<Self, AppError> {
        let now = Utc::now();

        // Validate required fields
        if name.trim().is_empty() {
            return Err(AppError::ValidationError("Vendor name cannot be empty".to_string()));
        }

        if vendor_category_id.trim().is_empty() {
            return Err(AppError::ValidationError("Vendor category ID cannot be empty".to_string()));
        }

        if phone_number.trim().is_empty() {
            return Err(AppError::ValidationError("Phone number cannot be empty".to_string()));
        }

        if email_address.trim().is_empty() {
            return Err(AppError::ValidationError("Email address cannot be empty".to_string()));
        }

        // Validate email format (basic validation)
        if !email_address.contains('@') || !email_address.contains('.') {
            return Err(AppError::ValidationError("Invalid email format".to_string()));
        }

        if tax_id.trim().is_empty() {
            return Err(AppError::ValidationError("Tax ID cannot be empty".to_string()));
        }

        if payment_terms.trim().is_empty() {
            return Err(AppError::ValidationError("Payment terms cannot be empty".to_string()));
        }

        if currency.trim().is_empty() {
            return Err(AppError::ValidationError("Currency cannot be empty".to_string()));
        }

        if primary_contact_name.trim().is_empty() {
            return Err(
                AppError::ValidationError("Primary contact name cannot be empty".to_string())
            );
        }

        if primary_contact_title.trim().is_empty() {
            return Err(
                AppError::ValidationError("Primary contact title cannot be empty".to_string())
            );
        }

        if compliance_status.trim().is_empty() {
            return Err(AppError::ValidationError("Compliance status cannot be empty".to_string()));
        }

        if preferred_communication.trim().is_empty() {
            return Err(
                AppError::ValidationError("Preferred communication cannot be empty".to_string())
            );
        }

        // Validate secondary email if provided
        if let Some(ref sec_email) = secondary_email {
            if
                !sec_email.trim().is_empty() &&
                (!sec_email.contains('@') || !sec_email.contains('.'))
            {
                return Err(AppError::ValidationError("Invalid secondary email format".to_string()));
            }
        }

        // Validate primary contact email if provided
        if let Some(ref contact_email) = primary_contact_email {
            if
                !contact_email.trim().is_empty() &&
                (!contact_email.contains('@') || !contact_email.contains('.'))
            {
                return Err(
                    AppError::ValidationError("Invalid primary contact email format".to_string())
                );
            }
        }

        // Validate credit limit
        if let Some(limit) = credit_limit {
            if limit < 0.0 {
                return Err(
                    AppError::ValidationError("Credit limit cannot be negative".to_string())
                );
            }
        }

        // Validate auto approval limit
        if let Some(limit) = auto_approval_limit {
            if limit < 0.0 {
                return Err(
                    AppError::ValidationError("Auto approval limit cannot be negative".to_string())
                );
            }
        }

        // Validate contract dates
        if let (Some(start), Some(end)) = (&contract_start_date, &contract_end_date) {
            if end <= start {
                return Err(
                    AppError::ValidationError(
                        "Contract end date must be after start date".to_string()
                    )
                );
            }
        }

        let status_enum = VendorStatus::from_string(&status)?;
        let tier_enum = VendorTier::from_string(&tier)?;

        Ok(Self {
            id,
            name,
            legal_name,
            vendor_code,
            vendor_category_id,
            status: status_enum,
            tier: tier_enum,
            phone_number,
            secondary_phone,
            email_address,
            secondary_email,
            website,
            tax_id,
            registration_number,
            payment_terms,
            currency,
            credit_limit,
            primary_contact_name,
            primary_contact_title,
            primary_contact_email,
            primary_contact_phone,
            billing_address,
            shipping_address,
            certifications,
            compliance_status,
            insurance_info,
            contract_start_date,
            contract_end_date,
            last_order_date: None,
            total_orders: 0,
            total_spent: 0.0,
            average_rating: None,
            performance_notes: None,
            emergency_contact,
            preferred_communication,
            time_zone,
            business_hours,
            tags,
            custom_fields,
            attachments,
            approval_required: *approval_required,
            auto_approval_limit,
            notes,
            created_by,
            created_at: now,
            updated_at: now,
        })
    }

    /// Creates Vendor instance from DynamoDB item
    ///
    /// # Arguments
    ///
    /// * `item` - The dynamo db item
    ///
    /// # Returns
    ///
    /// 'Some' Vendor if item fields match, 'None' otherwise
    pub fn from_item(item: &HashMap<String, AttributeValue>) -> Option<Self> {
        info!("calling from_item with: {:?}", &item);

        let id = item.get("id")?.as_s().ok()?.to_string();
        let name = item.get("name")?.as_s().ok()?.to_string();

        let legal_name = item
            .get("legal_name")
            .and_then(|v| v.as_s().ok())
            .map(|s| s.to_string());

        let vendor_code = item
            .get("vendor_code")
            .and_then(|v| v.as_s().ok())
            .map(|s| s.to_string());

        let vendor_category_id = item.get("vendor_category_id")?.as_s().ok()?.to_string();

        let status_str = item.get("status")?.as_s().ok()?;
        let status = VendorStatus::from_string(&status_str)
            .map_err(|e| e)
            .ok()?;

        let tier_str = item.get("tier")?.as_s().ok()?;
        let tier = VendorTier::from_string(&tier_str)
            .map_err(|e| e)
            .ok()?;

        let phone_number = item.get("phone_number")?.as_s().ok()?.to_string();

        let secondary_phone = item
            .get("secondary_phone")
            .and_then(|v| v.as_s().ok())
            .map(|s| s.to_string());

        let email_address = item.get("email_address")?.as_s().ok()?.to_string();

        let secondary_email = item
            .get("secondary_email")
            .and_then(|v| v.as_s().ok())
            .map(|s| s.to_string());

        let website = item
            .get("website")
            .and_then(|v| v.as_s().ok())
            .map(|s| s.to_string());

        let tax_id = item.get("tax_id")?.as_s().ok()?.to_string();

        let registration_number = item
            .get("registration_number")
            .and_then(|v| v.as_s().ok())
            .map(|s| s.to_string());

        let payment_terms = item.get("payment_terms")?.as_s().ok()?.to_string();
        let currency = item.get("currency")?.as_s().ok()?.to_string();

        let credit_limit = item
            .get("credit_limit")
            .and_then(|v| v.as_n().ok())
            .and_then(|s| s.parse::<f64>().ok());

        let primary_contact_name = item.get("primary_contact_name")?.as_s().ok()?.to_string();
        let primary_contact_title = item.get("primary_contact_title")?.as_s().ok()?.to_string();

        let primary_contact_email = item
            .get("primary_contact_email")
            .and_then(|v| v.as_s().ok())
            .map(|s| s.to_string());

        let primary_contact_phone = item
            .get("primary_contact_phone")
            .and_then(|v| v.as_s().ok())
            .map(|s| s.to_string());

        let billing_address = item
            .get("billing_address")
            .and_then(|v| v.as_s().ok())
            .and_then(|s| serde_json::from_str::<Json>(s).ok());

        let shipping_address = item
            .get("shipping_address")
            .and_then(|v| v.as_s().ok())
            .and_then(|s| serde_json::from_str::<Json>(s).ok());

        let certifications = item
            .get("certifications")
            .and_then(|v| v.as_ss().ok())
            .map(|certs| {
                certs
                    .iter()
                    .map(|s| s.to_string())
                    .collect()
            })
            .unwrap_or_default();

        let compliance_status = item.get("compliance_status")?.as_s().ok()?.to_string();

        let insurance_info = item
            .get("insurance_info")
            .and_then(|v| v.as_s().ok())
            .and_then(|s| serde_json::from_str::<Json>(s).ok());

        let contract_start_date = item
            .get("contract_start_date")
            .and_then(|v| v.as_s().ok())
            .and_then(|s| s.parse::<DateTime<Utc>>().ok());

        let contract_end_date = item
            .get("contract_end_date")
            .and_then(|v| v.as_s().ok())
            .and_then(|s| s.parse::<DateTime<Utc>>().ok());

        let last_order_date = item
            .get("last_order_date")
            .and_then(|v| v.as_s().ok())
            .and_then(|s| s.parse::<DateTime<Utc>>().ok());

        let total_orders = item
            .get("total_orders")
            .and_then(|v| v.as_n().ok())
            .and_then(|s| s.parse::<i32>().ok())
            .unwrap_or(0);

        let total_spent = item
            .get("total_spent")
            .and_then(|v| v.as_n().ok())
            .and_then(|s| s.parse::<f64>().ok())
            .unwrap_or(0.0);

        let average_rating = item
            .get("average_rating")
            .and_then(|v| v.as_n().ok())
            .and_then(|s| s.parse::<f64>().ok());

        let performance_notes = item
            .get("performance_notes")
            .and_then(|v| v.as_s().ok())
            .map(|s| s.to_string());

        let emergency_contact = item
            .get("emergency_contact")
            .and_then(|v| v.as_s().ok())
            .and_then(|s| serde_json::from_str::<Json>(s).ok());

        let preferred_communication = item.get("preferred_communication")?.as_s().ok()?.to_string();

        let time_zone = item
            .get("time_zone")
            .and_then(|v| v.as_s().ok())
            .map(|s| s.to_string());

        let business_hours = item
            .get("business_hours")
            .and_then(|v| v.as_s().ok())
            .and_then(|s| serde_json::from_str::<Json>(s).ok());

        let tags = item
            .get("tags")
            .and_then(|v| v.as_ss().ok())
            .map(|tag_list| {
                tag_list
                    .iter()
                    .map(|s| s.to_string())
                    .collect()
            })
            .unwrap_or_default();

        let custom_fields = item
            .get("custom_fields")
            .and_then(|v| v.as_s().ok())
            .and_then(|s| serde_json::from_str::<Json>(s).ok());

        let attachments = item
            .get("attachments")
            .and_then(|v| v.as_ss().ok())
            .map(|att_list| {
                att_list
                    .iter()
                    .map(|s| s.to_string())
                    .collect()
            })
            .unwrap_or_default();

        let approval_required = item
            .get("approval_required")
            .and_then(|v| v.as_bool().ok())
            .unwrap_or(&true);

        let auto_approval_limit = item
            .get("auto_approval_limit")
            .and_then(|v| v.as_n().ok())
            .and_then(|s| s.parse::<f64>().ok());

        let notes = item
            .get("notes")
            .and_then(|v| v.as_s().ok())
            .map(|s| s.to_string());

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
            legal_name,
            vendor_code,
            vendor_category_id,
            status,
            tier,
            phone_number,
            secondary_phone,
            email_address,
            secondary_email,
            website,
            tax_id,
            registration_number,
            payment_terms,
            currency,
            credit_limit,
            primary_contact_name,
            primary_contact_title,
            primary_contact_email,
            primary_contact_phone,
            billing_address,
            shipping_address,
            certifications,
            compliance_status,
            insurance_info,
            contract_start_date,
            contract_end_date,
            last_order_date,
            total_orders,
            total_spent,
            average_rating,
            performance_notes,
            emergency_contact,
            preferred_communication,
            time_zone,
            business_hours,
            tags,
            custom_fields,
            attachments,
            approval_required: *approval_required,
            auto_approval_limit,
            notes,
            created_by,
            created_at,
            updated_at,
        });

        info!("result of from_item on vendor: {:?}", res);
        res
    }

    /// Creates DynamoDB item from Vendor instance
    ///
    /// # Arguments
    ///
    /// * `self` - borrowed instance of self
    ///
    /// # Returns
    ///
    /// HashMap representing DB item for Vendor instance
    pub fn to_item(&self) -> HashMap<String, AttributeValue> {
        let mut item = HashMap::new();

        item.insert("id".to_string(), AttributeValue::S(self.id.clone()));
        item.insert("name".to_string(), AttributeValue::S(self.name.clone()));

        if let Some(legal_name) = &self.legal_name {
            item.insert("legal_name".to_string(), AttributeValue::S(legal_name.clone()));
        }

        if let Some(code) = &self.vendor_code {
            item.insert("vendor_code".to_string(), AttributeValue::S(code.clone()));
        }

        item.insert(
            "vendor_category_id".to_string(),
            AttributeValue::S(self.vendor_category_id.clone())
        );
        item.insert("status".to_string(), AttributeValue::S(self.status.to_str().to_string()));
        item.insert("tier".to_string(), AttributeValue::S(self.tier.to_str().to_string()));
        item.insert("phone_number".to_string(), AttributeValue::S(self.phone_number.clone()));

        if let Some(secondary_phone) = &self.secondary_phone {
            item.insert("secondary_phone".to_string(), AttributeValue::S(secondary_phone.clone()));
        }

        item.insert("email_address".to_string(), AttributeValue::S(self.email_address.clone()));

        if let Some(secondary_email) = &self.secondary_email {
            item.insert("secondary_email".to_string(), AttributeValue::S(secondary_email.clone()));
        }

        if let Some(website) = &self.website {
            item.insert("website".to_string(), AttributeValue::S(website.clone()));
        }

        item.insert("tax_id".to_string(), AttributeValue::S(self.tax_id.clone()));

        if let Some(reg_number) = &self.registration_number {
            item.insert("registration_number".to_string(), AttributeValue::S(reg_number.clone()));
        }

        item.insert("payment_terms".to_string(), AttributeValue::S(self.payment_terms.clone()));
        item.insert("currency".to_string(), AttributeValue::S(self.currency.clone()));

        if let Some(credit_limit) = &self.credit_limit {
            item.insert("credit_limit".to_string(), AttributeValue::N(credit_limit.to_string()));
        }

        item.insert(
            "primary_contact_name".to_string(),
            AttributeValue::S(self.primary_contact_name.clone())
        );
        item.insert(
            "primary_contact_title".to_string(),
            AttributeValue::S(self.primary_contact_title.clone())
        );

        if let Some(contact_email) = &self.primary_contact_email {
            item.insert(
                "primary_contact_email".to_string(),
                AttributeValue::S(contact_email.clone())
            );
        }

        if let Some(contact_phone) = &self.primary_contact_phone {
            item.insert(
                "primary_contact_phone".to_string(),
                AttributeValue::S(contact_phone.clone())
            );
        }

        if let Some(billing) = &self.billing_address {
            if let Ok(billing_json) = serde_json::to_string(billing) {
                item.insert("billing_address".to_string(), AttributeValue::S(billing_json));
            }
        }

        if let Some(shipping) = &self.shipping_address {
            if let Ok(shipping_json) = serde_json::to_string(shipping) {
                item.insert("shipping_address".to_string(), AttributeValue::S(shipping_json));
            }
        }

        // Store certifications as string set
        if !self.certifications.is_empty() {
            item.insert(
                "certifications".to_string(),
                AttributeValue::Ss(self.certifications.clone())
            );
        }

        item.insert(
            "compliance_status".to_string(),
            AttributeValue::S(self.compliance_status.clone())
        );

        if let Some(insurance) = &self.insurance_info {
            if let Ok(insurance_json) = serde_json::to_string(insurance) {
                item.insert("insurance_info".to_string(), AttributeValue::S(insurance_json));
            }
        }

        if let Some(start_date) = &self.contract_start_date {
            item.insert(
                "contract_start_date".to_string(),
                AttributeValue::S(start_date.to_string())
            );
        }

        if let Some(end_date) = &self.contract_end_date {
            item.insert("contract_end_date".to_string(), AttributeValue::S(end_date.to_string()));
        }

        if let Some(last_order) = &self.last_order_date {
            item.insert("last_order_date".to_string(), AttributeValue::S(last_order.to_string()));
        }

        item.insert("total_orders".to_string(), AttributeValue::N(self.total_orders.to_string()));
        item.insert("total_spent".to_string(), AttributeValue::N(self.total_spent.to_string()));

        if let Some(rating) = &self.average_rating {
            item.insert("average_rating".to_string(), AttributeValue::N(rating.to_string()));
        }

        if let Some(perf_notes) = &self.performance_notes {
            item.insert("performance_notes".to_string(), AttributeValue::S(perf_notes.clone()));
        }

        if let Some(emergency) = &self.emergency_contact {
            if let Ok(emergency_json) = serde_json::to_string(emergency) {
                item.insert("emergency_contact".to_string(), AttributeValue::S(emergency_json));
            }
        }

        item.insert(
            "preferred_communication".to_string(),
            AttributeValue::S(self.preferred_communication.clone())
        );

        if let Some(tz) = &self.time_zone {
            item.insert("time_zone".to_string(), AttributeValue::S(tz.clone()));
        }

        if let Some(hours) = &self.business_hours {
            if let Ok(hours_json) = serde_json::to_string(hours) {
                item.insert("business_hours".to_string(), AttributeValue::S(hours_json));
            }
        }

        // Store tags as string set
        if !self.tags.is_empty() {
            item.insert("tags".to_string(), AttributeValue::Ss(self.tags.clone()));
        }

        if let Some(custom) = &self.custom_fields {
            if let Ok(custom_json) = serde_json::to_string(custom) {
                item.insert("custom_fields".to_string(), AttributeValue::S(custom_json));
            }
        }

        // Store attachments as string set
        if !self.attachments.is_empty() {
            item.insert("attachments".to_string(), AttributeValue::Ss(self.attachments.clone()));
        }

        item.insert("approval_required".to_string(), AttributeValue::Bool(self.approval_required));

        if let Some(auto_limit) = &self.auto_approval_limit {
            item.insert(
                "auto_approval_limit".to_string(),
                AttributeValue::N(auto_limit.to_string())
            );
        }

        if let Some(vendor_notes) = &self.notes {
            item.insert("notes".to_string(), AttributeValue::S(vendor_notes.clone()));
        }

        if let Some(creator) = &self.created_by {
            item.insert("created_by".to_string(), AttributeValue::S(creator.clone()));
        }

        item.insert("created_at".to_string(), AttributeValue::S(self.created_at.to_string()));
        item.insert("updated_at".to_string(), AttributeValue::S(self.updated_at.to_string()));

        item
    }

    /// Checks if the vendor is currently active and usable
    pub fn is_active(&self) -> bool {
        matches!(self.status, VendorStatus::Active)
    }

    /// Checks if the vendor contract has expired
    pub fn is_contract_expired(&self) -> bool {
        if let Some(end_date) = &self.contract_end_date { Utc::now() > *end_date } else { false }
    }

    /// Checks if the vendor can be used for new orders
    pub fn can_place_orders(&self) -> bool {
        self.is_active() &&
            !self.is_contract_expired() &&
            !matches!(
                self.status,
                VendorStatus::Suspended | VendorStatus::Blacklisted | VendorStatus::Terminated
            )
    }

    /// Updates order statistics
    pub fn record_order(&mut self, order_amount: f64) -> Result<(), AppError> {
        if order_amount < 0.0 {
            return Err(AppError::ValidationError("Order amount cannot be negative".to_string()));
        }

        self.total_orders += 1;
        self.total_spent += order_amount;
        self.last_order_date = Some(Utc::now());
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Updates the vendor's performance rating
    pub fn update_rating(&mut self, new_rating: f64) -> Result<(), AppError> {
        if new_rating < 0.0 || new_rating > 5.0 {
            return Err(AppError::ValidationError("Rating must be between 0.0 and 5.0".to_string()));
        }

        self.average_rating = Some(new_rating);
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Adds a certification to the vendor
    pub fn add_certification(&mut self, certification: String) {
        if !self.certifications.contains(&certification) {
            self.certifications.push(certification);
            self.updated_at = Utc::now();
        }
    }

    /// Removes a certification from the vendor
    pub fn remove_certification(&mut self, certification: &str) {
        if let Some(pos) = self.certifications.iter().position(|x| x == certification) {
            self.certifications.remove(pos);
            self.updated_at = Utc::now();
        }
    }

    /// Adds a tag to the vendor
    pub fn add_tag(&mut self, tag: String) {
        if !self.tags.contains(&tag) {
            self.tags.push(tag);
            self.updated_at = Utc::now();
        }
    }

    /// Removes a tag from the vendor
    pub fn remove_tag(&mut self, tag: &str) {
        if let Some(pos) = self.tags.iter().position(|x| x == tag) {
            self.tags.remove(pos);
            self.updated_at = Utc::now();
        }
    }

    /// Suspends the vendor with a reason
    pub fn suspend(&mut self, reason: Option<String>) -> Result<(), AppError> {
        if matches!(self.status, VendorStatus::Terminated | VendorStatus::Blacklisted) {
            return Err(
                AppError::ValidationError(
                    "Cannot suspend terminated or blacklisted vendor".to_string()
                )
            );
        }

        self.status = VendorStatus::Suspended;
        if let Some(reason_text) = reason {
            let current_notes = self.notes.clone().unwrap_or_default();
            self.notes = Some(format!("{}; SUSPENDED: {}", current_notes, reason_text));
        }
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Reactivates a suspended vendor
    pub fn reactivate(&mut self) -> Result<(), AppError> {
        if !matches!(self.status, VendorStatus::Suspended | VendorStatus::Inactive) {
            return Err(
                AppError::ValidationError(
                    "Only suspended or inactive vendors can be reactivated".to_string()
                )
            );
        }

        self.status = VendorStatus::Active;
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Calculates average order value
    pub fn average_order_value(&self) -> f64 {
        if self.total_orders > 0 { self.total_spent / (self.total_orders as f64) } else { 0.0 }
    }
}
