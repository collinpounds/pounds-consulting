use crate::components::{parse_icon_name, Icon, IconName};
use crate::content::load_services;
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn ServiceDetail(slug: String) -> Element {
    let services_data = load_services();
    let service = services_data.services.iter().find(|s| s.slug == slug);

    match service {
        Some(service) => {
            rsx! {
                // Service Hero with accent color
                section {
                    class: "service-hero",
                    style: "--service-accent: {service.accent_color}",

                    div { class: "container",
                        Link { to: Route::Services {}, class: "service-back-link",
                            Icon { name: IconName::ArrowLeft, size: 16, color: "currentColor".to_string() }
                            " All Services"
                        }

                        div { class: "service-hero-content",
                            span { class: "service-hero-icon",
                                if let Some(icon_name) = parse_icon_name(&service.icon) {
                                    Icon { name: icon_name, size: 64, color: "var(--service-accent, var(--color-secondary))".to_string() }
                                }
                            }
                            h1 { class: "service-hero-title", "{service.title}" }
                            p { class: "service-hero-tagline", "{service.tagline}" }
                        }
                    }
                }

                // Main Description
                section { class: "section service-intro-section",
                    div { class: "container",
                        div { class: "service-intro glass-card",
                            p { class: "service-intro-text", "{service.description}" }
                        }
                    }
                }

                // Features Section
                section { class: "section service-features-section",
                    div { class: "container",
                        h2 { class: "section-title", "What's Included" }
                        div { class: "service-features-grid",
                            for (i, feature) in service.features.iter().enumerate() {
                                div {
                                    key: "{i}",
                                    class: "service-feature-item",
                                    style: "--animation-delay: {i * 100}ms; --service-accent: {service.accent_color}",
                                    span { class: "feature-check",
                                        Icon { name: IconName::Check, size: 20, color: "var(--service-accent, var(--color-secondary))".to_string() }
                                    }
                                    span { class: "feature-text", "{feature}" }
                                }
                            }
                        }
                    }
                }

                // Long Description Content
                section { class: "section service-content-section",
                    div { class: "container",
                        div { class: "service-content glass-card",
                            for paragraph in service.long_description.split("\n\n") {
                                if paragraph.starts_with("### ") {
                                    h3 { class: "service-h3", {paragraph.trim_start_matches("### ")} }
                                } else if paragraph.starts_with("## ") {
                                    // Change "What We Do" to "How We Can Help"
                                    {
                                        let heading = paragraph.trim_start_matches("## ");
                                        let display_heading = if heading == "What We Do" {
                                            "How We Can Help"
                                        } else {
                                            heading
                                        };
                                        rsx! { h2 { class: "service-h2", "{display_heading}" } }
                                    }
                                } else if paragraph.starts_with("- ") || paragraph.starts_with("* ") {
                                    ul { class: "service-list",
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
                    }
                }

                // Persona Cards Section
                section { class: "section service-personas-section",
                    div { class: "container",
                        h2 { class: "section-title", "Who This Is For" }
                        div { class: "persona-cards-grid centered-grid",
                            for (i, persona) in service.personas.iter().enumerate() {
                                div {
                                    key: "{i}",
                                    class: "persona-card glass-card",
                                    style: "--animation-delay: {i * 150}ms; --service-accent: {service.accent_color}",
                                    div { class: "persona-icon",
                                        if let Some(icon_name) = parse_icon_name(&persona.icon) {
                                            Icon { name: icon_name, size: 48, color: "var(--service-accent, var(--color-secondary))".to_string() }
                                        }
                                    }
                                    h3 { class: "persona-title", "{persona.title}" }
                                    p { class: "persona-description", "{persona.description}" }
                                }
                            }
                        }
                    }
                }

                // Book CTA
                section { class: "section service-book-section",
                    div { class: "container",
                        div {
                            class: "service-book-card glass-card",
                            style: "--service-accent: {service.accent_color}",
                            h2 { "Ready to get started?" }
                            p { "Let's talk about how {service.title} can help your business." }
                            a {
                                href: "https://calendar.app.google/NxuWY3RDGE5Miaan7",
                                target: "_blank",
                                rel: "noopener noreferrer",
                                class: "btn btn-primary btn-large service-cta-btn",
                                "Book a Free Call"
                            }
                        }
                    }
                }

                // Other Services
                section { class: "section related-services",
                    div { class: "container",
                        h2 { class: "section-title", "Explore Other Services" }
                        div { class: "services-mini-grid centered-grid",
                            for other in services_data.services.iter()
                                .filter(|s| s.slug != slug)
                                .take(3)
                            {
                                Link {
                                    key: "{other.id}",
                                    to: Route::ServiceDetail { slug: other.slug.clone() },
                                    class: "service-mini-card glass-card",
                                    style: "--service-accent: {other.accent_color}",

                                    span { class: "service-mini-icon",
                                        if let Some(icon_name) = parse_icon_name(&other.icon) {
                                            Icon { name: icon_name, size: 28, color: "var(--service-accent, var(--color-secondary))".to_string() }
                                        }
                                    }
                                    h3 { class: "service-mini-title", "{other.title}" }
                                    span { class: "service-mini-arrow",
                                        Icon { name: IconName::ArrowRight, size: 20, color: "var(--service-accent, var(--color-secondary))".to_string() }
                                    }
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
                        h1 { class: "hero-title", "Service Not Found" }
                        p { class: "hero-subtitle", "The service you're looking for doesn't exist." }
                        Link { to: Route::Services {}, class: "btn btn-primary", "View All Services" }
                    }
                }
            }
        }
    }
}
