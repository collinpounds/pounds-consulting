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
                                            alt: "{project.title} logo",
                                            width: "180",
                                            height: "180",
                                            loading: "lazy"
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
