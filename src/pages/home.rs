use crate::components::{CtaSection, Icon, IconName, ServiceCard};
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
                div { class: "services-grid centered-grid",
                    ServiceCard {
                        title: "Website Development".to_string(),
                        description: "We build websites that load fast, look professional, and turn visitors into customers. No templates. No bloat. Just clean, custom work.".to_string(),
                        icon: "globe".to_string()
                    }
                    ServiceCard {
                        title: "Digital Marketing".to_string(),
                        description: "Email, SMS, scheduling, customer intake, memberships. We set up the systems that bring in leads and keep customers coming back.".to_string(),
                        icon: "megaphone".to_string()
                    }
                    ServiceCard {
                        title: "Technical Strategy".to_string(),
                        description: "Not sure what you need? We help you figure it out before you spend money on the wrong thing. Honest advice, no sales pitch.".to_string(),
                        icon: "target".to_string()
                    }
                    ServiceCard {
                        title: "Business Solutions".to_string(),
                        description: "Connect your software, automate repetitive tasks, move data between systems. We fix the technical problems that slow you down.".to_string(),
                        icon: "zap".to_string()
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
                div { class: "why-grid centered-grid",
                    div { class: "why-card glass-card",
                        div { class: "why-icon",
                            Icon { name: IconName::Star, size: 40, color: "var(--color-secondary)".to_string() }
                        }
                        h3 { class: "why-title", "We keep it simple" }
                        p { class: "why-description",
                            "Every recommendation we make is based on what actually works for your situation, not what sounds impressive."
                        }
                    }
                    div { class: "why-card glass-card",
                        div { class: "why-icon",
                            Icon { name: IconName::Handshake, size: 40, color: "var(--color-secondary)".to_string() }
                        }
                        h3 { class: "why-title", "We show up" }
                        p { class: "why-description",
                            "We answer emails. We meet deadlines. We tell you the truth, even when it's not what you want to hear."
                        }
                    }
                    div { class: "why-card glass-card",
                        div { class: "why-icon",
                            Icon { name: IconName::TrendingUp, size: 40, color: "var(--color-secondary)".to_string() }
                        }
                        h3 { class: "why-title", "We've done this before" }
                        p { class: "why-description",
                            "Five years of building software in healthcare, finance, and small business. We know what works and what doesn't."
                        }
                    }
                    div { class: "why-card glass-card",
                        div { class: "why-icon",
                            Icon { name: IconName::Brain, size: 40, color: "var(--color-secondary)".to_string() }
                        }
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
