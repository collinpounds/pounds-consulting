use super::types::{ArticlesData, PortfolioData, ServicesData, SiteSettings};
use serde::{Deserialize, Serialize};
use web_sys::wasm_bindgen::JsCast;
use web_sys::window;

const SETTINGS_KEY: &str = "site_settings";
const ARTICLES_KEY: &str = "site_articles";
const AUTH_KEY: &str = "admin_auth";
const ARTICLES_VERSION_KEY: &str = "articles_version";
const CURRENT_ARTICLES_VERSION: &str = "v3"; // Increment this to force refresh
const THEME_KEY: &str = "site_theme";

/// Theme configuration with all 8 CSS color variables
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ThemeConfig {
    pub name: String,
    pub primary: String,
    pub secondary: String,
    pub accent: String,
    pub background: String,
    pub surface: String,
    pub text_primary: String,
    pub text_secondary: String,
    pub border: String,
}

impl Default for ThemeConfig {
    fn default() -> Self {
        Self::default_gold()
    }
}

impl ThemeConfig {
    pub fn default_gold() -> Self {
        Self {
            name: "Default Gold".to_string(),
            primary: "#FAFAFA".to_string(),
            secondary: "#D4A017".to_string(),
            accent: "#E6B422".to_string(),
            background: "#1A1A1A".to_string(),
            surface: "#2A2A2A".to_string(),
            text_primary: "#FAFAFA".to_string(),
            text_secondary: "#CCCCCC".to_string(),
            border: "#3A3A3A".to_string(),
        }
    }

    pub fn metallic_dark() -> Self {
        Self {
            name: "Metallic Dark".to_string(),
            primary: "#F5F5F0".to_string(),
            secondary: "#B8860B".to_string(),
            accent: "#CD950C".to_string(),
            background: "#0D0D0D".to_string(),
            surface: "#1A1A1A".to_string(),
            text_primary: "#F5F5F0".to_string(),
            text_secondary: "#E8E8E0".to_string(),
            border: "#2A2A2A".to_string(),
        }
    }

    pub fn blue_steel() -> Self {
        Self {
            name: "Blue Steel".to_string(),
            primary: "#E8E8F0".to_string(),
            secondary: "#4A90D9".to_string(),
            accent: "#5BA0E9".to_string(),
            background: "#0A0A12".to_string(),
            surface: "#12121A".to_string(),
            text_primary: "#E8E8F0".to_string(),
            text_secondary: "#B8B8C8".to_string(),
            border: "#2A2A3A".to_string(),
        }
    }

    pub fn emerald() -> Self {
        Self {
            name: "Emerald".to_string(),
            primary: "#E8F0E8".to_string(),
            secondary: "#2E8B57".to_string(),
            accent: "#3CB371".to_string(),
            background: "#0A120A".to_string(),
            surface: "#121A12".to_string(),
            text_primary: "#E8F0E8".to_string(),
            text_secondary: "#B8C8B8".to_string(),
            border: "#2A3A2A".to_string(),
        }
    }

    pub fn crimson() -> Self {
        Self {
            name: "Crimson".to_string(),
            primary: "#F0E8E8".to_string(),
            secondary: "#DC143C".to_string(),
            accent: "#E63950".to_string(),
            background: "#120A0A".to_string(),
            surface: "#1A1212".to_string(),
            text_primary: "#F0E8E8".to_string(),
            text_secondary: "#C8B8B8".to_string(),
            border: "#3A2A2A".to_string(),
        }
    }

    pub fn monochrome() -> Self {
        Self {
            name: "Monochrome".to_string(),
            primary: "#F0F0F0".to_string(),
            secondary: "#808080".to_string(),
            accent: "#A0A0A0".to_string(),
            background: "#0A0A0A".to_string(),
            surface: "#141414".to_string(),
            text_primary: "#F0F0F0".to_string(),
            text_secondary: "#B0B0B0".to_string(),
            border: "#303030".to_string(),
        }
    }

