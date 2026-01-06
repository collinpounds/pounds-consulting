use super::types::{ArticlesData, PortfolioData, SiteSettings};
use web_sys::window;

const SETTINGS_KEY: &str = "site_settings";
const ARTICLES_KEY: &str = "site_articles";
const AUTH_KEY: &str = "admin_auth";
const ARTICLES_VERSION_KEY: &str = "articles_version";
const CURRENT_ARTICLES_VERSION: &str = "v3"; // Increment this to force refresh

/// Load settings from localStorage, or return defaults
pub fn load_settings() -> SiteSettings {
    get_from_storage(SETTINGS_KEY).unwrap_or_default()
}

/// Save settings to localStorage
pub fn save_settings(settings: &SiteSettings) -> bool {
    set_to_storage(SETTINGS_KEY, settings)
}

/// Load articles from localStorage, or return defaults
pub fn load_articles() -> ArticlesData {
    get_from_storage(ARTICLES_KEY).unwrap_or_default()
}

/// Save articles to localStorage
pub fn save_articles(articles: &ArticlesData) -> bool {
    set_to_storage(ARTICLES_KEY, articles)
}

/// Load portfolio projects (static data, not persisted)
pub fn load_portfolio() -> PortfolioData {
    PortfolioData::default()
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
