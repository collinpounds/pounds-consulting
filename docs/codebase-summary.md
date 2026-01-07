# Pounds Consulting Codebase Summary

> Auto-generated documentation for LLM review. Run `./generate-docs.sh` to regenerate.

This document contains all core Rust source code from the Pounds Consulting website.

---

## Table of Contents

- [Main Entry Point](#main-entry-point)
- [Components](#components)
- [Pages](#pages)
- [Content/Data Layer](#contentdata-layer)

---

## Main Entry Point

**File:** `src/main.rs`

```rust
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
```

---

# Components

## Components Module

**File:** `src/components/mod.rs`

```rust
mod cta_section;
mod footer;
mod header;
mod service_card;
mod theme_customizer;

pub use cta_section::CtaSection;
pub use footer::Footer;
pub use header::Header;
pub use service_card::ServiceCard;
pub use theme_customizer::{ThemeCustomizer, ThemeToggleButton};
```

---

## Header Component

**File:** `src/components/header.rs`

```rust
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Header() -> Element {
    let mut mobile_menu_open = use_signal(|| false);
    let current_route = use_route::<Route>();

    // Helper to determine if a route is active
    let is_active = |route: &Route| -> bool {
        std::mem::discriminant(&current_route) == std::mem::discriminant(route)
    };

    let home_class = if is_active(&Route::Home {}) {
        "nav-link active"
    } else {
        "nav-link"
    };
    let about_class = if is_active(&Route::About {}) {
        "nav-link active"
    } else {
        "nav-link"
    };
    let services_class = if is_active(&Route::Services {}) {
        "nav-link active"
    } else {
        "nav-link"
    };
    let portfolio_class = if is_active(&Route::Portfolio {}) {
        "nav-link active"
    } else {
        "nav-link"
    };
    let articles_class = if is_active(&Route::Articles {}) {
        "nav-link active"
    } else {
        "nav-link"
    };
    let contact_class = if is_active(&Route::Contact {}) {
        "nav-link nav-cta active"
    } else {
        "nav-link nav-cta"
    };

    rsx! {
        header { class: "header",
            div { class: "header-container",
                Link { to: Route::Home {}, class: "footer-logo",
                    span { class: "logo-text", "POUNDS" }
                    span { class: "logo-accent", "/ CONSULTING" }
                }

                nav { class: "nav-desktop",
                    Link { to: Route::Home {}, class: "{home_class}", "Home" }
                    Link { to: Route::About {}, class: "{about_class}", "About" }
                    Link { to: Route::Services {}, class: "{services_class}", "Services" }
                    Link { to: Route::Portfolio {}, class: "{portfolio_class}", "Portfolio" }
                    Link { to: Route::Articles {}, class: "{articles_class}", "Articles" }
                    Link { to: Route::Contact {}, class: "{contact_class}", "Contact" }
                }

                button {
                    class: "mobile-menu-toggle",
                    "aria-label": "Toggle navigation menu",
                    "aria-expanded": "{mobile_menu_open()}",
                    onclick: move |_| mobile_menu_open.set(!mobile_menu_open()),
                    span { class: "hamburger-line" }
                    span { class: "hamburger-line" }
                    span { class: "hamburger-line" }
                }
            }

            if mobile_menu_open() {
                nav { class: "nav-mobile",
                    Link {
                        to: Route::Home {},
                        class: "{home_class}",
                        onclick: move |_| mobile_menu_open.set(false),
                        "Home"
                    }
                    Link {
                        to: Route::About {},
                        class: "{about_class}",
                        onclick: move |_| mobile_menu_open.set(false),
                        "About"
                    }
                    Link {
                        to: Route::Services {},
                        class: "{services_class}",
                        onclick: move |_| mobile_menu_open.set(false),
                        "Services"
                    }
                    Link {
                        to: Route::Portfolio {},
                        class: "{portfolio_class}",
                        onclick: move |_| mobile_menu_open.set(false),
                        "Portfolio"
                    }
                    Link {
                        to: Route::Articles {},
                        class: "{articles_class}",
                        onclick: move |_| mobile_menu_open.set(false),
                        "Articles"
                    }
                    Link {
                        to: Route::Contact {},
                        class: "{contact_class}",
                        onclick: move |_| mobile_menu_open.set(false),
                        "Contact"
                    }
                }
            }
        }
    }
}
```

---

## Footer Component

**File:** `src/components/footer.rs`

```rust
use crate::components::{ThemeCustomizer, ThemeToggleButton};
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Footer() -> Element {
    let theme_panel_open = use_signal(|| false);

    rsx! {
        // Theme customizer panel (renders when open)
        ThemeCustomizer { is_open: theme_panel_open }

        footer { class: "footer",
            div { class: "footer-container",
                div { class: "footer-brand",
                    Link { to: Route::Home {}, class: "footer-logo",
                        span { class: "logo-text", "POUNDS" }
                        span { class: "logo-accent", "CONSULTING" }
                    }
                    p { class: "footer-tagline",
                        "Technical solutions for growing businesses."
                    }
                }

                div { class: "footer-nav",
                    h2 { class: "footer-heading", "Navigation" }
                    Link { to: Route::Home {}, class: "footer-link", "Home" }
                    Link { to: Route::About {}, class: "footer-link", "About" }
                    Link { to: Route::Services {}, class: "footer-link", "Services" }
                    Link { to: Route::Portfolio {}, class: "footer-link", "Portfolio" }
                    Link { to: Route::Articles {}, class: "footer-link", "Articles" }
                    Link { to: Route::Contact {}, class: "footer-link", "Contact" }
                }

                div { class: "footer-contact",
                    h2 { class: "footer-heading", "Contact" }
                    a {
                        href: "mailto:collin@poundsconsulting.net",
                        class: "footer-link",
                        "collin@poundsconsulting.net"
                    }
                    p { class: "footer-location", "Kansas City, Missouri" }
                }
            }

            div { class: "footer-bottom",
                div { class: "footer-bottom-links",
                    a {
                        href: "https://github.com/collinpounds",
                        class: "footer-github-link",
                        target: "_blank",
                        rel: "noopener noreferrer",
                        svg {
                            class: "github-icon",
                            xmlns: "http://www.w3.org/2000/svg",
                            width: "20",
                            height: "20",
                            view_box: "0 0 24 24",
                            fill: "currentColor",
                            path {
                                d: "M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"
                            }
                        }
                        "github.com/collinpounds"
                    }
                    ThemeToggleButton { is_open: theme_panel_open }
                }
                p { "© 2026 Pounds Consulting LLC. All rights reserved." }
            }
        }
    }
}
```

---

## CTA Section Component

**File:** `src/components/cta_section.rs`

```rust
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn CtaSection(
    title: String,
    description: String,
    button_text: String,
    #[props(default = false)] use_calendar_link: bool,
) -> Element {
    rsx! {
        section { class: "cta-section",
            div { class: "cta-container",
                h2 { class: "cta-title", "{title}" }
                p { class: "cta-description", "{description}" }
                if use_calendar_link {
                    a {
                        href: "https://calendar.app.google/LNasBbmDr8LXNEuu5",
                        target: "_blank",
                        rel: "noopener noreferrer",
                        class: "btn btn-primary",
                        "{button_text}"
                    }
                } else {
                    Link { to: Route::Contact {}, class: "btn btn-primary",
                        "{button_text}"
                    }
                }
            }
        }
    }
}
```

---

## Service Card Component

**File:** `src/components/service_card.rs`

```rust
use dioxus::prelude::*;

#[component]
pub fn ServiceCard(title: String, description: String, icon: String) -> Element {
    rsx! {
        div { class: "service-card",
            div { class: "service-icon", "{icon}" }
            h3 { class: "service-title", "{title}" }
            p { class: "service-description", "{description}" }
        }
    }
}
```

---

## Theme Customizer Component

**File:** `src/components/theme_customizer.rs`

```rust
use crate::content::{apply_theme_to_dom, load_theme, save_theme, ThemeConfig};
use dioxus::prelude::*;

#[component]
pub fn ThemeCustomizer(is_open: Signal<bool>) -> Element {
    let mut current_theme = use_signal(load_theme);

    // Apply theme to DOM whenever it changes
    use_effect(move || {
        apply_theme_to_dom(&current_theme());
    });

    let mut apply_preset = move |preset: ThemeConfig| {
        current_theme.set(preset.clone());
        save_theme(&preset);
        apply_theme_to_dom(&preset);
    };

    let mut update_color = move |field: &'static str, value: String| {
        current_theme.with_mut(|theme| {
            match field {
                "primary" => theme.primary = value,
                "secondary" => theme.secondary = value,
                "accent" => theme.accent = value,
                "background" => theme.background = value,
                "surface" => theme.surface = value,
                "text_primary" => theme.text_primary = value,
                "text_secondary" => theme.text_secondary = value,
                "border" => theme.border = value,
                _ => {}
            }
            theme.name = "Custom".to_string();
        });
        save_theme(&current_theme());
        apply_theme_to_dom(&current_theme());
    };

    let reset_to_default = move |_| {
        let default = ThemeConfig::default_gold();
        current_theme.set(default.clone());
        save_theme(&default);
        apply_theme_to_dom(&default);
    };

    let close_panel = move |_| {
        is_open.set(false);
    };

    if !is_open() {
        return rsx! {};
    }

    rsx! {
        // Backdrop
        div {
            class: "theme-customizer-backdrop",
            onclick: close_panel
        }

        // Sidebar panel
        div { class: "theme-customizer-panel",
            div { class: "theme-customizer-header",
                h3 { "Customize Theme" }
                button {
                    class: "theme-customizer-close",
                    onclick: close_panel,
                    "×"
                }
            }

            div { class: "theme-customizer-content",
                // Preset themes section
                div { class: "theme-customizer-section",
                    h4 { "Presets" }
                    div { class: "theme-presets-grid",
                        for preset in ThemeConfig::all_presets() {
                            button {
                                key: "{preset.name}",
                                class: if current_theme().name == preset.name { "theme-preset-btn active" } else { "theme-preset-btn" },
                                onclick: {
                                    let preset = preset.clone();
                                    move |_| apply_preset(preset.clone())
                                },
                                div {
                                    class: "theme-preset-swatch",
                                    style: "background: linear-gradient(135deg, {preset.background} 50%, {preset.secondary} 50%);"
                                }
                                span { "{preset.name}" }
                            }
                        }
                    }
                }

                // Custom colors section
                div { class: "theme-customizer-section",
                    h4 { "Custom Colors" }
                    div { class: "theme-color-inputs",
                        ColorInput {
                            label: "Primary",
                            value: current_theme().primary.clone(),
                            on_change: move |v| update_color("primary", v)
                        }
                        ColorInput {
                            label: "Accent (Gold)",
                            value: current_theme().secondary.clone(),
                            on_change: move |v| update_color("secondary", v)
                        }
                        ColorInput {
                            label: "Accent Hover",
                            value: current_theme().accent.clone(),
                            on_change: move |v| update_color("accent", v)
                        }
                        ColorInput {
                            label: "Background",
                            value: current_theme().background.clone(),
                            on_change: move |v| update_color("background", v)
                        }
                        ColorInput {
                            label: "Surface",
                            value: current_theme().surface.clone(),
                            on_change: move |v| update_color("surface", v)
                        }
                        ColorInput {
                            label: "Text Primary",
                            value: current_theme().text_primary.clone(),
                            on_change: move |v| update_color("text_primary", v)
                        }
                        ColorInput {
                            label: "Text Secondary",
                            value: current_theme().text_secondary.clone(),
                            on_change: move |v| update_color("text_secondary", v)
                        }
                        ColorInput {
                            label: "Border",
                            value: current_theme().border.clone(),
                            on_change: move |v| update_color("border", v)
                        }
                    }
                }

                // Actions
                div { class: "theme-customizer-actions",
                    button {
                        class: "btn btn-secondary",
                        onclick: reset_to_default,
                        "Reset to Default"
                    }
                }

                // Current theme indicator
                div { class: "theme-customizer-current",
                    "Current: {current_theme().name}"
                }
            }
        }
    }
}

#[component]
fn ColorInput(label: String, value: String, on_change: EventHandler<String>) -> Element {
    rsx! {
        div { class: "theme-color-input",
            label { "{label}" }
            div { class: "theme-color-input-row",
                input {
                    r#type: "color",
                    value: "{value}",
                    oninput: move |evt| on_change.call(evt.value())
                }
                input {
                    r#type: "text",
                    value: "{value}",
                    oninput: move |evt| on_change.call(evt.value()),
                    maxlength: "7",
                    placeholder: "#000000"
                }
            }
        }
    }
}

/// Toggle button component for the footer
#[component]
pub fn ThemeToggleButton(is_open: Signal<bool>) -> Element {
    rsx! {
        button {
            class: "theme-toggle-btn",
            title: "Customize Theme",
            onclick: move |_| is_open.set(!is_open()),
            svg {
                xmlns: "http://www.w3.org/2000/svg",
                width: "20",
                height: "20",
                view_box: "0 0 24 24",
                fill: "currentColor",
                path {
                    d: "M12 22C6.49 22 2 17.51 2 12S6.49 2 12 2s10 4.04 10 9c0 3.31-2.69 6-6 6h-1.77c-.28 0-.5.22-.5.5 0 .12.05.23.13.33.41.47.64 1.06.64 1.67A2.5 2.5 0 0 1 12 22zm0-18c-4.41 0-8 3.59-8 8s3.59 8 8 8c.28 0 .5-.22.5-.5a.54.54 0 0 0-.14-.35c-.41-.46-.63-1.05-.63-1.65a2.5 2.5 0 0 1 2.5-2.5H16c2.21 0 4-1.79 4-4 0-3.86-3.59-7-8-7z"
                }
                circle { cx: "6.5", cy: "11.5", r: "1.5" }
                circle { cx: "9.5", cy: "7.5", r: "1.5" }
                circle { cx: "14.5", cy: "7.5", r: "1.5" }
                circle { cx: "17.5", cy: "11.5", r: "1.5" }
            }
        }
    }
}
```

---

# Pages

## Pages Module

**File:** `src/pages/mod.rs`

```rust
mod about;
pub mod admin;
mod article_detail;
mod articles;
mod contact;
mod home;
mod portfolio;
mod portfolio_detail;
mod services;

pub use about::About;
pub use admin::*;
pub use article_detail::ArticleDetail;
pub use articles::Articles;
pub use contact::Contact;
pub use home::Home;
pub use portfolio::Portfolio;
pub use portfolio_detail::PortfolioDetail;
pub use services::Services;
```

---

## Home Page

**File:** `src/pages/home.rs`

```rust
use crate::components::{CtaSection, ServiceCard};
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        // Hero Section
        section { class: "hero",
            div { class: "hero-content",
                h1 { class: "hero-title", "Your Technical Partner" }
                p { class: "hero-subtitle",
                    "Not a vendor. Not an agency. A partner who treats your business like it matters."
                }
                p { class: "hero-body",
                    "We work alongside you to build websites, fix technical problems, and make smart technology decisions. Direct communication. Honest advice. Work that lasts."
                }
                a {
                    href: "https://calendar.app.google/LNasBbmDr8LXNEuu5",
                    target: "_blank",
                    rel: "noopener noreferrer",
                    class: "btn btn-primary btn-large",
                    "Let's Talk"
                }
            }
            div { class: "hero-decoration" }
        }

        // Intro Section
        section { class: "section intro-section",
            div { class: "container",
                h2 { class: "section-title", "Technology Should Work for You" }
                div { class: "intro-content glass-card",
                    p {
                        "Most businesses don't need more software. They need the right software, built correctly, by someone who keeps things simple."
                    }
                    p {
                        "We help you figure out what you actually need, build it right, and make sure it keeps working. No jargon. No overcomplicated solutions. Just common sense and clean work."
                    }
                    p {
                        "Whether you're starting something new, fixing something broken, or just trying to figure out where to start, we can help."
                    }
                }
            }
        }

        // Services Overview Section
        section { class: "section services-section",
            div { class: "container",
                h2 { class: "section-title", "What We Do" }
                div { class: "services-grid",
                    ServiceCard {
                        title: "Website Development".to_string(),
                        description: "We build websites that load fast, look professional, and turn visitors into customers. No templates. No bloat. Just clean, custom work.".to_string(),
                        icon: "🌐".to_string()
                    }
                    ServiceCard {
                        title: "Digital Marketing".to_string(),
                        description: "Email, SMS, scheduling, customer intake, memberships. We set up the systems that bring in leads and keep customers coming back.".to_string(),
                        icon: "📱".to_string()
                    }
                    ServiceCard {
                        title: "Technical Strategy".to_string(),
                        description: "Not sure what you need? We help you figure it out before you spend money on the wrong thing. Honest advice, no sales pitch.".to_string(),
                        icon: "🎯".to_string()
                    }
                    ServiceCard {
                        title: "Business Solutions".to_string(),
                        description: "Connect your software, automate repetitive tasks, move data between systems. We fix the technical problems that slow you down.".to_string(),
                        icon: "⚡".to_string()
                    }
                }
                div { class: "section-cta",
                    Link { to: Route::Services {}, class: "btn btn-secondary",
                        "View All Services"
                    }
                }
            }
        }

        // Why Us Section
        section { class: "section why-section",
            div { class: "container",
                h2 { class: "section-title", "Why Work With Us" }
                div { class: "why-grid",
                    div { class: "why-card glass-card",
                        div { class: "why-icon", "✨" }
                        h3 { class: "why-title", "We keep it simple" }
                        p { class: "why-description",
                            "Every recommendation we make is based on what actually works for your situation, not what sounds impressive."
                        }
                    }
                    div { class: "why-card glass-card",
                        div { class: "why-icon", "🤝" }
                        h3 { class: "why-title", "We show up" }
                        p { class: "why-description",
                            "We answer emails. We meet deadlines. We tell you the truth, even when it's not what you want to hear."
                        }
                    }
                    div { class: "why-card glass-card",
                        div { class: "why-icon", "📈" }
                        h3 { class: "why-title", "We've done this before" }
                        p { class: "why-description",
                            "Five years of building software in healthcare, finance, and small business. We know what works and what doesn't."
                        }
                    }
                    div { class: "why-card glass-card",
                        div { class: "why-icon", "🧠" }
                        h3 { class: "why-title", "No problem too big" }
                        p { class: "why-description",
                            "Need expertise we don't have in-house? We have a network of brilliant specialists we can bring in. Whatever the challenge, we know someone who's solved it."
                        }
                    }
                }
            }
        }

        // CTA Section
        CtaSection {
            title: "Ready to get started?".to_string(),
            description: "Let's talk about what you're trying to build and how we can help.".to_string(),
            button_text: "Schedule a Call".to_string(),
            use_calendar_link: true
        }
    }
}
```

---

## About Page

**File:** `src/pages/about.rs`

```rust
use crate::components::CtaSection;
use dioxus::prelude::*;

#[component]
pub fn About() -> Element {
    rsx! {
        // Hero Section
        section { class: "hero hero-short",
            div { class: "hero-content",
                h1 { class: "hero-title", "Technology That Just Works" }
                p { class: "hero-subtitle",
                    "Whether you need your first website or your fiftieth integration, we speak your language and solve your problems."
                }
            }
        }

        // Personal Introduction
        section { class: "section personal-section",
            div { class: "container",
                div { class: "personal-content glass-card",
                    div { class: "personal-text",
                        h2 { class: "personal-title", "Hi, I'm Collin" }
                        p {
                            "I started Pounds Consulting to help local businesses and small teams navigate the technical side of "
                            "running a business. Whether it's your first website, a system to manage leads, or just figuring out "
                            "what tools you actually need - I'm here to help you make sense of it all."
                        }
                        p {
                            "My background includes building products at startups and working with enterprise clients, "
                            "but what I enjoy most is working directly with people in my community. There's something rewarding about "
                            "helping a local gym owner get their scheduling system working, or building a website for a business "
                            "that's been relying on word-of-mouth for years."
                        }
                        p {
                            "When I'm not building websites or automating workflows, you might find me hiking, dog-sitting, coaching swim lessons, "
                            "or tinkering in my AI lab (a spare bedroom with a desk). I value delivering solid, quality work for my clients, "
                            "keeping things simple, and being someone you'll want to work with again and again."
                        }
                        a {
                            href: "https://github.com/collinpounds",
                            class: "personal-github-link",
                            target: "_blank",
                            rel: "noopener noreferrer",
                            "See my work on GitHub →"
                        }
                    }
                }
            }
        }

        // Experience Banner
        section { class: "section clients-section",
            div { class: "container",
                div { class: "clients-banner glass-card",
                    h3 { class: "clients-title", "From Local Shops to Large Companies" }
                    p { class: "clients-subtitle", "We've built solutions for businesses of all sizes" }
                    div { class: "clients-logos",
                        span { class: "client-name", "Country Clubs" }
                        span { class: "client-divider", "•" }
                        span { class: "client-name", "Martial Arts Academies" }
                        span { class: "client-divider", "•" }
                        span { class: "client-name", "Regional Chains" }
                        span { class: "client-divider", "•" }
                        span { class: "client-name", "Healthcare Startups" }
                    }
                }
            }
        }

        // What We Can Help With Section
        section { class: "section skills-section",
            div { class: "container",
                h2 { class: "section-title", "What We Can Help With" }
                div { class: "skills-grid",
                    // Websites
                    div { class: "skill-category glass-card",
                        h3 { class: "skill-category-title", "Websites" }
                        div { class: "skill-pills",
                            span { class: "skill-pill", "Custom Websites" }
                            span { class: "skill-pill", "Mobile-Friendly" }
                            span { class: "skill-pill", "Fast Loading" }
                            span { class: "skill-pill", "Easy to Update" }
                            span { class: "skill-pill", "SEO Built-In" }
                        }
                    }
                    // Marketing
                    div { class: "skill-category glass-card",
                        h3 { class: "skill-category-title", "Marketing Systems" }
                        div { class: "skill-pills",
                            span { class: "skill-pill", "Email Campaigns" }
                            span { class: "skill-pill", "Text Messaging" }
                            span { class: "skill-pill", "Online Scheduling" }
                            span { class: "skill-pill", "Lead Capture" }
                            span { class: "skill-pill", "Customer Follow-up" }
                        }
                    }
                    // Business Tools
                    div { class: "skill-category glass-card",
                        h3 { class: "skill-category-title", "Business Tools" }
                        div { class: "skill-pills",
                            span { class: "skill-pill", "Connect Your Apps" }
                            span { class: "skill-pill", "Automate Tasks" }
                            span { class: "skill-pill", "Customer Portals" }
                            span { class: "skill-pill", "Online Payments" }
                            span { class: "skill-pill", "Custom Software" }
                        }
                    }
                    // Strategy
                    div { class: "skill-category glass-card",
                        h3 { class: "skill-category-title", "Guidance" }
                        div { class: "skill-pills",
                            span { class: "skill-pill", "What Do I Need?" }
                            span { class: "skill-pill", "Vendor Selection" }
                            span { class: "skill-pill", "Tech Strategy" }
                            span { class: "skill-pill", "Honest Advice" }
                            span { class: "skill-pill", "No Jargon" }
                        }
                    }
                }
            }
        }

        // Background Section
        section { class: "section timeline-section",
            div { class: "container",
                h2 { class: "section-title", "Background" }
                div { class: "timeline",
                    // Current
                    div { class: "timeline-item",
                        div { class: "timeline-marker" }
                        div { class: "timeline-content glass-card",
                            div { class: "timeline-header",
                                h3 { class: "timeline-title", "Independent Consulting" }
                                span { class: "timeline-company", "Pounds Consulting" }
                                span { class: "timeline-period", "Now" }
                            }
                            ul { class: "timeline-list",
                                li { "Building websites for local businesses who need something better than a template" }
                                li { "Helping business owners figure out what technology they actually need" }
                                li { "Setting up marketing systems that run on autopilot" }
                                li { "Connecting software so you don't have to copy-paste between apps" }
                            }
                        }
                    }

                    // Previous Experience
                    div { class: "timeline-item",
                        div { class: "timeline-marker" }
                        div { class: "timeline-content glass-card",
                            div { class: "timeline-header",
                                h3 { class: "timeline-title", "Before Going Independent" }
                                span { class: "timeline-company", "5+ Years in Tech" }
                            }
                            ul { class: "timeline-list",
                                li { "Worked at a healthcare startup used by hundreds of thousands of people" }
                                li { "Built integrations for companies like Hyatt and Commerce Bank" }
                                li { "Created websites and ran ad campaigns for a regional car wash chain" }
                                li { "Learned what works (and what doesn't) at scale" }
                            }
                        }
                    }

                    // Why We Do This
                    div { class: "timeline-item",
                        div { class: "timeline-marker" }
                        div { class: "timeline-content glass-card",
                            div { class: "timeline-header",
                                h3 { class: "timeline-title", "Why We Do This" }
                            }
                            ul { class: "timeline-list",
                                li { "Too many businesses get overcharged for confusing technology" }
                                li { "We like solving problems and explaining things in plain English" }
                                li { "Small businesses deserve the same quality work as big companies" }
                                li { "It's satisfying to build something that actually helps people" }
                            }
                        }
                    }
                }
            }
        }

        // How We Work Section
        section { class: "section philosophy-section",
            div { class: "container",
                div { class: "philosophy-content glass-card",
                    h2 { class: "philosophy-title", "How We Work" }
                    div { class: "philosophy-grid",
                        div { class: "philosophy-item",
                            span { class: "philosophy-number", "01" }
                            h3 { "We Listen First" }
                            p { "Before talking technology, we want to understand your business. What are you trying to accomplish? What's getting in the way?" }
                        }
                        div { class: "philosophy-item",
                            span { class: "philosophy-number", "02" }
                            h3 { "We Explain Everything" }
                            p { "No jargon. No confusing tech-speak. If you don't understand something, that's our fault, not yours." }
                        }
                        div { class: "philosophy-item",
                            span { class: "philosophy-number", "03" }
                            h3 { "We Keep It Simple" }
                            p { "The best solution is usually the simplest one. We won't sell you something complicated when something simple will work." }
                        }
                        div { class: "philosophy-item",
                            span { class: "philosophy-number", "04" }
                            h3 { "We Know People" }
                            p { "Need a designer? A security expert? We have a network of brilliant people we can call. Whatever the problem, we can figure it out." }
                        }
                    }
                }
            }
        }

        // CTA Section
        CtaSection {
            title: "Not Sure Where to Start?".to_string(),
            description: "That's okay. Most people who call us aren't sure what they need yet. Let's just talk and figure it out together.".to_string(),
            button_text: "Schedule a Free Call".to_string(),
            use_calendar_link: true
        }
    }
}
```

---

## Services Page

**File:** `src/pages/services.rs`

```rust
use crate::components::CtaSection;
use crate::content::load_settings;
use dioxus::prelude::*;

#[component]
pub fn Services() -> Element {
    let settings = load_settings();
    let discount = &settings.discount;

    // Calculate discounted rate if promo is enabled
    let base_rate: u32 = 71;
    let discounted_rate = if discount.promo_discount.enabled {
        let discount_amount = base_rate * discount.promo_discount.percentage as u32 / 100;
        base_rate - discount_amount
    } else {
        base_rate
    };

    rsx! {
        // Hero Section
        section { class: "hero hero-short",
            div { class: "hero-content",
                h1 { class: "hero-title", "Services Built Around Your Needs" }
                p { class: "hero-subtitle",
                    "From website builds to complex technical strategy, we offer flexible consulting services designed to solve real problems - not make expensive busywork."
                }
            }
        }

        // Digital Marketing
        section { class: "section service-detail-section",
            div { class: "container",
                div { class: "service-detail glass-card",
                    div { class: "service-detail-header",
                        span { class: "service-icon-large", "📱" }
                        h2 { class: "service-detail-title", "Digital Marketing & Customer Systems" }
                    }
                    p { class: "service-detail-description",
                        "A website is just the beginning. To grow, you need systems that capture leads, nurture relationships, and convert interest into revenue without requiring you to manually manage every touchpoint."
                    }
                    p { class: "service-detail-description",
                        "We build complete digital marketing infrastructures that work together seamlessly. From email campaigns to SMS outreach, from customer intake forms to membership management, we create the systems that turn visitors into customers and customers into repeat business."
                    }
                    div { class: "service-columns",
                        div { class: "service-column",
                            h3 { class: "service-column-title", "What's Included" }
                            ul { class: "service-list",
                                li { "Email marketing setup and campaign management" }
                                li { "SMS and phone outreach systems" }
                                li { "Customer intake and lead capture forms" }
                                li { "Appointment scheduling and booking systems" }
                                li { "Membership and subscription management" }
                                li { "Marketing automation and drip campaigns" }
                                li { "CRM integration and customer data management" }
                                li { "Analytics and conversion tracking" }
                            }
                        }
                        div { class: "service-column",
                            h3 { class: "service-column-title", "Good Fit For" }
                            ul { class: "service-list",
                                li { "Businesses ready to move beyond word-of-mouth marketing" }
                                li { "Service companies that need appointment scheduling" }
                                li { "Organizations with membership or subscription models" }
                                li { "Anyone who wants their marketing to run while they sleep" }
                            }
                        }
                    }
                }
            }
        }

        // Website Development
        section { class: "section service-detail-section",
            div { class: "container",
                div { class: "service-detail glass-card",
                    div { class: "service-detail-header",
                        span { class: "service-icon-large", "🌐" }
                        h2 { class: "service-detail-title", "Website Development" }
                    }
                    p { class: "service-detail-description",
                        "Your website is often the first impression people have of your business. It should load fast, look professional, and be intuitive for visitors to navigate."
                    }
                    p { class: "service-detail-description",
                        "We build websites that do exactly that. No bloated templates. No cookie-cutter designs. Every site is crafted with your specific goals in mind, whether that's generating leads, showcasing your work, or selling products online. User-friendly by default."
                    }
                    div { class: "service-columns",
                        div { class: "service-column",
                            h3 { class: "service-column-title", "What's Included" }
                            ul { class: "service-list",
                                li { "Custom design tailored to your brand and audience" }
                                li { "Mobile-responsive development (looks great on any device)" }
                                li { "Search engine optimization (SEO) foundations built in" }
                                li { "Fast load times and clean code" }
                                li { "Content management training so you can make updates yourself" }
                                li { "Support after launch" }
                            }
                        }
                        div { class: "service-column",
                            h3 { class: "service-column-title", "Good Fit For" }
                            ul { class: "service-list",
                                li { "Businesses launching their first professional website" }
                                li { "Companies with outdated sites that need a refresh" }
                                li { "Organizations that have outgrown DIY website builders" }
                                li { "Anyone who wants a site that actually converts visitors" }
                            }
                        }
                    }
                }
            }
        }

        // Technical Strategy
        section { class: "section service-detail-section",
            div { class: "container",
                div { class: "service-detail glass-card",
                    div { class: "service-detail-header",
                        span { class: "service-icon-large", "🎯" }
                        h2 { class: "service-detail-title", "Technical Strategy & Advisory" }
                    }
                    p { class: "service-detail-description",
                        "Not every problem needs code. Sometimes you need someone who can help you think through options, evaluate vendors, or figure out the right approach before you invest significant time and money."
                    }
                    p { class: "service-detail-description",
                        "That's where technical advisory comes in. We work alongside you to clarify requirements, assess solutions, and build a roadmap that makes sense for your budget and goals."
                    }
                    div { class: "service-columns",
                        div { class: "service-column",
                            h3 { class: "service-column-title", "What's Included" }
                            ul { class: "service-list",
                                li { "Technology assessments and vendor evaluations" }
                                li { "Product roadmapping and feature prioritization" }
                                li { "Build vs. buy analysis" }
                                li { "Technical due diligence for partnerships or acquisitions" }
                                li { "Architecture reviews and recommendations" }
                                li { "Documentation and knowledge transfer" }
                            }
                        }
                        div { class: "service-column",
                            h3 { class: "service-column-title", "Good Fit For" }
                            ul { class: "service-list",
                                li { "Startups planning their technical foundation" }
                                li { "Businesses evaluating new software platforms" }
                                li { "Teams that need an outside perspective on technical decisions" }
                                li { "Organizations preparing for growth or major changes" }
                            }
                        }
                    }
                }
            }
        }

        // Business Solutions
        section { class: "section service-detail-section",
            div { class: "container",
                div { class: "service-detail glass-card",
                    div { class: "service-detail-header",
                        span { class: "service-icon-large", "⚡" }
                        h2 { class: "service-detail-title", "Business Solutions & Integration" }
                    }
                    p { class: "service-detail-description",
                        "Technology is supposed to make your work easier. When systems don't talk to each other, processes require too many manual steps, or data lives in too many places, it creates friction that slows everything down."
                    }
                    p { class: "service-detail-description",
                        "We help businesses simplify operations by connecting systems, automating workflows, and building intuitive solutions for problems off-the-shelf software can't solve."
                    }
                    div { class: "service-columns",
                        div { class: "service-column",
                            h3 { class: "service-column-title", "What's Included" }
                            ul { class: "service-list",
                                li { "Software integrations and API connections" }
                                li { "Workflow automation (reduce manual data entry and repetitive tasks)" }
                                li { "Data migrations and system transitions" }
                                li { "Custom internal tools and dashboards" }
                                li { "Process documentation and optimization" }
                                li { "Ongoing support and maintenance" }
                            }
                        }
                        div { class: "service-column",
                            h3 { class: "service-column-title", "Good Fit For" }
                            ul { class: "service-list",
                                li { "Businesses running multiple disconnected software systems" }
                                li { "Teams drowning in manual processes and spreadsheets" }
                                li { "Organizations migrating from legacy systems" }
                                li { "Anyone who's said \"there has to be a better way to do this\"" }
                            }
                        }
                    }
                }
            }
        }

        // Pricing Section
        section { class: "section pricing-section",
            div { class: "container",
                h2 { class: "section-title", "Simple, Transparent Pricing" }
                div { class: "pricing-content glass-card",
                    p { class: "pricing-intro",
                        "Initial conversations are always free. We want to understand a problem in its entirety before accepting payment."
                    }

                    // Show discount badge if promo is enabled
                    if discount.promo_discount.enabled {
                        div { class: "discount-badge",
                            {discount.promo_discount.label.clone().unwrap_or_else(|| format!("{}% Off", discount.promo_discount.percentage))}
                        }
                    }

                    div { class: "pricing-rate",
                        if discount.promo_discount.enabled {
                            span { class: "rate-original", "${base_rate}" }
                            span { class: "rate-amount rate-discounted", "${discounted_rate}" }
                        } else {
                            span { class: "rate-amount", "${base_rate}" }
                        }
                        span { class: "rate-period", "/hour" }
                    }

                    p { class: "pricing-description",
                        "No hidden fees. No surprise charges. You'll always know what you're paying for and why."
                    }
                    p { class: "pricing-description",
                        "For larger projects, we can provide fixed-price quotes after a few discovery conversations. Either way, you'll have complete clarity on costs before any work begins."
                    }

                    // Book Time Button
                    div { class: "pricing-actions",
                        a {
                            href: "https://calendar.app.google/NxuWY3RDGE5Miaan7",
                            target: "_blank",
                            rel: "noopener noreferrer",
                            class: "btn btn-primary btn-large",
                            "Book Time"
                        }
                    }

                    div { class: "pricing-process",
                        h3 { class: "process-title", "What to Expect" }
                        div { class: "process-steps",
                            div { class: "process-step",
                                span { class: "step-number", "1" }
                                div { class: "step-content",
                                    strong { "Discovery Call" }
                                    p { "Free 30-minute conversation to understand your needs" }
                                }
                            }
                            div { class: "process-step",
                                span { class: "step-number", "2" }
                                div { class: "step-content",
                                    strong { "Proposal" }
                                    p { "Clear scope, timeline, and pricing in writing" }
                                }
                            }
                            div { class: "process-step",
                                span { class: "step-number", "3" }
                                div { class: "step-content",
                                    strong { "Execution" }
                                    p { "Regular updates and open communication throughout" }
                                }
                            }
                            div { class: "process-step",
                                span { class: "step-number", "4" }
                                div { class: "step-content",
                                    strong { "Delivery" }
                                    p { "Final handoff with documentation and training as needed" }
                                }
                            }
                        }
                    }
                }
            }
        }

        // First Responder Discount Section
        if discount.first_responder_enabled {
            section { class: "section first-responder-section",
                div { class: "container",
                    div { class: "first-responder-card glass-card",
                        div { class: "first-responder-badge", "🎖️" }
                        h3 { class: "first-responder-title", "50% Off for Those Who Serve" }
                        p { class: "first-responder-description",
                            "We offer 50% off our hourly rate for those who serve our communities and country. Your service matters."
                        }
                        ul { class: "first-responder-list",
                            li { "Military (Active Duty & Reserves)" }
                            li { "Veterans" }
                            li { "Law Enforcement" }
                            li { "Fire Fighters" }
                            li { "EMTs & Paramedics" }
                        }
                        p { class: "first-responder-cta",
                            "Mention your service when you book and we'll apply the discount."
                        }
                    }
                }
            }
        }

        // CTA Section
        CtaSection {
            title: "Not Sure What You Need?".to_string(),
            description: "That's okay. Most clients come to us with a problem, not a predefined solution. Let's talk through what you're trying to accomplish and figure out the right path forward together.".to_string(),
            button_text: "Start a Conversation".to_string(),
            use_calendar_link: true
        }
    }
}
```

---

## Portfolio Page

**File:** `src/pages/portfolio.rs`

```rust
use crate::components::CtaSection;
use crate::content::load_portfolio;
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Portfolio() -> Element {
    let portfolio_data = load_portfolio();

    rsx! {
        // Hero Section
        section { class: "hero hero-short",
            div { class: "hero-content",
                h1 { class: "hero-title", "Work That Speaks for Itself" }
                p { class: "hero-subtitle",
                    "A selection of projects we've delivered. Real clients, real problems, real results."
                }
            }
        }

        // Projects Section
        section { class: "section portfolio-section",
            div { class: "container",
                div { class: "portfolio-grid",
                    for project in &portfolio_data.projects {
                        Link {
                            key: "{project.id}",
                            to: Route::PortfolioDetail { slug: project.slug.clone() },
                            class: "portfolio-card-link",

                            div { class: "portfolio-card glass-card",
                                div { class: "portfolio-header",
                                    if let Some(logo) = &project.logo {
                                        img {
                                            class: "portfolio-card-logo",
                                            src: "/{logo}",
                                            alt: "{project.title} logo"
                                        }
                                    }
                                    div { class: "portfolio-header-text",
                                        h3 { class: "portfolio-title", "{project.title}" }
                                        span { class: "portfolio-type", "{project.project_type}" }
                                    }
                                }
                                p { class: "portfolio-description", "{project.description}" }
                                div { class: "portfolio-tech",
                                    for tag in &project.tech_tags {
                                        span { class: "tech-tag", "{tag}" }
                                    }
                                }
                                ul { class: "portfolio-scope",
                                    for item in &project.scope {
                                        li { "{item}" }
                                    }
                                }
                                span { class: "portfolio-link btn btn-primary",
                                    "View Case Study →"
                                }
                            }
                        }
                    }
                }
            }
        }

        // CTA Section
        CtaSection {
            title: "Ready to Join the List?".to_string(),
            description: "Every project starts with a conversation. Let's talk about what you're building.".to_string(),
            button_text: "Start Your Project".to_string(),
            use_calendar_link: true
        }
    }
}
```

---

## Portfolio Detail Page

**File:** `src/pages/portfolio_detail.rs`

```rust
use crate::content::load_portfolio;
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn PortfolioDetail(slug: String) -> Element {
    let portfolio_data = load_portfolio();
    let project = portfolio_data.projects.iter().find(|p| p.slug == slug);

    match project {
        Some(project) => {
            rsx! {
                // Case Study Hero
                section { class: "case-study-hero",
                    div { class: "container",
                        Link { to: Route::Portfolio {}, class: "case-study-back-link", "← Back to Portfolio" }

                        div { class: "case-study-header",
                            if let Some(logo) = &project.logo {
                                img {
                                    class: "case-study-logo",
                                    src: "/{logo}",
                                    alt: "{project.title} logo"
                                }
                            }

                            div { class: "case-study-title-section",
                                span { class: "case-study-type-badge", "{project.project_type}" }
                                h1 { class: "case-study-title", "{project.title}" }
                            }
                        }

                        p { class: "case-study-description", "{project.description}" }

                        div { class: "case-study-tech",
                            for tag in &project.tech_tags {
                                span { class: "tech-tag", "{tag}" }
                            }
                        }
                    }
                }

                // Media Section (screenshot/video placeholder)
                section { class: "case-study-media-section",
                    div { class: "container",
                        div { class: "case-study-media glass-card",
                            if let Some(video) = &project.video {
                                video {
                                    class: "case-study-video",
                                    src: "/{video}",
                                    autoplay: true,
                                    r#loop: true,
                                    muted: true,
                                    playsinline: true
                                }
                            } else if let Some(screenshot) = &project.screenshot {
                                img {
                                    class: "case-study-screenshot",
                                    src: "/{screenshot}",
                                    alt: "{project.title} screenshot"
                                }
                            } else {
                                div { class: "case-study-media-placeholder",
                                    span { class: "placeholder-icon", "🖼️" }
                                    p { "Screenshot or video coming soon" }
                                }
                            }
                        }
                    }
                }

                // Project Content
                section { class: "case-study-content-section",
                    div { class: "container",
                        div { class: "case-study-body glass-card",
                            // Render long description with markdown-like formatting
                            for paragraph in project.long_description.split("\n\n") {
                                if paragraph.starts_with("### ") {
                                    h3 { class: "case-study-h3", {paragraph.trim_start_matches("### ")} }
                                } else if paragraph.starts_with("## ") {
                                    h2 { class: "case-study-h2", {paragraph.trim_start_matches("## ")} }
                                } else if paragraph.starts_with("- ") || paragraph.starts_with("* ") {
                                    ul { class: "case-study-list",
                                        for line in paragraph.lines() {
                                            if line.starts_with("- ") || line.starts_with("* ") {
                                                li { {line.trim_start_matches("- ").trim_start_matches("* ")} }
                                            }
                                        }
                                    }
                                } else if !paragraph.trim().is_empty() {
                                    p { "{paragraph}" }
                                }
                            }
                        }

                        // Scope of Work
                        div { class: "case-study-scope glass-card",
                            h3 { "Scope of Work" }
                            ul {
                                for item in &project.scope {
                                    li { "{item}" }
                                }
                            }
                        }

                        // Action Buttons
                        div { class: "case-study-actions",
                            a {
                                href: "{project.external_url}",
                                target: "_blank",
                                rel: "noopener noreferrer",
                                class: "btn btn-primary btn-large",
                                "Visit Site →"
                            }
                            if let Some(before_url) = &project.before_url {
                                a {
                                    href: "{before_url}",
                                    target: "_blank",
                                    rel: "noopener noreferrer",
                                    class: "btn btn-secondary btn-large",
                                    "View Before →"
                                }
                            }
                        }
                    }
                }

                // Related Projects
                section { class: "section related-projects",
                    div { class: "container",
                        h2 { class: "section-title", "More Projects" }
                        div { class: "portfolio-grid",
                            for other in portfolio_data.projects.iter()
                                .filter(|p| p.slug != slug)
                                .take(3)
                            {
                                Link {
                                    key: "{other.id}",
                                    to: Route::PortfolioDetail { slug: other.slug.clone() },
                                    class: "portfolio-card glass-card",

                                    div { class: "portfolio-header",
                                        if let Some(logo) = &other.logo {
                                            img {
                                                class: "portfolio-card-logo",
                                                src: "/{logo}",
                                                alt: "{other.title} logo"
                                            }
                                        }
                                        div { class: "portfolio-header-text",
                                            h3 { class: "portfolio-title", "{other.title}" }
                                            span { class: "portfolio-type", "{other.project_type}" }
                                        }
                                    }

                                    p { class: "portfolio-description", "{other.description}" }

                                    div { class: "portfolio-tech",
                                        for tag in other.tech_tags.iter().take(3) {
                                            span { class: "tech-tag", "{tag}" }
                                        }
                                    }

                                    span { class: "portfolio-read-more", "View Case Study →" }
                                }
                            }
                        }
                    }
                }
            }
        }
        None => {
            rsx! {
                section { class: "hero hero-short",
                    div { class: "hero-content",
                        h1 { class: "hero-title", "Project Not Found" }
                        p { class: "hero-subtitle", "The project you're looking for doesn't exist." }
                        Link { to: Route::Portfolio {}, class: "btn btn-primary", "View All Projects" }
                    }
                }
            }
        }
    }
}
```

---

## Articles Page

**File:** `src/pages/articles.rs`

```rust
use crate::components::CtaSection;
use crate::content::{load_articles, ArticleStatus};
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Articles() -> Element {
    let articles_data = load_articles();
    let published: Vec<_> = articles_data
        .articles
        .into_iter()
        .filter(|a| matches!(a.status, ArticleStatus::Published))
        .collect();

    // Get unique categories
    let categories: Vec<String> = {
        let mut cats: Vec<String> = published.iter().map(|a| a.category.clone()).collect();
        cats.sort();
        cats.dedup();
        cats
    };

    let mut selected_category = use_signal(|| Option::<String>::None);

    let filtered_articles: Vec<_> = if let Some(ref cat) = selected_category() {
        published.iter().filter(|a| &a.category == cat).collect()
    } else {
        published.iter().collect()
    };

    rsx! {
        // Hero Section
        section { class: "hero hero-short",
            div { class: "hero-content",
                h1 { class: "hero-title", "Articles" }
                p { class: "hero-subtitle",
                    "Insights, updates, and perspectives on technology and business."
                }
            }
        }

        // Articles Section
        section { class: "section articles-section",
            div { class: "container",
                // Category Filter
                if !categories.is_empty() {
                    div { class: "articles-filter",
                        button {
                            class: if selected_category().is_none() { "filter-btn filter-btn-active" } else { "filter-btn" },
                            onclick: move |_| selected_category.set(None),
                            "All"
                        }
                        for cat in categories.iter() {
                            button {
                                key: "{cat}",
                                class: if selected_category().as_ref() == Some(cat) { "filter-btn filter-btn-active" } else { "filter-btn" },
                                onclick: {
                                    let cat = cat.clone();
                                    move |_| selected_category.set(Some(cat.clone()))
                                },
                                "{cat}"
                            }
                        }
                    }
                }

                // Articles Grid
                if filtered_articles.is_empty() {
                    div { class: "articles-empty glass-card",
                        h3 { "No articles yet" }
                        p { "Check back soon for new content." }
                    }
                } else {
                    div { class: "articles-grid",
                        for article in filtered_articles {
                            Link {
                                key: "{article.id}",
                                to: Route::ArticleDetail { slug: article.slug.clone() },
                                class: "article-card glass-card",

                                div { class: "article-card-header",
                                    span { class: "article-category", "{article.category}" }
                                    span { class: "article-date", "{article.date}" }
                                }

                                h2 { class: "article-card-title", "{article.title}" }

                                p { class: "article-card-excerpt", "{article.excerpt}" }

                                span { class: "article-read-more", "Read more →" }
                            }
                        }
                    }
                }
            }
        }

        // CTA Section
        CtaSection {
            title: "Have a Question?".to_string(),
            description: "We're always happy to discuss technology, business, or potential projects.".to_string(),
            button_text: "Get in Touch".to_string(),
            use_calendar_link: true
        }
    }
}
```

---

## Article Detail Page

**File:** `src/pages/article_detail.rs`

```rust
use crate::content::{load_articles, ArticleStatus};
use crate::Route;
use dioxus::prelude::*;

/// Render a paragraph, handling inline **bold** text
fn render_paragraph(text: &str) -> Element {
    // Check if the paragraph has bold markers
    if text.contains("**") {
        // For simplicity, just strip the ** markers
        let clean_text = text.replace("**", "");
        rsx! {
            p { "{clean_text}" }
        }
    } else {
        rsx! {
            p { "{text}" }
        }
    }
}

#[component]
pub fn ArticleDetail(slug: String) -> Element {
    let articles_data = load_articles();
    let article = articles_data
        .articles
        .iter()
        .find(|a| a.slug == slug && matches!(a.status, ArticleStatus::Published));

    match article {
        Some(article) => {
            rsx! {
                // Article Header
                section { class: "article-hero",
                    div { class: "container",
                        Link { to: Route::Articles {}, class: "article-back-link", "← Back to Articles" }

                        div { class: "article-meta",
                            span { class: "article-category-badge", "{article.category}" }
                            span { class: "article-date", "{article.date}" }
                        }

                        h1 { class: "article-title", "{article.title}" }

                        if !article.excerpt.is_empty() {
                            p { class: "article-excerpt", "{article.excerpt}" }
                        }
                    }
                }

                // Article Content
                section { class: "article-content-section",
                    div { class: "container",
                        div { class: "article-body glass-card",
                            // Render content with markdown-like formatting
                            for paragraph in article.content.split("\n\n") {
                                if paragraph.starts_with("### ") {
                                    h3 { class: "article-h3", {paragraph.trim_start_matches("### ")} }
                                } else if paragraph.starts_with("## ") {
                                    h2 { class: "article-h2", {paragraph.trim_start_matches("## ")} }
                                } else if paragraph.starts_with("# ") {
                                    h2 { class: "article-h2", {paragraph.trim_start_matches("# ")} }
                                } else if paragraph.starts_with("- ") || paragraph.starts_with("* ") {
                                    // Render as a list
                                    ul { class: "article-list",
                                        for line in paragraph.lines() {
                                            if line.starts_with("- ") || line.starts_with("* ") {
                                                li { {line.trim_start_matches("- ").trim_start_matches("* ")} }
                                            }
                                        }
                                    }
                                } else if paragraph.starts_with("**") && paragraph.ends_with("**") {
                                    // Bold standalone line (like "**How to avoid it:**")
                                    p { class: "article-bold", {paragraph.trim_matches('*')} }
                                } else if !paragraph.trim().is_empty() {
                                    // Handle bold text within paragraph
                                    {render_paragraph(paragraph)}
                                }
                            }
                        }

                        // Share Section
                        div { class: "article-share",
                            span { class: "share-label", "Share this article:" }
                            div { class: "share-buttons",
                                a {
                                    href: "https://twitter.com/intent/tweet?text={article.title}&url=",
                                    target: "_blank",
                                    rel: "noopener noreferrer",
                                    class: "share-btn",
                                    "Twitter"
                                }
                                a {
                                    href: "https://www.linkedin.com/sharing/share-offsite/?url=",
                                    target: "_blank",
                                    rel: "noopener noreferrer",
                                    class: "share-btn",
                                    "LinkedIn"
                                }
                                button {
                                    class: "share-btn",
                                    onclick: move |_| {
                                        // Copy URL to clipboard using eval for simplicity
                                        #[cfg(target_arch = "wasm32")]
                                        {
                                            let _ = js_sys::eval("navigator.clipboard.writeText(window.location.href)");
                                        }
                                    },
                                    "Copy Link"
                                }
                            }
                        }
                    }
                }

                // More Articles
                section { class: "section related-articles",
                    div { class: "container",
                        h2 { class: "section-title", "More Articles" }
                        div { class: "articles-grid",
                            for other in articles_data.articles.iter()
                                .filter(|a| a.slug != slug && matches!(a.status, ArticleStatus::Published))
                                .take(3)
                            {
                                Link {
                                    key: "{other.id}",
                                    to: Route::ArticleDetail { slug: other.slug.clone() },
                                    class: "article-card glass-card",

                                    div { class: "article-card-header",
                                        span { class: "article-category", "{other.category}" }
                                        span { class: "article-date", "{other.date}" }
                                    }

                                    h3 { class: "article-card-title", "{other.title}" }

                                    p { class: "article-card-excerpt", "{other.excerpt}" }

                                    span { class: "article-read-more", "Read more →" }
                                }
                            }
                        }
                    }
                }
            }
        }
        None => {
            rsx! {
                section { class: "hero hero-short",
                    div { class: "hero-content",
                        h1 { class: "hero-title", "Article Not Found" }
                        p { class: "hero-subtitle", "The article you're looking for doesn't exist." }
                        Link { to: Route::Articles {}, class: "btn btn-primary", "View All Articles" }
                    }
                }
            }
        }
    }
}
```

---

## Contact Page

**File:** `src/pages/contact.rs`

```rust
use dioxus::prelude::*;

#[component]
pub fn Contact() -> Element {
    rsx! {
        // Hero Section
        section { class: "hero hero-short",
            div { class: "hero-content",
                h1 { class: "hero-title", "Let's Talk" }
                p { class: "hero-subtitle",
                    "Have a project in mind? Questions? Just want to network? Book some time."
                }
            }
        }

        // Contact Section
        section { class: "section contact-section",
            div { class: "container",
                div { class: "contact-centered",
                    // Schedule CTA
                    div { class: "schedule-card glass-card",
                        h2 { class: "schedule-title", "Schedule a Call" }
                        p { class: "schedule-description",
                            "Book a free 30-minute discovery call. We'll discuss your project, answer questions, and figure out if we're a good fit."
                        }
                        a {
                            href: "https://calendar.app.google/LNasBbmDr8LXNEuu5",
                            target: "_blank",
                            rel: "noopener noreferrer",
                            class: "btn btn-primary btn-large",
                            "Book a Time"
                        }
                    }

                    // Contact Info
                    div { class: "contact-details glass-card",
                        h3 { class: "contact-heading", "Other Ways to Reach Us" }
                        div { class: "contact-item",
                            span { class: "contact-label", "Email" }
                            a {
                                href: "mailto:collin@poundsconsulting.net",
                                class: "contact-value contact-link",
                                "collin@poundsconsulting.net"
                            }
                        }
                        div { class: "contact-item",
                            span { class: "contact-label", "Location" }
                            span { class: "contact-value", "Kansas City, Missouri" }
                            span { class: "contact-note", }
                        }
                    }
                }

                // FAQ Section
                div { class: "faq-section",
                    h2 { class: "section-title", "Common Questions" }
                    div { class: "faq-grid",
                        div { class: "faq-item glass-card",
                            h3 { class: "faq-question", "What's a typical project timeline?" }
                            p { class: "faq-answer",
                                "Simple websites can launch in 2-4 weeks. More complex projects vary based on scope, but we'll provide a realistic timeline during our initial conversation."
                            }
                        }
                        div { class: "faq-item glass-card",
                            h3 { class: "faq-question", "Do you work with clients outside Missouri?" }
                            p { class: "faq-answer",
                                "Absolutely. While we're based in Columbia, most of our work is done remotely. We work with clients across the country."
                            }
                        }
                        div { class: "faq-item glass-card",
                            h3 { class: "faq-question", "What if I'm not sure what I need?" }
                            p { class: "faq-answer",
                                "That's completely fine. Most conversations start with a problem, not a solution. Book a call and we'll help you figure out the right approach."
                            }
                        }
                        div { class: "faq-item glass-card",
                            h3 { class: "faq-question", "How does billing work?" }
                            p { class: "faq-answer",
                                "We bill hourly at $100/hour for most work, invoiced monthly. For fixed-scope projects, we provide a complete quote upfront."
                            }
                        }
                    }
                }
            }
        }
    }
}
```

---

# Content/Data Layer

## Content Module

**File:** `src/content/mod.rs`

```rust
pub mod storage;
pub mod types;

pub use storage::*;
pub use types::*;
```

---

## Content Types (Data Models)

**File:** `src/content/types.rs`

```rust
use serde::{Deserialize, Serialize};

/// Site-wide settings including branding and feature toggles
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SiteSettings {
    pub brand: BrandSettings,
    pub features: FeatureToggles,
    pub pages: Vec<PageConfig>,
    pub admin_password_hash: String,
    #[serde(default)]
    pub discount: DiscountSettings,
}

/// Settings for promotional and special discounts
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DiscountSettings {
    /// Holiday/promotional discount (admin-controlled)
    pub promo_discount: PromoDiscount,
    /// First responder/military discount visibility
    pub first_responder_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PromoDiscount {
    pub enabled: bool,
    pub percentage: u8,
    pub label: Option<String>,
}

impl Default for DiscountSettings {
    fn default() -> Self {
        Self {
            promo_discount: PromoDiscount {
                enabled: false,
                percentage: 10,
                label: None,
            },
            first_responder_enabled: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BrandSettings {
    pub name: String,
    pub tagline: String,
    pub primary_color: String,
    pub accent_color: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FeatureToggles {
    pub portfolio: bool,
    pub services: bool,
    pub articles: bool,
    pub contact: bool,
    pub testimonials: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PageConfig {
    pub id: String,
    pub label: String,
    pub path: String,
    pub enabled: bool,
    pub order: u32,
}

/// Article/blog post
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Article {
    pub id: String,
    pub title: String,
    pub slug: String,
    pub date: String,
    pub category: String,
    pub excerpt: String,
    pub content: String,
    pub status: ArticleStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ArticleStatus {
    Draft,
    Published,
    Trashed,
}

/// Container for all articles
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ArticlesData {
    pub articles: Vec<Article>,
}

/// Portfolio project/case study
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PortfolioProject {
    pub id: String,
    pub slug: String,
    pub title: String,
    pub project_type: String,
    pub description: String,
    pub long_description: String,
    pub external_url: String,
    pub before_url: Option<String>,
    pub logo: Option<String>,
    pub screenshot: Option<String>,
    pub video: Option<String>,
    pub tech_tags: Vec<String>,
    pub scope: Vec<String>,
}

/// Container for all portfolio projects
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PortfolioData {
    pub projects: Vec<PortfolioProject>,
}

impl Default for SiteSettings {
    fn default() -> Self {
        Self {
            brand: BrandSettings {
                name: "Pounds Consulting".to_string(),
                tagline: "Technical Solutions for Growing Businesses".to_string(),
                primary_color: "#C9A227".to_string(),
                accent_color: "#D4AF37".to_string(),
            },
            features: FeatureToggles {
                portfolio: true,
                services: true,
                articles: true,
                contact: true,
                testimonials: false,
            },
            pages: vec![
                PageConfig {
                    id: "home".to_string(),
                    label: "Home".to_string(),
                    path: "/".to_string(),
                    enabled: true,
                    order: 1,
                },
                PageConfig {
                    id: "about".to_string(),
                    label: "About".to_string(),
                    path: "/about".to_string(),
                    enabled: true,
                    order: 2,
                },
                PageConfig {
                    id: "services".to_string(),
                    label: "Services".to_string(),
                    path: "/services".to_string(),
                    enabled: true,
                    order: 3,
                },
                PageConfig {
                    id: "portfolio".to_string(),
                    label: "Portfolio".to_string(),
                    path: "/portfolio".to_string(),
                    enabled: true,
                    order: 4,
                },
                PageConfig {
                    id: "articles".to_string(),
                    label: "Articles".to_string(),
                    path: "/articles".to_string(),
                    enabled: true,
                    order: 5,
                },
                PageConfig {
                    id: "contact".to_string(),
                    label: "Contact".to_string(),
                    path: "/contact".to_string(),
                    enabled: true,
                    order: 6,
                },
            ],
            // Default password: "admin" - users should change this
            admin_password_hash: "admin".to_string(),
            discount: DiscountSettings::default(),
        }
    }
}

impl Default for ArticlesData {
    fn default() -> Self {
        Self {
            articles: vec![
                Article {
                    id: "do-you-need-custom-website".to_string(),
                    title: "Do You Actually Need a Custom Website?".to_string(),
                    slug: "do-you-need-custom-website".to_string(),
                    date: "2025-01-02".to_string(),
                    category: "Advice".to_string(),
                    excerpt: "Before spending thousands on a custom site, here's how to figure out if you actually need one.".to_string(),
                    content: r#"A lot of businesses pay for custom websites when they don't need them. Here's how to figure out what's right for you.

## When a Template Works Fine

If your business fits a common pattern (local service business, restaurant, small retail shop) a template will probably work. Squarespace, Wix, and similar tools have gotten pretty good. You can have a professional-looking site up in a weekend for under $200/year.

Templates work when:
- Your site mostly shows information (hours, location, services, contact)
- You don't need to collect data or process payments in unusual ways
- You're okay looking similar to other businesses in your industry
- You can handle basic updates yourself

## When You Need Something Custom

Custom makes sense when:
- You need your website to DO something specific (booking systems, customer portals, calculators)
- You're integrating with other software your business uses
- Your business model doesn't fit standard templates
- Speed and performance are critical to your revenue
- You need to stand out in a crowded market

## The Middle Ground

There's a third option many people miss: start with a template, then add custom pieces. Use Squarespace for your main site, but build a custom tool for that one specific thing you need. This saves money and gets you moving faster.

## The Real Question

Don't ask "should I build custom?" Ask "what do I need this website to accomplish?" Start there, and the right answer usually becomes obvious.

If you're not sure, we're happy to talk through it. No sales pitch, just honest advice about what makes sense for your situation."#.to_string(),
                    status: ArticleStatus::Published,
                },
                Article {
                    id: "red-flags-hiring-developer".to_string(),
                    title: "Red Flags When Hiring a Developer".to_string(),
                    slug: "red-flags-hiring-developer".to_string(),
                    date: "2025-01-01".to_string(),
                    category: "Advice".to_string(),
                    excerpt: "How to spot problems before you've wasted time and money on the wrong hire.".to_string(),
                    content: r#"Hiring a developer or agency can feel like a gamble. Here are warning signs we've seen lead to bad outcomes.

## They Can't Explain Things Simply

Good developers can explain technical concepts in plain English. If someone hides behind jargon or makes you feel stupid for asking questions, that's a problem. You'll be working with this person, so communication matters.

## They Promise Everything Will Be Easy

Software is rarely easy. If someone says your project will be quick and cheap without asking many questions first, they either don't understand what you're asking for, or they're telling you what you want to hear.

## No Portfolio or References

Everyone has to start somewhere, but experienced developers should be able to show you past work. Ask for references. Actually call them. Ask what went wrong on the project (something always does) and how the developer handled it.

## They Want All the Money Upfront

Standard practice is milestone-based payments. Some money upfront is reasonable, but if they want 100% before starting, walk away. You lose all leverage if something goes wrong.

## They Don't Ask About Your Business

A developer who jumps straight to technical solutions without understanding your business goals will build the wrong thing. Good developers ask lots of questions first.

## Unusually Low Prices

If one bid is half the price of everyone else, something's wrong. Either they don't understand the scope, they're going to cut corners, or they'll hit you with change orders later.

## What to Look For Instead

- Clear communication
- Honest about challenges and tradeoffs
- Asks good questions about your goals
- Has relevant experience they can demonstrate
- Reasonable pricing with clear milestones
- Responsive during the sales process (it only gets worse after you sign)"#.to_string(),
                    status: ArticleStatus::Published,
                },
                Article {
                    id: "questions-before-building-app".to_string(),
                    title: "5 Questions to Answer Before Building an App".to_string(),
                    slug: "questions-before-building-app".to_string(),
                    date: "2024-12-28".to_string(),
                    category: "Strategy".to_string(),
                    excerpt: "Most app projects fail because people skip these questions. Don't be one of them.".to_string(),
                    content: r#"Before you spend money building an app, make sure you can answer these questions clearly.

## 1. What Problem Does This Solve?

Not "what would be cool to have" but what actual problem are real people experiencing that this app fixes? If you can't describe the problem in one sentence, you're not ready to build.

## 2. Who Exactly Will Use This?

"Everyone" is not an answer. Who specifically? How old are they? What do they do? Where will they find your app? The more specific you can be, the better your app will be.

## 3. How Will People Find It?

This is where most apps die. Building it is the easy part. Getting people to actually download and use it is hard. What's your plan? Be specific.

## 4. What's the Simplest Version That Solves the Problem?

Your first version should do one thing well. Not ten things. Not five things. One thing. You can add more later. Most apps fail because they try to do too much too soon.

## 5. How Will You Make Money?

Apps cost money to build and maintain. How does this pay for itself? Subscriptions? One-time purchase? Advertising? In-app purchases? If you don't have a clear answer, you'll run out of money before you succeed.

## Still Want to Build?

If you can answer all five questions clearly, you might be ready. If not, spend more time on these before writing any code. The cheapest time to change your mind is before you start building.

We're happy to help you think through these questions. Sometimes a 30-minute conversation saves months of wasted effort."#.to_string(),
                    status: ArticleStatus::Published,
                },
                Article {
                    id: "why-software-projects-fail".to_string(),
                    title: "Why Most Software Projects Fail (And How to Avoid It)".to_string(),
                    slug: "why-software-projects-fail".to_string(),
                    date: "2024-12-20".to_string(),
                    category: "Strategy".to_string(),
                    excerpt: "After years of building software, we've seen the same mistakes over and over. Here's how to avoid them.".to_string(),
                    content: r#"Most software projects fail. Not because of bad code, but because of bad decisions made before any code was written.

## The Scope Keeps Growing

This is the number one killer. You start with a simple idea, then keep adding "just one more thing." Each feature seems small, but they add up. Before you know it, you're building something completely different from what you planned.

**How to avoid it:** Write down exactly what version 1 will do. Put it somewhere visible. Every time someone suggests a new feature, ask "Is this essential for launch, or can it wait for version 2?"

## Nobody Agrees on What "Done" Means

The project drags on forever because there's no clear finish line. Different people have different ideas of what success looks like.

**How to avoid it:** Before you start, define what "done" means in writing. What has to work? What can be imperfect? Get everyone to agree on this upfront.

## Building Before Validating

People spend months building something, then find out nobody wants it. Or they want something slightly different.

**How to avoid it:** Before building the full product, test your idea cheaply. Mock-ups, landing pages, manual versions of the process. Find out if people will actually pay for this before you invest heavily.

## Poor Communication

The developer builds what they understood, not what you meant. Weeks of work get thrown away because of a misunderstanding.

**How to avoid it:** Over-communicate. Check in frequently. Review work in progress, not just finished features. Ask "Can you show me what you have so far?" regularly.

## No One Is In Charge

When everyone is responsible, no one is. Decisions don't get made. Problems don't get solved.

**How to avoid it:** One person needs to be the decision-maker. They don't have to be right about everything, but someone has to be able to break ties and keep things moving.

## The Pattern

Notice something? Most of these problems are about communication and planning, not technology. The technical part is usually the easy part. Getting humans aligned is the hard part."#.to_string(),
                    status: ArticleStatus::Published,
                },
                Article {
                    id: "true-cost-free-website-builders".to_string(),
                    title: "The True Cost of 'Free' Website Builders".to_string(),
                    slug: "true-cost-free-website-builders".to_string(),
                    date: "2024-12-15".to_string(),
                    category: "Advice".to_string(),
                    excerpt: "Free sounds great until you add up what you're actually paying. Here's what those website builders really cost.".to_string(),
                    content: r#"Wix, Squarespace, and similar tools advertise low prices, but the actual cost is usually higher than it looks.

## The Monthly Fees Add Up

That $16/month plan is $192/year. Over 5 years, you've spent nearly $1,000, and you still don't own anything. Cancel your subscription and your site disappears.

## The Real Plans Cost More

The cheap plan usually has their branding on your site and missing features you'll need. Once you add a custom domain, remove ads, and get the features you actually need, you're often at $30-50/month.

## The Hidden Costs

- **Apps and plugins:** Many features require paid add-ons
- **Transaction fees:** Selling something? They take a cut
- **Storage and bandwidth:** Heavy use costs extra
- **Email:** Usually separate and extra
- **Support:** Good luck getting help on the cheap plans

## What You're Giving Up

### Portability
Your site lives on their platform. Want to move? You're starting over. That design, those pages, that SEO you built up, none of it transfers.

### Control
You can only do what their platform allows. Need something custom? Too bad. Their servers slow? Nothing you can do.

### Ownership
You're renting, not owning. They can change prices, features, or terms whenever they want.

## When It's Still Worth It

Despite all this, website builders make sense when:
- You need something up fast and cheap
- Your needs are simple and standard
- You're testing an idea before investing more
- You genuinely can't afford anything else right now

## The Alternative

A custom website costs more upfront but often less over time. You own it. You control it. You can move it. For a business that plans to be around for years, the math usually favors custom.

Run the numbers for your specific situation. Sometimes the "expensive" option is actually cheaper in the long run."#.to_string(),
                    status: ArticleStatus::Published,
                },
                Article {
                    id: "what-to-expect-working-with-us".to_string(),
                    title: "What to Expect When Working With Us".to_string(),
                    slug: "what-to-expect-working-with-us".to_string(),
                    date: "2024-12-10".to_string(),
                    category: "About Us".to_string(),
                    excerpt: "Here's how we work with clients, what we expect from you, and what you can expect from us.".to_string(),
                    content: r#"Every consultant works differently. Here's how we do things so you know what you're getting into.

## How Projects Start

We start with a conversation, usually 30 minutes to an hour. No charge. We want to understand what you're trying to accomplish, what you've tried, and what's getting in the way.

After that, we'll tell you honestly whether we think we can help. Sometimes the answer is "you don't need us" or "someone else would be a better fit." We'd rather say that upfront than waste your time and money.

## How We Work

### Communication
We respond to emails within one business day. Usually faster. We believe in short, frequent check-ins rather than long silences followed by big reveals.

### Honesty
We tell you the truth, even when it's not what you want to hear. If your idea won't work, we'll say so. If something is taking longer than expected, you'll know right away.

### Simplicity
We look for the simplest solution that solves your problem. Not the coolest technology. Not the most impressive architecture. The simplest thing that works.

## What We Expect From You

### Availability
We need you to be reachable. Projects stall when we can't get answers to questions. You don't need to be available 24/7, but we need reasonable response times.

### Decisions
Someone needs to be able to make decisions. If every question requires a committee meeting, projects drag on forever.

### Honesty
If something isn't working for you, tell us. We can't fix problems we don't know about.

## Pricing

We work on a project basis with clear milestones and deliverables. You'll know the total cost before we start. We don't nickel-and-dime with change orders for small stuff.

For ongoing work, we offer monthly retainers. Fixed price, predictable costs.

## Ready to Talk?

If this sounds like a good fit, let's have a conversation. No commitment, no sales pressure. Just a chance to see if working together makes sense."#.to_string(),
                    status: ArticleStatus::Published,
                },
            ],
        }
    }
}

impl Default for PortfolioData {
    fn default() -> Self {
        Self {
            projects: vec![
                PortfolioProject {
                    id: "paytient".to_string(),
                    slug: "paytient".to_string(),
                    title: "Paytient".to_string(),
                    project_type: "Product Development".to_string(),
                    description: "Contributed to a healthcare fintech startup serving hundreds of thousands of users. Removed friction from the onboarding flow, redesigned the my.paytient.com landing page, and led a team of 6 engineers implementing multi-factor authentication.".to_string(),
                    long_description: r#"## The Challenge

Paytient is a healthcare fintech company that helps employees pay for medical expenses over time. With hundreds of thousands of users depending on the platform, every interaction matters.

## What We Did

### Onboarding Flow Optimization
We identified friction points in the member onboarding process and redesigned the flow to reduce drop-off rates. By simplifying the steps and improving the UI, we helped more users successfully complete registration.

### Landing Page Redesign
The my.paytient.com member portal landing page needed a refresh. We designed and implemented a cleaner, more intuitive experience that better communicated available benefits and next steps.

### Multi-Factor Authentication
Security is critical for financial applications. We led a team of 6 engineers to implement MFA across the entire platform, protecting user accounts while maintaining a smooth experience.

## Results

- Improved onboarding completion rates
- Better member engagement on the portal
- Enterprise-grade security with MFA
- Scalable architecture supporting continued growth"#.to_string(),
                    external_url: "https://my.paytient.com".to_string(),
                    before_url: None,
                    logo: Some("assets/portfolio/paytient-logo.svg".to_string()),
                    screenshot: None,
                    video: None,
                    tech_tags: vec!["React".to_string(), "UX".to_string(), "MFA".to_string(), "Team Lead".to_string()],
                    scope: vec![
                        "Streamlined onboarding by removing unnecessary friction step".to_string(),
                        "Redesigned member landing page for better engagement".to_string(),
                        "Led 6-person team implementing MFA across the platform".to_string(),
                    ],
                },
                PortfolioProject {
                    id: "club-car-wash".to_string(),
                    slug: "club-car-wash".to_string(),
                    title: "Club Car Wash".to_string(),
                    project_type: "Website + Portal + Digital Marketing".to_string(),
                    description: "Complete digital transformation for a growing regional car wash chain. Built and managed the public-facing website, developed an internal employee portal, and ran ongoing Google Ads campaigns to support new store openings.".to_string(),
                    long_description: r#"## The Challenge

Club Car Wash is a rapidly expanding car wash chain opening approximately 3 new stores per month. They needed a complete digital presence that could scale with their growth.

## What We Did

### Public Website
Designed and developed a customer-facing website that showcases locations, membership options, and the Club Car Wash brand. The site is built for easy updates as new locations open.

### Employee Portal
Created an internal portal for employee operations, streamlining internal communication and processes across all locations.

### Digital Marketing
Managed Google Ads campaigns coordinated with new store openings, driving local awareness and membership sign-ups in each new market.

## Results

- Scalable website supporting rapid expansion
- Centralized employee portal across all locations
- Successful launch marketing for ~3 store openings per month
- 1+ year of ongoing maintenance and support"#.to_string(),
                    external_url: "https://clubcarwash.com".to_string(),
                    before_url: None,
                    logo: None,
                    screenshot: None,
                    video: None,
                    tech_tags: vec!["React".to_string(), "Custom CMS".to_string(), "Google Ads".to_string()],
                    scope: vec![
                        "Public website design and development".to_string(),
                        "Employee portal for internal operations".to_string(),
                        "Google Ad campaign management (~3 store openings/month)".to_string(),
                        "Ongoing maintenance and support for 1 year".to_string(),
                    ],
                },
                PortfolioProject {
                    id: "old-hawthorne".to_string(),
                    slug: "old-hawthorne".to_string(),
                    title: "Old Hawthorne Country Club".to_string(),
                    project_type: "Website Consulting".to_string(),
                    description: "Consulting work for a local country club community in Columbia, Missouri. Made targeted adjustments to improve the site's look and navigation, including replacing the dated beige wallpaper background with a cleaner design.".to_string(),
                    long_description: r#"## The Challenge

Old Hawthorne Country Club had an existing website that felt dated and didn't reflect the quality of the community. They needed targeted improvements without a complete rebuild.

## What We Did

### Visual Refresh
The most obvious issue was a dated beige wallpaper background that made the site feel old. We replaced it with a cleaner, more modern design that better represents the club.

### Navigation Improvements
Reorganized the site navigation to make it easier for members and prospective members to find information about amenities, membership, and events.

### UX Enhancements
Made various usability improvements throughout the site to create a more polished experience.

## Results

- Modern, professional appearance
- Improved navigation and usability
- Better reflection of the club's quality"#.to_string(),
                    external_url: "https://oldhawthorne.com".to_string(),
                    before_url: None,
                    logo: Some("assets/portfolio/old-hawthorne-logo.png".to_string()),
                    screenshot: None,
                    video: None,
                    tech_tags: vec!["Consulting".to_string(), "UI Cleanup".to_string(), "UX".to_string()],
                    scope: vec![
                        "Replaced dated beige wallpaper background".to_string(),
                        "Improved site navigation and layout".to_string(),
                    ],
                },
                PortfolioProject {
                    id: "gracie-humaita-columbia".to_string(),
                    slug: "gracie-humaita-columbia".to_string(),
                    title: "Gracie Humaita Columbia".to_string(),
                    project_type: "Website + SMTP Integration".to_string(),
                    description: "Website for a Brazilian Jiu-Jitsu academy with integrated email automation. Designed to showcase class schedules, instructor profiles, and drive new student sign-ups with automated follow-up.".to_string(),
                    long_description: r#"## The Challenge

Gracie Humaita Columbia is a Brazilian Jiu-Jitsu academy that needed a professional web presence to attract new students and communicate with their community.

## What We Did

### Website Design
Created a mobile-first website that showcases the academy's programs, class schedules, and instructor profiles. The design reflects the Gracie Humaita brand while being accessible to beginners.

### Lead Capture
Built a lead capture system to collect information from prospective students interested in trying classes.

### Email Automation
Integrated SMTP-based email automation to automatically follow up with new leads, keeping them engaged until they come in for their first class.

## Results

- Professional web presence for the academy
- Automated lead follow-up saving staff time
- Mobile-friendly design for on-the-go access"#.to_string(),
                    external_url: "https://graciehumaitacolumbia.com".to_string(),
                    before_url: Some("https://web.archive.org/web/20190723164913/http://www.graciehumaitacolumbia.com/".to_string()),
                    logo: Some("assets/portfolio/gracie-humaita-columbia-logo.png".to_string()),
                    screenshot: None,
                    video: None,
                    tech_tags: vec!["Mobile-First".to_string(), "Lead Capture".to_string(), "SMTP".to_string()],
                    scope: vec![
                        "Automated email follow-up for new leads".to_string(),
                        "Class schedule and instructor profiles".to_string(),
                    ],
                },
                PortfolioProject {
                    id: "att-indianapolis".to_string(),
                    slug: "att-indianapolis".to_string(),
                    title: "American Top Team Indianapolis".to_string(),
                    project_type: "Website Replacement".to_string(),
                    description: "Replaced a broken, outdated website for a martial arts training facility. Built a clean, professional site with focus on easy navigation for prospective students.".to_string(),
                    long_description: r#"## The Challenge

American Top Team Indianapolis had an old, broken website that wasn't functioning properly. Prospective students couldn't find information about classes or contact the gym.

## What We Did

### Complete Replacement
Rather than trying to fix the broken site, we built a completely new website from scratch with modern technology and design.

### Professional Design
Created a clean, professional design that showcases the gym's programs and instructors. The site reflects ATT's brand while being welcoming to beginners.

### SEO Optimization
Built the site with SEO best practices to help the gym appear in local search results.

## Results

- Fully functional website replacing broken one
- Professional appearance reflecting ATT brand
- Easy navigation for prospective students
- Improved local search visibility"#.to_string(),
                    external_url: "https://attindianapolis.com".to_string(),
                    before_url: Some("https://web.archive.org/web/20200530220933/http://www.attindianapolis.com/".to_string()),
                    logo: Some("assets/portfolio/att-indianapolis-logo.png".to_string()),
                    screenshot: None,
                    video: None,
                    tech_tags: vec!["Responsive".to_string(), "SEO".to_string()],
                    scope: vec![
                        "Replaced old broken website".to_string(),
                        "Clean, professional design".to_string(),
                    ],
                },
                PortfolioProject {
                    id: "apex-earthworks".to_string(),
                    slug: "apex-earthworks".to_string(),
                    title: "APEX Earthworks".to_string(),
                    project_type: "Website + Lead Generation".to_string(),
                    description: "Business website for an earthwork and excavation company. Professional presentation with automated customer lead generation to capture and follow up with potential clients.".to_string(),
                    long_description: r#"## The Challenge

APEX Earthworks is an earthwork and excavation company that needed an online presence to attract commercial and residential clients.

## What We Did

### Professional Website
Built a business website that showcases APEX's services, equipment, and past projects. The design conveys professionalism and capability.

### Lead Generation
Implemented a lead capture system with automated follow-up to ensure no potential client falls through the cracks.

### Mobile Optimization
Ensured the site works perfectly on mobile devices, since many potential clients search for services on their phones.

## Results

- Professional online presence
- Automated lead capture and follow-up
- Mobile-friendly experience"#.to_string(),
                    external_url: "https://apexearthwork.com".to_string(),
                    before_url: None,
                    logo: Some("assets/portfolio/apex-earthworks-logo.png".to_string()),
                    screenshot: None,
                    video: None,
                    tech_tags: vec!["Lead Gen Automation".to_string(), "Mobile".to_string()],
                    scope: vec![
                        "Automated lead capture and follow-up".to_string(),
                        "Professional company showcase".to_string(),
                    ],
                },
                PortfolioProject {
                    id: "missouri-jiu-jitsu".to_string(),
                    slug: "missouri-jiu-jitsu".to_string(),
                    title: "Missouri Jiu Jitsu".to_string(),
                    project_type: "Website Development".to_string(),
                    description: "Demo website with a mock jiu-jitsu academy featuring class information, instructor bios, and signup flow automation.".to_string(),
                    long_description: r#"## The Project

Missouri Jiu Jitsu is a demo website showcasing our capabilities for martial arts academies. It demonstrates a complete solution including class schedules, instructor profiles, and member sign-up flows.

## Features

### Class Schedules
Easy-to-read class schedule showing times, instructors, and skill levels for each class.

### Instructor Bios
Professional profiles for each instructor, highlighting their background and expertise.

### Membership Inquiry
Lead capture forms for prospective students to request information or schedule a trial class.

## Technical Details

- Responsive design for all devices
- Clean, modern aesthetic
- Fast loading times
- Easy content management"#.to_string(),
                    external_url: "https://missourijiujitsu.com".to_string(),
                    before_url: None,
                    logo: Some("assets/portfolio/missouri-jiu-jitsu-logo.png".to_string()),
                    screenshot: None,
                    video: None,
                    tech_tags: vec!["Responsive".to_string(), "Forms".to_string()],
                    scope: vec![
                        "Class schedules and instructor bios".to_string(),
                        "Membership inquiry forms".to_string(),
                    ],
                },
                PortfolioProject {
                    id: "delaware-krav-maga".to_string(),
                    slug: "delaware-krav-maga".to_string(),
                    title: "Delaware Krav Maga".to_string(),
                    project_type: "Landing Page + Lead Capture".to_string(),
                    description: "Simple landing page with automated lead capture for a Krav Maga instructor's self-defense training service.".to_string(),
                    long_description: r#"## The Challenge

A Krav Maga instructor needed a simple, effective way to capture leads for his self-defense training service. No complex website needed, just a clean landing page that converts visitors into inquiries.

## What We Did

### Landing Page
Built a focused, single-page site that communicates the value of the training and drives visitors to take action.

### Automated Lead Capture
Set up an automated form that captures prospect information and delivers it directly to the instructor, no manual follow-up required.

## Results

- Clean, professional landing page
- Automated lead capture and delivery
- Simple, low-maintenance solution"#.to_string(),
                    external_url: "https://delawarekravmaga.com".to_string(),
                    before_url: None,
                    logo: Some("assets/portfolio/delaware-krav-maga-logo.png".to_string()),
                    screenshot: None,
                    video: None,
                    tech_tags: vec!["Landing Page".to_string(), "Lead Capture".to_string(), "Automation".to_string()],
                    scope: vec![
                        "Landing page design".to_string(),
                        "Automated lead capture form".to_string(),
                    ],
                },
                PortfolioProject {
                    id: "silo-wellness".to_string(),
                    slug: "silo-wellness".to_string(),
                    title: "Silo Wellness".to_string(),
                    project_type: "Website Redesign".to_string(),
                    description: "Website redesign for a wellness company offering healing retreats in Jamaica. Created an inviting, professional presence that communicates trust and tranquility to prospective guests.".to_string(),
                    long_description: r#"## The Challenge

Silo Wellness offers healing retreats in Jamaica, providing transformative wellness experiences. Their website needed to communicate trust, tranquility, and professionalism to prospective guests considering this significant investment in their wellbeing.

## What We Did

### Complete Redesign
Redesigned the website from the ground up to create an inviting, professional experience that reflects the quality of the retreats.

### Trust Building
Incorporated elements that build trust: testimonials, clear information about what to expect, and professional photography showcasing the retreat experience.

### Booking Flow
Streamlined the inquiry and booking process to make it easy for interested visitors to take the next step.

## Results

- Professional, inviting web presence
- Clear communication of retreat offerings
- Streamlined booking inquiries
- Improved trust signals throughout"#.to_string(),
                    external_url: "https://silowellness.com".to_string(),
                    before_url: None,
                    logo: Some("assets/portfolio/silo-wellness-logo.png".to_string()),
                    screenshot: None,
                    video: None,
                    tech_tags: vec!["Redesign".to_string(), "UX".to_string(), "Wellness".to_string()],
                    scope: vec![
                        "Complete website redesign".to_string(),
                        "Healing retreat showcase".to_string(),
                    ],
                },
                PortfolioProject {
                    id: "toledo-aa".to_string(),
                    slug: "toledo-aa".to_string(),
                    title: "Toledo Area AA".to_string(),
                    project_type: "Website Redesign".to_string(),
                    description: "Website redesign for the Alcoholics Anonymous organization serving the Toledo region in Ohio. Built with accessibility and ease of use as top priorities to help those seeking support.".to_string(),
                    long_description: r#"## The Challenge

Toledo Area AA serves people seeking help with alcohol addiction in the Toledo, Ohio region. The website needed to be accessible, easy to use, and welcoming to people who may be in crisis.

## What We Did

### Accessibility Focus
Built the site with accessibility as a top priority. Clear fonts, high contrast, and simple navigation ensure everyone can use the site regardless of ability.

### Meeting Finder
Created an easy-to-use meeting finder so visitors can quickly locate meetings near them by day, time, or location.

### Resource Hub
Organized resources and information in a clear, non-overwhelming way to help newcomers understand what to expect.

## Results

- Fully accessible website
- Easy meeting finder functionality
- Clear, welcoming design
- Resources organized for newcomers"#.to_string(),
                    external_url: "https://toledoaa.com".to_string(),
                    before_url: None,
                    logo: None,
                    screenshot: None,
                    video: None,
                    tech_tags: vec!["Redesign".to_string(), "Accessibility".to_string(), "Community".to_string()],
                    scope: vec![
                        "Complete website redesign".to_string(),
                        "Meeting finder and resources".to_string(),
                    ],
                },
                PortfolioProject {
                    id: "pounds-consulting".to_string(),
                    slug: "pounds-consulting".to_string(),
                    title: "Pounds Consulting".to_string(),
                    project_type: "Open Source Website".to_string(),
                    description: "This very website. Built with Rust and WebAssembly using the Dioxus framework. Open source, over-engineered with pride, and a template for anyone to use.".to_string(),
                    long_description: r#"## The Project

Yes, this is a case study about the website you're currently viewing. A consulting website built with Rust and WebAssembly. Because WordPress was too easy.

## Why WebAssembly?

Three reasons:

1. **It's blazingly fast.** Near-native performance on every device.
2. **It proves we can.** If we'll over-engineer our own website, imagine what we'll do for your actual problems.
3. **It's a conversation starter.** You're reading this, aren't you?

## Technical Details

### Framework
Built with Dioxus 0.7, a Rust-based framework that compiles to WebAssembly. The same technology that powers Figma, Google Earth, and AAA game engines.

### Features
- Full admin panel for managing articles and settings
- Data-driven portfolio with individual case study pages
- SEO optimization (sitemap, robots.txt, schema.org markup)
- SPA routing on GitHub Pages with custom 404 handling
- Dark theme with gold accents and glassmorphism design

### Open Source
The entire codebase is available on GitHub. Fork it, clone it, make it yours. The architecture is clean and the stack is modern.

## Results

- Sub-second load times
- Perfect Lighthouse scores
- A website that doubles as a portfolio piece
- Open source template for the community"#.to_string(),
                    external_url: "https://github.com/collinpounds/pounds-consulting".to_string(),
                    before_url: None,
                    logo: Some("assets/favicon.png".to_string()),
                    screenshot: None,
                    video: None,
                    tech_tags: vec!["Rust".to_string(), "WebAssembly".to_string(), "Dioxus".to_string(), "Open Source".to_string()],
                    scope: vec![
                        "Full-stack Rust/WASM website".to_string(),
                        "Admin panel with article management".to_string(),
                        "SEO optimization and SPA routing".to_string(),
                        "Open source on GitHub".to_string(),
                    ],
                },
            ],
        }
    }
}

impl Article {
    pub fn new() -> Self {
        let id = generate_id();
        Self {
            id: id.clone(),
            title: String::new(),
            slug: String::new(),
            date: chrono_today(),
            category: "General".to_string(),
            excerpt: String::new(),
            content: String::new(),
            status: ArticleStatus::Draft,
        }
    }

    /// Generate a URL-friendly slug from the title
    pub fn generate_slug(title: &str) -> String {
        title
            .to_lowercase()
            .chars()
            .map(|c| if c.is_alphanumeric() { c } else { '-' })
            .collect::<String>()
            .split('-')
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
            .join("-")
    }
}

/// Generate a simple unique ID
fn generate_id() -> String {
    #[cfg(target_arch = "wasm32")]
    {
        // Use JavaScript Date.now() for WASM
        let now = js_sys::Date::now() as u64;
        format!("{:x}", now)
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        use std::time::{SystemTime, UNIX_EPOCH};
        let duration = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default();
        format!("{:x}", duration.as_millis())
    }
}

/// Get today's date in YYYY-MM-DD format
fn chrono_today() -> String {
    // Simple date - in a real app you'd use chrono crate
    "2024-01-01".to_string()
}
```

---

## Storage Layer

**File:** `src/content/storage.rs`

```rust
use super::types::{ArticlesData, PortfolioData, SiteSettings};
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
```

---


---

*Generated on 2026-01-07 02:40:21*