    pub fn all_presets() -> Vec<ThemeConfig> {
        vec![
            Self::default_gold(),
            Self::metallic_dark(),
            Self::blue_steel(),
            Self::emerald(),
            Self::crimson(),
            Self::monochrome(),
        ]
    }
}

/// Apply theme to DOM by updating CSS variables on :root
pub fn apply_theme_to_dom(theme: &ThemeConfig) {
    if let Some(window) = window() {
        if let Some(doc) = window.document() {
            if let Some(root) = doc.document_element() {
                if let Ok(html) = root.dyn_into::<web_sys::HtmlElement>() {
                    let style = html.style();
                    let _ = style.set_property("--color-primary", &theme.primary);
                    let _ = style.set_property("--color-secondary", &theme.secondary);
                    let _ = style.set_property("--color-accent", &theme.accent);
                    let _ = style.set_property("--color-background", &theme.background);
                    let _ = style.set_property("--color-surface", &theme.surface);
                    let _ = style.set_property("--color-text-primary", &theme.text_primary);
                    let _ = style.set_property("--color-text-secondary", &theme.text_secondary);
                    let _ = style.set_property("--color-border", &theme.border);
                }
            }
        }
    }
}

/// Load theme from localStorage, or return default
pub fn load_theme() -> ThemeConfig {
    get_from_storage(THEME_KEY).unwrap_or_default()
}

/// Save theme to localStorage
pub fn save_theme(theme: &ThemeConfig) -> bool {
    set_to_storage(THEME_KEY, theme)
}

/// Load settings from localStorage, or return defaults
pub fn load_settings() -> SiteSettings {
    get_from_storage(SETTINGS_KEY).unwrap_or_default()
}

/// Save settings to localStorage
pub fn save_settings(settings: &SiteSettings) -> bool {
    set_to_storage(SETTINGS_KEY, settings)
}

/// Load articles - returns compile-time defaults so all visitors see them
pub fn load_articles() -> ArticlesData {
    // Articles are now compiled into the binary so all visitors can see them
    // To add/edit articles, modify ArticlesData::default() in types.rs
    ArticlesData::default()
}

/// Save articles to localStorage
pub fn save_articles(articles: &ArticlesData) -> bool {
    set_to_storage(ARTICLES_KEY, articles)
}

/// Load portfolio projects (static data, not persisted)
pub fn load_portfolio() -> PortfolioData {
    PortfolioData::default()
}

/// Load services (static data, not persisted)
pub fn load_services() -> ServicesData {
    ServicesData::default()
}

/// Check if user is authenticated
pub fn is_authenticated() -> bool {
    if let Some(window) = window() {
        if let Ok(Some(storage)) = window.local_storage() {
            return storage.get_item(AUTH_KEY).ok().flatten().is_some();
        }
    }
    false
}

/// Set authentication (login)
pub fn set_authenticated(authenticated: bool) {
    if let Some(window) = window() {
        if let Ok(Some(storage)) = window.local_storage() {
            if authenticated {
                let _ = storage.set_item(AUTH_KEY, "true");
            } else {
                let _ = storage.remove_item(AUTH_KEY);
            }
        }
    }
}

/// Verify password against stored hash
pub fn verify_password(password: &str, settings: &SiteSettings) -> bool {
    // Simple comparison - in production you'd use proper password hashing
    password == settings.admin_password_hash
}

/// Generic helper to get JSON from localStorage
fn get_from_storage<T: serde::de::DeserializeOwned>(key: &str) -> Option<T> {
    let window = window()?;
    let storage = window.local_storage().ok()??;
    let value = storage.get_item(key).ok()??;
    serde_json::from_str(&value).ok()
}

