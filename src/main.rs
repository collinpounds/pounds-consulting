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
    // Apply saved theme on startup
    content::apply_theme_to_dom(&content::load_theme());
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

    // ==================== Dynamic Route Tests ====================

    #[test]
    fn test_portfolio_detail_route_parsing() {
        let route: Result<Route, _> = "/portfolio/paytient".parse();
        assert!(route.is_ok());

        if let Ok(Route::PortfolioDetail { slug }) = route {
            assert_eq!(slug, "paytient");
        } else {
            panic!("Expected PortfolioDetail route");
        }
    }

    #[test]
    fn test_portfolio_detail_with_hyphenated_slug() {
        let route: Result<Route, _> = "/portfolio/club-car-wash".parse();
        assert!(route.is_ok());

        if let Ok(Route::PortfolioDetail { slug }) = route {
            assert_eq!(slug, "club-car-wash");
        } else {
            panic!("Expected PortfolioDetail route");
        }
    }

    #[test]
    fn test_article_detail_route_parsing() {
        let route: Result<Route, _> = "/articles/do-you-need-custom-website".parse();
        assert!(route.is_ok());

        if let Ok(Route::ArticleDetail { slug }) = route {
            assert_eq!(slug, "do-you-need-custom-website");
        } else {
            panic!("Expected ArticleDetail route");
        }
    }

    // ==================== Admin Route Tests ====================

    #[test]
    fn test_admin_routes_are_valid() {
        let admin_routes = vec![
            "/admin",
            "/admin/dashboard",
            "/admin/settings",
            "/admin/articles",
            "/admin/articles/new",
        ];

        for path in &admin_routes {
            let result: Result<Route, _> = path.parse();
            assert!(
                result.is_ok(),
                "Admin route '{}' is not defined in the router",
                path
            );
        }
    }

    #[test]
    fn test_admin_article_edit_route() {
        let route: Result<Route, _> = "/admin/articles/test-article-id".parse();
        assert!(route.is_ok());

        if let Ok(Route::AdminArticleEdit { id }) = route {
            assert_eq!(id, "test-article-id");
        } else {
            panic!("Expected AdminArticleEdit route");
        }
    }

    #[test]
    fn test_all_admin_route_variants_exist() {
        // Compile-time check that admin routes exist
        let _routes = [
            Route::AdminLogin {},
            Route::AdminDashboard {},
            Route::AdminSettings {},
            Route::AdminArticles {},
            Route::AdminArticleNew {},
            Route::AdminArticleEdit {
                id: "test".to_string(),
            },
        ];
    }

    // ==================== Route Negative Tests ====================

    #[test]
    fn test_invalid_routes() {
        let invalid_routes = vec![
            "/nonexistent",
            "/admin/invalid",
            "/portfolio/detail/extra", // Too many segments
        ];

        for path in &invalid_routes {
            let result: Result<Route, _> = path.parse();
            assert!(result.is_err(), "Route '{}' should not be valid", path);
        }
    }

    // ==================== Route Display/ToString Tests ====================

    #[test]
    fn test_route_display() {
        // Routes should display as their path
        assert_eq!(Route::Home {}.to_string(), "/");
        assert_eq!(Route::About {}.to_string(), "/about");
        assert_eq!(Route::Services {}.to_string(), "/services");
        assert_eq!(Route::Portfolio {}.to_string(), "/portfolio");
        assert_eq!(Route::Contact {}.to_string(), "/contact");
        assert_eq!(Route::Articles {}.to_string(), "/articles");
    }

    #[test]
    fn test_dynamic_route_display() {
        let portfolio = Route::PortfolioDetail {
            slug: "test-project".to_string(),
        };
        assert_eq!(portfolio.to_string(), "/portfolio/test-project");

        let article = Route::ArticleDetail {
            slug: "my-article".to_string(),
        };
        assert_eq!(article.to_string(), "/articles/my-article");
    }

    #[test]
    fn test_admin_route_display() {
        assert_eq!(Route::AdminLogin {}.to_string(), "/admin");
        assert_eq!(Route::AdminDashboard {}.to_string(), "/admin/dashboard");
        assert_eq!(Route::AdminSettings {}.to_string(), "/admin/settings");
        assert_eq!(Route::AdminArticles {}.to_string(), "/admin/articles");
        assert_eq!(Route::AdminArticleNew {}.to_string(), "/admin/articles/new");

        let edit = Route::AdminArticleEdit {
            id: "abc123".to_string(),
        };
        assert_eq!(edit.to_string(), "/admin/articles/abc123");
    }
}

#[component]
fn App() -> Element {
    // Handle SPA redirect from 404.html before router mounts
    // This runs once on initial load
    #[cfg(target_arch = "wasm32")]
    {
        use_hook(|| {
            if let Some(window) = web_sys::window() {
                if let Ok(Some(storage)) = window.session_storage() {
                    if let Ok(Some(path)) = storage.get_item("spa-redirect-path") {
                        // Clear the stored path immediately
                        let _ = storage.remove_item("spa-redirect-path");
                        // Update the browser URL without reload using history API
                        if let Ok(history) = window.history() {
                            let _ = history.replace_state_with_url(
                                &web_sys::wasm_bindgen::JsValue::NULL,
                                "",
                                Some(&path),
                            );
                        }
                    }
                }
            }
        });
    }

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
