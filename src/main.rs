use dioxus::prelude::*;

mod components;
mod content;
mod pages;

use components::{Footer, Header};
use pages::{
    About, AdminArticleEdit, AdminArticleNew, AdminArticles, AdminDashboard, AdminLogin,
    AdminSettings, ArticleDetail, Articles, Contact, Home, Portfolio, PortfolioDetail, Services,
};

const CSS: Asset = asset!("/assets/main.css");

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    // Public routes with layout
    #[layout(Layout)]
    #[route("/")]
    Home {},
    #[route("/about")]
    About {},
    #[route("/services")]
    Services {},
    #[route("/portfolio")]
    Portfolio {},
    #[route("/portfolio/:slug")]
    PortfolioDetail { slug: String },
    #[route("/contact")]
    Contact {},
    #[route("/articles")]
    Articles {},
    #[route("/articles/:slug")]
    ArticleDetail { slug: String },
    #[end_layout]

    // Admin routes (no public layout)
    #[route("/admin")]
    AdminLogin {},
    #[route("/admin/dashboard")]
    AdminDashboard {},
    #[route("/admin/settings")]
    AdminSettings {},
    #[route("/admin/articles")]
    AdminArticles {},
    #[route("/admin/articles/new")]
    AdminArticleNew {},
    #[route("/admin/articles/:id")]
    AdminArticleEdit { id: String },
}

fn main() {
    // Initialize storage with defaults
    content::init_storage();
    dioxus::launch(App);
}

#[cfg(test)]
mod tests {
    use super::*;

    /// All routes in the sitemap must be defined and reachable
    #[test]
    fn test_all_sitemap_routes_are_valid() {
        // Define expected routes (our sitemap)
        let expected_routes = vec![
            "/",
            "/about",
            "/services",
            "/portfolio",
            "/contact",
            "/articles",
        ];

        // Verify each route can be parsed by the router
        for path in &expected_routes {
            let result: Result<Route, _> = path.parse();
            assert!(
                result.is_ok(),
                "Route '{}' is not defined in the router",
                path
            );
        }
    }

    /// Verify route enum has all expected variants
    #[test]
    fn test_route_variants_exist() {
        // This test ensures the Route enum has all required variants
        // If any variant is removed, this will fail to compile
        let _routes = [
            Route::Home {},
            Route::About {},
            Route::Services {},
            Route::Portfolio {},
            Route::Contact {},
            Route::Articles {},
            Route::AdminLogin {},
            Route::AdminDashboard {},
        ];
    }
}

#[component]
fn App() -> Element {
    // Handle SPA redirect from 404.html
    use_effect(|| {
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(window) = web_sys::window() {
                if let Ok(Some(storage)) = window.session_storage() {
                    if let Ok(Some(path)) = storage.get_item("spa-redirect-path") {
                        // Clear the stored path
                        let _ = storage.remove_item("spa-redirect-path");
                        // Navigate to the stored path
                        if let Ok(history) = window.history() {
                            let _ = history.replace_state_with_url(
                                &web_sys::wasm_bindgen::JsValue::NULL,
                                "",
                                Some(&path),
                            );
                            // Reload to let the router handle the new path
                            let _ = window.location().reload();
                        }
                    }
                }
            }
        }
    });

    rsx! {
        document::Link { rel: "stylesheet", href: CSS }
        document::Link {
            rel: "preconnect",
            href: "https://fonts.googleapis.com"
        }
        document::Link {
            rel: "preconnect",
            href: "https://fonts.gstatic.com",
            crossorigin: "anonymous"
        }
        document::Link {
            rel: "stylesheet",
            href: "https://fonts.googleapis.com/css2?family=Inter:wght@500;600&family=Montserrat:wght@700&family=Open+Sans:wght@400;500&display=swap"
        }
        Router::<Route> {}
    }
}

#[component]
fn Layout() -> Element {
    rsx! {
        Header {}
        main {
            Outlet::<Route> {}
        }
        Footer {}
    }
}