/// Generic helper to set JSON to localStorage
fn set_to_storage<T: serde::Serialize>(key: &str, value: &T) -> bool {
    if let Some(window) = window() {
        if let Ok(Some(storage)) = window.local_storage() {
            if let Ok(json) = serde_json::to_string(value) {
                return storage.set_item(key, &json).is_ok();
            }
        }
    }
    false
}

/// Initialize storage with defaults if empty or outdated
pub fn init_storage() {
    // Only set defaults if storage is empty
    if get_from_storage::<SiteSettings>(SETTINGS_KEY).is_none() {
        save_settings(&SiteSettings::default());
    }

    // Check articles version - force refresh if version changed
    let stored_version: Option<String> = get_from_storage(ARTICLES_VERSION_KEY);
    let needs_refresh = stored_version.as_deref() != Some(CURRENT_ARTICLES_VERSION);

    if needs_refresh || get_from_storage::<ArticlesData>(ARTICLES_KEY).is_none() {
        save_articles(&ArticlesData::default());
        set_to_storage(ARTICLES_VERSION_KEY, &CURRENT_ARTICLES_VERSION.to_string());
    }
}

/// Clear all site data (for reset)
#[allow(dead_code)]
pub fn clear_all_data() {
    if let Some(window) = window() {
        if let Ok(Some(storage)) = window.local_storage() {
            let _ = storage.remove_item(SETTINGS_KEY);
            let _ = storage.remove_item(ARTICLES_KEY);
            let _ = storage.remove_item(AUTH_KEY);
        }
    }
}

/// Export all data as JSON string
#[allow(dead_code)]
pub fn export_data() -> Option<String> {
    let settings = load_settings();
    let articles = load_articles();

    #[derive(serde::Serialize)]
    struct ExportData {
        settings: SiteSettings,
        articles: ArticlesData,
    }

    serde_json::to_string_pretty(&ExportData { settings, articles }).ok()
}

