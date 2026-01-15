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
                                    alt: "{project.title} logo",
                                    width: "250",
                                    height: "250"
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
                                    playsinline: true,
                                    width: "1200",
                                    height: "675"
                                }
                            } else if let Some(screenshot) = &project.screenshot {
                                img {
                                    class: "case-study-screenshot",
                                    src: "/{screenshot}",
                                    alt: "{project.title} screenshot",
                                    width: "1200",
                                    height: "675",
                                    loading: "lazy"
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
                                                alt: "{other.title} logo",
                                                width: "180",
                                                height: "180",
                                                loading: "lazy"
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
