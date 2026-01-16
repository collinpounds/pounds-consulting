use crate::components::{parse_icon_name, CtaSection, Icon, IconName};
use crate::content::{load_services, load_settings};
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Services() -> Element {
    let settings = load_settings();
    let discount = &settings.discount;
    let services_data = load_services();

    rsx! {
        // Hero Section with animated background
        section { class: "hero hero-services",
            div { class: "hero-bg-animation" }
            div { class: "hero-content",
                h1 { class: "hero-title hero-title-animated", "Technical Solutions for Every Challenge" }
                p { class: "hero-subtitle",
                    "From AI strategy to web development, mobile apps to business automation. Whatever technical problem you're facing, we can help solve it."
                }
                a {
                    href: "https://calendar.app.google/NxuWY3RDGE5Miaan7",
                    target: "_blank",
                    rel: "noopener noreferrer",
                    class: "btn btn-primary btn-large btn-pulse",
                    "Book a Free Discovery Call"
                }
            }
        }

        // Services Grid Section
        section { class: "section services-grid-section",
            div { class: "container",
                div { class: "services-grid",
                    for (i, service) in services_data.services.iter().enumerate() {
                        Link {
                            key: "{service.id}",
                            to: Route::ServiceDetail { slug: service.slug.clone() },
                            class: "service-card-link",
                            style: "--animation-delay: {i * 100}ms",

                            div {
                                class: "service-card-new glass-card",
                                style: "--service-accent: {service.accent_color}",

                                div { class: "service-card-accent-bar" }

                                div { class: "service-card-header",
                                    span { class: "service-card-icon",
                                        if let Some(icon_name) = parse_icon_name(&service.icon) {
                                            Icon { name: icon_name, size: 32, color: "var(--service-accent, var(--color-secondary))".to_string() }
                                        }
                                    }
                                    h3 { class: "service-card-title", "{service.title}" }
                                }

                                p { class: "service-card-tagline", "{service.tagline}" }

                                p { class: "service-card-description",
                                    {service.description.chars().take(150).collect::<String>()}
                                    if service.description.len() > 150 { "..." }
                                }

                                div { class: "service-card-footer",
                                    span { class: "service-card-cta", "Learn More" }
                                    span { class: "service-card-arrow",
                                        Icon { name: IconName::ArrowRight, size: 20, color: "var(--service-accent, var(--color-secondary))".to_string() }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // Why Choose Us Section
        section { class: "section why-choose-section",
            div { class: "container",
                h2 { class: "section-title", "Why Work With Us" }
                div { class: "why-choose-grid centered-grid",
                    div { class: "why-card glass-card",
                        style: "--animation-delay: 0ms",
                        div { class: "why-icon",
                            Icon { name: IconName::Target, size: 40, color: "var(--color-secondary)".to_string() }
                        }
                        h3 { "One Point of Contact" }
                        p { "No account managers or layers of bureaucracy. You work directly with me, someone who understands both the business and technical sides." }
                    }
                    div { class: "why-card glass-card",
                        style: "--animation-delay: 100ms",
                        div { class: "why-icon",
                            Icon { name: IconName::Users, size: 40, color: "var(--color-secondary)".to_string() }
                        }
                        h3 { "Network of Experts" }
                        p { "Need a designer? A security specialist? A data scientist? I have a network of brilliant people I can bring in for any challenge." }
                    }
                    div { class: "why-card glass-card",
                        style: "--animation-delay: 200ms",
                        div { class: "why-icon",
                            Icon { name: IconName::MessageCircle, size: 40, color: "var(--color-secondary)".to_string() }
                        }
                        h3 { "Honest Advice" }
                        p { "Sometimes the answer is 'you don't need this' or 'use a cheaper solution.' I'll tell you the truth, even when it means less work for me." }
                    }
                    div { class: "why-card glass-card",
                        style: "--animation-delay: 300ms",
                        div { class: "why-icon",
                            Icon { name: IconName::Zap, size: 40, color: "var(--color-secondary)".to_string() }
                        }
                        h3 { "Fast Response" }
                        p { "Emails answered within one business day. Usually faster. No waiting weeks for a callback or getting lost in a ticketing system." }
                    }
                }
            }
        }

        // How We Work Section
        section { class: "section pricing-section",
            div { class: "container",
                h2 { class: "section-title", "How We Work Together" }
                div { class: "pricing-content glass-card",
                    div { class: "discovery-highlight",
                        div { class: "discovery-badge",
                            Icon { name: IconName::Calendar, size: 32, color: "var(--color-secondary)".to_string() }
                        }
                        h3 { class: "discovery-title", "Start With a Free Discovery Call" }
                        p { class: "discovery-description",
                            "Every project starts with a conversation. We'll discuss your goals, challenges, and what success looks like for you."
                        }
                    }

                    div { class: "promise-box",
                        h4 { class: "promise-title", "Our Promise" }
                        p { class: "promise-text",
                            "You'll receive a complete quote with a clear delivery timeline before we ask for a single dollar. No surprises. No pressure. Just honest numbers so you can make the right decision for your business."
                        }
                    }

                    div { class: "pricing-actions",
                        a {
                            href: "https://calendar.app.google/NxuWY3RDGE5Miaan7",
                            target: "_blank",
                            rel: "noopener noreferrer",
                            class: "btn btn-primary btn-large btn-pulse",
                            "Book Your Free Call"
                        }
                        p { class: "no-obligation", "No credit card. No obligation. Just a conversation." }
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
                                    strong { "Custom Quote" }
                                    p { "Detailed proposal with exact pricing and timeline" }
                                }
                            }
                            div { class: "process-step",
                                span { class: "step-number", "3" }
                                div { class: "step-content",
                                    strong { "You Decide" }
                                    p { "Review everything, ask questions, no pressure" }
                                }
                            }
                            div { class: "process-step",
                                span { class: "step-number", "4" }
                                div { class: "step-content",
                                    strong { "We Build" }
                                    p { "Regular updates until delivery and handoff" }
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
                        div { class: "first-responder-badge",
                            Icon { name: IconName::Award, size: 48, color: "var(--color-secondary)".to_string() }
                        }
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