/// Import data from JSON string
#[allow(dead_code)]
pub fn import_data(json: &str) -> Result<(), String> {
    #[derive(serde::Deserialize)]
    struct ImportData {
        settings: SiteSettings,
        articles: ArticlesData,
    }

    let data: ImportData =
        serde_json::from_str(json).map_err(|e| format!("Invalid JSON: {}", e))?;

    save_settings(&data.settings);
    save_articles(&data.articles);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== ThemeConfig Tests ====================

    #[test]
    fn test_theme_config_default_is_gold() {
        let default = ThemeConfig::default();
        let gold = ThemeConfig::default_gold();

        assert_eq!(default, gold);
        assert_eq!(default.name, "Default Gold");
    }

    #[test]
    fn test_all_presets_have_valid_hex_colors() {
        for preset in ThemeConfig::all_presets() {
            // All color fields should be valid hex colors
            assert!(
                preset.primary.starts_with('#') && preset.primary.len() == 7,
                "Invalid primary color in {}: {}",
                preset.name,
                preset.primary
            );
            assert!(
                preset.secondary.starts_with('#') && preset.secondary.len() == 7,
                "Invalid secondary color in {}: {}",
                preset.name,
                preset.secondary
            );
            assert!(
                preset.accent.starts_with('#') && preset.accent.len() == 7,
                "Invalid accent color in {}: {}",
                preset.name,
                preset.accent
            );
            assert!(
                preset.background.starts_with('#') && preset.background.len() == 7,
                "Invalid background color in {}: {}",
                preset.name,
                preset.background
            );
            assert!(
                preset.surface.starts_with('#') && preset.surface.len() == 7,
                "Invalid surface color in {}: {}",
                preset.name,
                preset.surface
            );
            assert!(
                preset.text_primary.starts_with('#') && preset.text_primary.len() == 7,
                "Invalid text_primary color in {}: {}",
                preset.name,
                preset.text_primary
            );
            assert!(
                preset.text_secondary.starts_with('#') && preset.text_secondary.len() == 7,
                "Invalid text_secondary color in {}: {}",
                preset.name,
                preset.text_secondary
            );
            assert!(
                preset.border.starts_with('#') && preset.border.len() == 7,
                "Invalid border color in {}: {}",
                preset.name,
                preset.border
            );
        }
    }

    #[test]
    fn test_all_presets_have_unique_names() {
        let presets = ThemeConfig::all_presets();
        let names: Vec<&str> = presets.iter().map(|p| p.name.as_str()).collect();

        for (i, name) in names.iter().enumerate() {
            for (j, other) in names.iter().enumerate() {
                if i != j {
                    assert_ne!(name, other, "Duplicate preset name: {}", name);
                }
            }
        }
    }

    #[test]
    fn test_all_presets_count() {
        let presets = ThemeConfig::all_presets();
        assert_eq!(presets.len(), 6); // gold, metallic, blue, emerald, crimson, monochrome
    }

    #[test]
    fn test_theme_config_roundtrip() {
        for preset in ThemeConfig::all_presets() {
            let json = serde_json::to_string(&preset).unwrap();
            let deserialized: ThemeConfig = serde_json::from_str(&json).unwrap();
            assert_eq!(preset, deserialized, "Roundtrip failed for {}", preset.name);
        }
    }

    #[test]
    fn test_preset_constructors_return_correct_names() {
        assert_eq!(ThemeConfig::default_gold().name, "Default Gold");
        assert_eq!(ThemeConfig::metallic_dark().name, "Metallic Dark");
        assert_eq!(ThemeConfig::blue_steel().name, "Blue Steel");
        assert_eq!(ThemeConfig::emerald().name, "Emerald");
        assert_eq!(ThemeConfig::crimson().name, "Crimson");
        assert_eq!(ThemeConfig::monochrome().name, "Monochrome");
    }

    #[test]
    fn test_dark_themes_have_dark_backgrounds() {
        // All themes should have dark backgrounds (low RGB values)
        for preset in ThemeConfig::all_presets() {
            let bg = &preset.background;
            // Parse hex to check it's dark (first digit after # should be 0-2)
            let r = u8::from_str_radix(&bg[1..3], 16).unwrap();
            let g = u8::from_str_radix(&bg[3..5], 16).unwrap();
            let b = u8::from_str_radix(&bg[5..7], 16).unwrap();

            // For dark theme, RGB values should be low (< 50)
            assert!(
                r < 50 && g < 50 && b < 50,
                "Background {} in {} is not dark enough",
                bg,
                preset.name
            );
        }
    }

    #[test]
    fn test_light_text_on_dark_backgrounds() {
        // Text primary should be light for contrast
        for preset in ThemeConfig::all_presets() {
            let text = &preset.text_primary;
            let r = u8::from_str_radix(&text[1..3], 16).unwrap();
            let g = u8::from_str_radix(&text[3..5], 16).unwrap();
            let b = u8::from_str_radix(&text[5..7], 16).unwrap();

            // For light text, RGB values should be high (> 200)
            assert!(
                r > 200 && g > 200 && b > 200,
                "Text primary {} in {} is not light enough for dark backgrounds",
                text,
                preset.name
            );
        }
    }

    // ==================== Password Verification Tests ====================

    #[test]
    fn test_verify_password_correct() {
        let settings = SiteSettings::default();
        // Default password is "admin"
        assert!(verify_password("admin", &settings));
    }

    #[test]
    fn test_verify_password_incorrect() {
        let settings = SiteSettings::default();
        assert!(!verify_password("wrong", &settings));
        assert!(!verify_password("", &settings));
        assert!(!verify_password("Admin", &settings)); // Case sensitive
    }

    #[test]
    fn test_verify_password_custom() {
        let mut settings = SiteSettings::default();
        settings.admin_password_hash = "custom_password".to_string();

        assert!(verify_password("custom_password", &settings));
        assert!(!verify_password("admin", &settings));
    }
}
