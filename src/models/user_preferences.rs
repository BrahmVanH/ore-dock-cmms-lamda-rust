use std::collections::HashMap;

use async_graphql::Object;
use aws_sdk_dynamodb::types::AttributeValue;
use chrono::{ DateTime, Timelike, Utc };
use serde::{ Deserialize, Serialize };
use serde_json::Value as Json;
use tracing::info;

use crate::error::AppError;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ThemeOptions {
    Light,
    Dark,
    Auto, // Follow system preference
    HighContrast, // Accessibility theme
}

impl ThemeOptions {
    fn to_str(&self) -> &str {
        match self {
            ThemeOptions::Light => "light",
            ThemeOptions::Dark => "dark",
            ThemeOptions::Auto => "auto",
            ThemeOptions::HighContrast => "high_contrast",
        }
    }

    fn to_string(&self) -> String {
        self.to_str().to_string()
    }

    fn from_string(s: &str) -> Result<ThemeOptions, AppError> {
        match s {
            "light" => Ok(Self::Light),
            "dark" => Ok(Self::Dark),
            "auto" => Ok(Self::Auto),
            "high_contrast" => Ok(Self::HighContrast),
            _ => Err(AppError::ValidationError("Invalid theme option".to_string())),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LanguageOptions {
    English,
    Spanish,
    French,
    German,
    Portuguese,
    Chinese,
    Japanese,
}

impl LanguageOptions {
    fn to_str(&self) -> &str {
        match self {
            LanguageOptions::English => "en",
            LanguageOptions::Spanish => "es",
            LanguageOptions::French => "fr",
            LanguageOptions::German => "de",
            LanguageOptions::Portuguese => "pt",
            LanguageOptions::Chinese => "zh",
            LanguageOptions::Japanese => "ja",
        }
    }

    fn to_string(&self) -> String {
        self.to_str().to_string()
    }

    fn from_string(s: &str) -> Result<LanguageOptions, AppError> {
        match s {
            "en" => Ok(Self::English),
            "es" => Ok(Self::Spanish),
            "fr" => Ok(Self::French),
            "de" => Ok(Self::German),
            "pt" => Ok(Self::Portuguese),
            "zh" => Ok(Self::Chinese),
            "ja" => Ok(Self::Japanese),
            _ => Err(AppError::ValidationError("Invalid language option".to_string())),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TimezoneFormat {
    TwelveHour, // 12-hour format with AM/PM
    TwentyFourHour, // 24-hour format
}

impl TimezoneFormat {
    fn to_str(&self) -> &str {
        match self {
            TimezoneFormat::TwelveHour => "12h",
            TimezoneFormat::TwentyFourHour => "24h",
        }
    }

    fn to_string(&self) -> String {
        self.to_str().to_string()
    }

    fn from_string(s: &str) -> Result<TimezoneFormat, AppError> {
        match s {
            "12h" => Ok(Self::TwelveHour),
            "24h" => Ok(Self::TwentyFourHour),
            _ => Err(AppError::ValidationError("Invalid timezone format".to_string())),
        }
    }
}

/// Represents User Preferences in the system
///
/// # Fields
///
/// * `id` - Unique identifier for the user preference entry
/// * `user_id` - ID of the user these preferences belong to
/// * `language` - User's preferred language
/// * `theme` - User's preferred theme (light, dark, auto, high_contrast)
/// * `timezone` - User's timezone preference
/// * `time_format` - Time display format preference (12h/24h)
/// * `date_format` - Date display format preference
/// * `dashboard_layout` - JSON configuration for dashboard layout
/// * `email_notifications_enabled` - Whether email notifications are enabled
/// * `mobile_notifications_enabled` - Whether mobile notifications are enabled
/// * `desktop_notifications_enabled` - Whether desktop notifications are enabled
/// * `default_location_id` - Default location for the user
/// * `auto_refresh_interval` - Auto-refresh interval in seconds (0 = disabled)
/// * `accessibility_enabled` - Whether accessibility features are enabled
/// * `high_contrast_mode` - Whether high contrast mode is enabled
/// * `font_size_multiplier` - Font size multiplier (1.0 = normal, 1.2 = 120%, etc.)
/// * `sidebar_collapsed` - Whether sidebar is collapsed by default
/// * `show_tooltips` - Whether to show helpful tooltips
/// * `default_page_size` - Default number of items per page in lists
/// * `custom_shortcuts` - JSON object containing custom keyboard shortcuts
/// * `created_at` - Date and time of creation
/// * `updated_at` - Date and time of last update
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserPreferences {
    pub id: String,
    pub user_id: String,
    pub language: LanguageOptions,
    pub theme: ThemeOptions,
    pub timezone: String,
    pub time_format: TimezoneFormat,
    pub date_format: String,
    pub dashboard_layout: Json,
    pub email_notifications_enabled: bool,
    pub mobile_notifications_enabled: bool,
    pub desktop_notifications_enabled: bool,
    pub default_location_id: Option<String>,
    pub auto_refresh_interval: i32,
    pub accessibility_enabled: bool,
    pub high_contrast_mode: bool,
    pub font_size_multiplier: f64,
    pub sidebar_collapsed: bool,
    pub show_tooltips: bool,
    pub default_page_size: i32,
    pub custom_shortcuts: Option<Json>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Defines methods for UserPreferences
impl UserPreferences {
    /// Creates new UserPreferences instance
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier
    /// * `user_id` - User ID these preferences belong to
    /// * `language` - Language preference as string
    /// * `theme` - Theme preference as string
    /// * `timezone` - Timezone string
    /// * `time_format` - Time format as string
    /// * `date_format` - Date format string
    /// * `dashboard_layout` - Dashboard layout JSON
    /// * `email_notifications_enabled` - Email notifications enabled
    /// * `mobile_notifications_enabled` - Mobile notifications enabled
    /// * `desktop_notifications_enabled` - Desktop notifications enabled
    /// * `default_location_id` - Optional default location ID
    /// * `auto_refresh_interval` - Auto refresh interval in seconds
    /// * `accessibility_enabled` - Accessibility features enabled
    /// * `high_contrast_mode` - High contrast mode enabled
    /// * `font_size_multiplier` - Font size multiplier
    /// * `sidebar_collapsed` - Sidebar collapsed by default
    /// * `show_tooltips` - Show tooltips
    /// * `default_page_size` - Default page size
    /// * `custom_shortcuts` - Optional custom shortcuts JSON
    ///
    /// # Returns
    ///
    /// New UserPreferences instance
    pub fn new(
        id: String,
        user_id: String,
        language: String,
        theme: String,
        timezone: String,
        time_format: String,
        date_format: String,
        dashboard_layout: Json,
        email_notifications_enabled: bool,
        mobile_notifications_enabled: bool,
        desktop_notifications_enabled: bool,
        default_location_id: Option<String>,
        auto_refresh_interval: i32,
        accessibility_enabled: bool,
        high_contrast_mode: bool,
        font_size_multiplier: f64,
        sidebar_collapsed: bool,
        show_tooltips: bool,
        default_page_size: i32,
        custom_shortcuts: Option<Json>
    ) -> Result<Self, AppError> {
        let now = Utc::now();

        if user_id.trim().is_empty() {
            return Err(AppError::ValidationError("User ID cannot be empty".to_string()));
        }

        let language_enum = LanguageOptions::from_string(&language)?;
        let theme_enum = ThemeOptions::from_string(&theme)?;
        let time_format_enum = TimezoneFormat::from_string(&time_format)?;

        // Validate font size multiplier
        if font_size_multiplier < 0.5 || font_size_multiplier > 3.0 {
            return Err(
                AppError::ValidationError(
                    "Font size multiplier must be between 0.5 and 3.0".to_string()
                )
            );
        }

        // Validate page size
        if default_page_size < 5 || default_page_size > 500 {
            return Err(
                AppError::ValidationError("Default page size must be between 5 and 500".to_string())
            );
        }

        // Validate auto refresh interval
        if auto_refresh_interval < 0 {
            return Err(
                AppError::ValidationError("Auto refresh interval cannot be negative".to_string())
            );
        }

        Ok(Self {
            id,
            user_id,
            language: language_enum,
            theme: theme_enum,
            timezone,
            time_format: time_format_enum,
            date_format,
            dashboard_layout,
            email_notifications_enabled,
            mobile_notifications_enabled,
            desktop_notifications_enabled,
            default_location_id,
            auto_refresh_interval,
            accessibility_enabled,
            high_contrast_mode,
            font_size_multiplier,
            sidebar_collapsed,
            show_tooltips,
            default_page_size,
            custom_shortcuts,
            created_at: now,
            updated_at: now,
        })
    }

    /// Creates UserPreferences instance from DynamoDB item
    ///
    /// # Arguments
    ///
    /// * `item` - The dynamo db item
    ///
    /// # Returns
    ///
    /// 'Some' UserPreferences if item fields match, 'None' otherwise
    pub fn from_item(item: &HashMap<String, AttributeValue>) -> Option<Self> {
        info!("calling from_item with: {:?}", &item);

        let id = item.get("id")?.as_s().ok()?.to_string();
        let user_id = item.get("user_id")?.as_s().ok()?.to_string();

        let language_str = item.get("language")?.as_s().ok()?;
        let language = LanguageOptions::from_string(&language_str)
            .map_err(|e| e)
            .ok()?;

        let theme_str = item.get("theme")?.as_s().ok()?;
        let theme = ThemeOptions::from_string(&theme_str)
            .map_err(|e| e)
            .ok()?;

        let timezone = item.get("timezone")?.as_s().ok()?.to_string();

        let time_format_str = item.get("time_format")?.as_s().ok()?;
        let time_format = TimezoneFormat::from_string(&time_format_str)
            .map_err(|e| e)
            .ok()?;

        let date_format = item.get("date_format")?.as_s().ok()?.to_string();

        let dashboard_layout = item
            .get("dashboard_layout")
            .and_then(|v| v.as_s().ok())
            .and_then(|s| serde_json::from_str::<Json>(s).ok())
            .unwrap_or(Json::Object(serde_json::Map::new()));

        let email_notifications_enabled = item
            .get("email_notifications_enabled")
            .and_then(|v| v.as_bool().ok())
            .unwrap_or(&true);

        let mobile_notifications_enabled = item
            .get("mobile_notifications_enabled")
            .and_then(|v| v.as_bool().ok())
            .unwrap_or(&true);

        let desktop_notifications_enabled = item
            .get("desktop_notifications_enabled")
            .and_then(|v| v.as_bool().ok())
            .unwrap_or(&true);

        let default_location_id = item
            .get("default_location_id")
            .and_then(|v| v.as_s().ok())
            .map(|s| s.to_string());

        let auto_refresh_interval = item
            .get("auto_refresh_interval")
            .and_then(|v| v.as_n().ok())
            .and_then(|s| s.parse::<i32>().ok())
            .unwrap_or(30);

        let accessibility_enabled = item
            .get("accessibility_enabled")
            .and_then(|v| v.as_bool().ok())
            .unwrap_or(&false);

        let high_contrast_mode = item
            .get("high_contrast_mode")
            .and_then(|v| v.as_bool().ok())
            .unwrap_or(&false);

        let font_size_multiplier = item
            .get("font_size_multiplier")
            .and_then(|v| v.as_n().ok())
            .and_then(|s| s.parse::<f64>().ok())
            .unwrap_or(1.0);

        let sidebar_collapsed = item
            .get("sidebar_collapsed")
            .and_then(|v| v.as_bool().ok())
            .unwrap_or(&false);

        let show_tooltips = item
            .get("show_tooltips")
            .and_then(|v| v.as_bool().ok())
            .unwrap_or(&true);

        let default_page_size = item
            .get("default_page_size")
            .and_then(|v| v.as_n().ok())
            .and_then(|s| s.parse::<i32>().ok())
            .unwrap_or(25);

        let custom_shortcuts = item
            .get("custom_shortcuts")
            .and_then(|v| v.as_s().ok())
            .and_then(|s| serde_json::from_str::<Json>(s).ok());

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
            user_id,
            language,
            theme,
            timezone,
            time_format,
            date_format,
            dashboard_layout,
            email_notifications_enabled,
            mobile_notifications_enabled,
            desktop_notifications_enabled,
            default_location_id,
            auto_refresh_interval,
            accessibility_enabled,
            high_contrast_mode,
            font_size_multiplier,
            sidebar_collapsed,
            show_tooltips,
            default_page_size,
            custom_shortcuts,
            created_at,
            updated_at,
        });

        info!("result of from_item on user_preferences: {:?}", res);
        res
    }

    /// Creates DynamoDB item from UserPreferences instance
    ///
    /// # Arguments
    ///
    /// * `self` - borrowed instance of self
    ///
    /// # Returns
    ///
    /// HashMap representing DB item for UserPreferences instance
    pub fn to_item(&self) -> HashMap<String, AttributeValue> {
        let mut item = HashMap::new();

        item.insert("id".to_string(), AttributeValue::S(self.id.clone()));
        item.insert("user_id".to_string(), AttributeValue::S(self.user_id.clone()));
        item.insert("language".to_string(), AttributeValue::S(self.language.to_str().to_string()));
        item.insert("theme".to_string(), AttributeValue::S(self.theme.to_str().to_string()));
        item.insert("timezone".to_string(), AttributeValue::S(self.timezone.clone()));
        item.insert(
            "time_format".to_string(),
            AttributeValue::S(self.time_format.to_str().to_string())
        );
        item.insert("date_format".to_string(), AttributeValue::S(self.date_format.clone()));

        if let Ok(dashboard_json) = serde_json::to_string(&self.dashboard_layout) {
            item.insert("dashboard_layout".to_string(), AttributeValue::S(dashboard_json));
        }

        item.insert(
            "email_notifications_enabled".to_string(),
            AttributeValue::Bool(self.email_notifications_enabled)
        );
        item.insert(
            "mobile_notifications_enabled".to_string(),
            AttributeValue::Bool(self.mobile_notifications_enabled)
        );
        item.insert(
            "desktop_notifications_enabled".to_string(),
            AttributeValue::Bool(self.desktop_notifications_enabled)
        );

        if let Some(location_id) = &self.default_location_id {
            item.insert("default_location_id".to_string(), AttributeValue::S(location_id.clone()));
        }

        item.insert(
            "auto_refresh_interval".to_string(),
            AttributeValue::N(self.auto_refresh_interval.to_string())
        );
        item.insert(
            "accessibility_enabled".to_string(),
            AttributeValue::Bool(self.accessibility_enabled)
        );
        item.insert(
            "high_contrast_mode".to_string(),
            AttributeValue::Bool(self.high_contrast_mode)
        );
        item.insert(
            "font_size_multiplier".to_string(),
            AttributeValue::N(self.font_size_multiplier.to_string())
        );
        item.insert("sidebar_collapsed".to_string(), AttributeValue::Bool(self.sidebar_collapsed));
        item.insert("show_tooltips".to_string(), AttributeValue::Bool(self.show_tooltips));
        item.insert(
            "default_page_size".to_string(),
            AttributeValue::N(self.default_page_size.to_string())
        );

        if let Some(shortcuts) = &self.custom_shortcuts {
            if let Ok(shortcuts_json) = serde_json::to_string(shortcuts) {
                item.insert("custom_shortcuts".to_string(), AttributeValue::S(shortcuts_json));
            }
        }

        item.insert("created_at".to_string(), AttributeValue::S(self.created_at.to_string()));
        item.insert("updated_at".to_string(), AttributeValue::S(self.updated_at.to_string()));

        item
    }

    /// Creates default preferences for a new user
    pub fn create_default(user_id: String) -> Result<Self, AppError> {
        let default_dashboard =
            serde_json::json!({
            "widgets": [
                {"type": "asset_summary", "position": {"x": 0, "y": 0, "w": 6, "h": 4}},
                {"type": "maintenance_schedule", "position": {"x": 6, "y": 0, "w": 6, "h": 4}},
                {"type": "recent_notifications", "position": {"x": 0, "y": 4, "w": 12, "h": 3}}
            ],
            "layout": "grid"
        });

        Self::new(
            format!("pref_{}", user_id),
            user_id,
            "en".to_string(),
            "auto".to_string(),
            "UTC".to_string(),
            "12h".to_string(),
            "MM/DD/YYYY".to_string(),
            default_dashboard,
            true,
            true,
            true,
            None,
            30,
            false,
            false,
            1.0,
            false,
            true,
            25,
            None
        )
    }

    /// Updates specific preference fields
    pub fn update_preferences(
        &mut self,
        updates: HashMap<String, serde_json::Value>
    ) -> Result<(), AppError> {
        for (key, value) in updates {
            match key.as_str() {
                "language" => {
                    if let Some(lang_str) = value.as_str() {
                        self.language = LanguageOptions::from_string(lang_str)?;
                    }
                }
                "theme" => {
                    if let Some(theme_str) = value.as_str() {
                        self.theme = ThemeOptions::from_string(theme_str)?;
                    }
                }
                "timezone" => {
                    if let Some(tz_str) = value.as_str() {
                        self.timezone = tz_str.to_string();
                    }
                }
                "dashboard_layout" => {
                    self.dashboard_layout = value;
                }
                "auto_refresh_interval" => {
                    if let Some(interval) = value.as_i64() {
                        if interval >= 0 {
                            self.auto_refresh_interval = interval as i32;
                        }
                    }
                }
                "font_size_multiplier" => {
                    if let Some(multiplier) = value.as_f64() {
                        if multiplier >= 0.5 && multiplier <= 3.0 {
                            self.font_size_multiplier = multiplier;
                        }
                    }
                }
                "default_page_size" => {
                    if let Some(size) = value.as_i64() {
                        if size >= 5 && size <= 500 {
                            self.default_page_size = size as i32;
                        }
                    }
                }
                _ => {} // Ignore unknown fields
            }
        }
        self.updated_at = Utc::now();
        Ok(())
    }
}

#[Object]
impl UserPreferences {
    async fn id(&self) -> &str {
        &self.id
    }

    async fn user_id(&self) -> &str {
        &self.user_id
    }

    async fn language(&self) -> &str {
        self.language.to_str()
    }

    async fn theme(&self) -> &str {
        self.theme.to_str()
    }

    async fn timezone(&self) -> &str {
        &self.timezone
    }

    async fn time_format(&self) -> &str {
        self.time_format.to_str()
    }

    async fn date_format(&self) -> &str {
        &self.date_format
    }

    async fn dashboard_layout(&self) -> String {
        serde_json::to_string(&self.dashboard_layout).unwrap_or_default()
    }

    async fn email_notifications_enabled(&self) -> bool {
        self.email_notifications_enabled
    }

    async fn mobile_notifications_enabled(&self) -> bool {
        self.mobile_notifications_enabled
    }

    async fn desktop_notifications_enabled(&self) -> bool {
        self.desktop_notifications_enabled
    }

    async fn default_location_id(&self) -> Option<&str> {
        self.default_location_id.as_deref()
    }

    async fn auto_refresh_interval(&self) -> i32 {
        self.auto_refresh_interval
    }

    async fn accessibility_enabled(&self) -> bool {
        self.accessibility_enabled
    }

    async fn high_contrast_mode(&self) -> bool {
        self.high_contrast_mode
    }

    async fn font_size_multiplier(&self) -> f64 {
        self.font_size_multiplier
    }

    async fn sidebar_collapsed(&self) -> bool {
        self.sidebar_collapsed
    }

    async fn show_tooltips(&self) -> bool {
        self.show_tooltips
    }

    async fn default_page_size(&self) -> i32 {
        self.default_page_size
    }

    async fn custom_shortcuts(&self) -> Option<String> {
        self.custom_shortcuts.as_ref().and_then(|s| serde_json::to_string(s).ok())
    }

    async fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }

    async fn updated_at(&self) -> &DateTime<Utc> {
        &self.updated_at
    }

    async fn effective_theme(&self) -> &str {
        // If theme is auto, could determine based on system time or other logic
        match self.theme {
            ThemeOptions::Auto => {
                // Simple logic: dark theme between 6 PM and 6 AM
                let hour12 = Utc::now().hour12();
                let mut hour = hour12.1;

                if hour12.0 == true {
                    hour += 12;
                }
                if hour >= 18 || hour < 6 {
                    return "dark";
                } else {
                    return "light";
                }
            }
            _ => self.theme.to_str(),
        }
    }

    async fn is_auto_refresh_enabled(&self) -> bool {
        self.auto_refresh_interval > 0
    }
}
