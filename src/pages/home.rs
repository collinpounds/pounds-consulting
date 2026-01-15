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
                    "We build websites that convert visitors into customers. We automate the manual work eating up your week. We help you make smart technology decisions before you spend money on the wrong thing. Direct communication. Honest advice. Work that lasts."
                }
                a {
                    href: "https://calendar.app.google/LNasBbmDr8LXNEuu5",
                    target: "_blank",
                    rel: "noopener noreferrer",
                    class: "btn btn-primary btn-large",
                    "Book a Free Call"
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
                        "A slow website costs you 7% in conversions for every extra second it takes to load. Manual processes that could be automated are eating 20+ hours of your week. The wrong technology decision can waste months and thousands of dollars."
                    }
                    p {
                        "We fix these problems. We build fast websites that rank well and convert visitors. We automate the repetitive work that's draining your team. We help you make smart decisions before you commit."
                    }
                    p {
                        "No jargon. No overcomplicated solutions. Just clean work that delivers results."
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
                        title: "Web Development".to_string(),
                        description: "Fast websites that rank in Google, work on every device, and convert visitors into paying customers. Custom built for your business.".to_string(),
                        icon: "globe".to_string()
                    }
                    ServiceCard {
                        title: "Digital Marketing".to_string(),
                        description: "Email sequences that nurture leads while you sleep. SMS reminders that reduce no-shows. Customer systems that run themselves.".to_string(),
                        icon: "megaphone".to_string()
                    }
                    ServiceCard {
                        title: "Technical Strategy".to_string(),
                        description: "Get the decision right before you spend the money. Vendor evaluation, build vs buy analysis, architecture reviews.".to_string(),
                        icon: "compass".to_string()
                    }
                    ServiceCard {
                        title: "Business Solutions".to_string(),
                        description: "Connect your tools with Zapier and custom integrations. Automate the repetitive tasks eating up your team's time.".to_string(),
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
                        h3 { class: "why-title", "Simple solutions" }
                        p { class: "why-description",
                            "We recommend what works, not what sounds impressive. Sometimes the answer is a simple website, not a custom app. We'll tell you."
                        }
                    }
                    div { class: "why-card glass-card",
                        div { class: "why-icon",
                            Icon { name: IconName::Handshake, size: 40, color: "var(--color-secondary)".to_string() }
                        }
                        h3 { class: "why-title", "Reliable communication" }
                        p { class: "why-description",
                            "Emails answered within one business day. No ghosting. No surprises. You'll always know where your project stands."
                        }
                    }
                    div { class: "why-card glass-card",
                        div { class: "why-icon",
                            Icon { name: IconName::TrendingUp, size: 40, color: "var(--color-secondary)".to_string() }
                        }
                        h3 { class: "why-title", "Proven experience" }
                        p { class: "why-description",
                            "Healthcare fintech serving hundreds of thousands of users. Regional businesses opening 3 locations per month. We've built systems that scale."
                        }
                    }
                    div { class: "why-card glass-card",
                        div { class: "why-icon",
                            Icon { name: IconName::Brain, size: 40, color: "var(--color-secondary)".to_string() }
                        }
                        h3 { class: "why-title", "Access to experts" }
                        p { class: "why-description",
                            "Need a designer? A security specialist? An AI expert? We have a network of brilliant people we can call. Whatever the challenge, we know someone who's solved it."
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
