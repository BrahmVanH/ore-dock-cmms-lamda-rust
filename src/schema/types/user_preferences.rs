use chrono::Timelike;

use crate::models::{
    prelude::*,
    user_preferences::{ LanguageOptions, ThemeOptions, TimezoneFormat, UserPreferences },
};

#[Object]
impl UserPreferences {
    async fn id(&self) -> &str {
        &self.id
    }

    async fn user_id(&self) -> &str {
        &self.user_id
    }

    async fn language(&self) -> LanguageOptions {
        self.language
    }

    async fn theme(&self) -> ThemeOptions {
        self.theme
    }

    async fn timezone(&self) -> &str {
        &self.timezone
    }

    async fn time_format(&self) -> TimezoneFormat {
        self.time_format
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
