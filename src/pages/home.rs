use dioxus::prelude::*;
use crate::Route;
use crate::components::{CtaSection, ServiceCard};

#[component]
pub fn Home() -> Element {
    rsx! {
        // Hero Section
        section { class: "hero",
            div { class: "hero-content",
                h1 { class: "hero-title", "We Can Help" }
                p { class: "hero-subtitle",
                    "Technical consulting for businesses that want solutions, not complexity. Based in Missouri, serving clients nationwide."
                }
                a {
                    href: "https://calendar.app.google/LNasBbmDr8LXNEuu5",
                    target: "_blank",
                    rel: "noopener noreferrer",
                    class: "btn btn-primary btn-large",
                    "Let's Meet"
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
                        "Most businesses don't need more software. They need the right software, built correctly, with someone who keeps things simple and user-friendly."
                    }
                    p {
                        "Pounds Consulting exists to bridge the gap between your business goals and the technical work required to get there. No jargon. No overengineered solutions. Just common sense and clean execution."
                    }
                    p {
                        "Whether you're launching a new product, fixing something that's broken, or trying to figure out what to build next—we can help you get there faster and with less headache."
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
                        description: "Build a web presence that actually works. From simple business sites to custom web applications, we design and develop solutions that load fast, look sharp, and convert visitors into customers.".to_string(),
                        icon: "🌐".to_string()
                    }
                    ServiceCard {
                        title: "Digital Marketing".to_string(),
                        description: "Build a web presence without the mess. We manage Email, SMS, Phone, Customer Intake, Scheduling, Membership. We build the digital marketing structure and then integrate the funnel directly into your operation.".to_string(),
                        icon: "📱".to_string()
                    }
                    ServiceCard {
                        title: "Technical Strategy".to_string(),
                        description: "Not sure what you need? We help you figure it out. Product roadmaps, technology evaluations, vendor selection—you get an expert perspective before you commit resources.".to_string(),
                        icon: "🎯".to_string()
                    }
                    ServiceCard {
                        title: "Business Solutions".to_string(),
                        description: "Software integrations, workflow automation, data migrations. We solve the technical problems that slow your business down so you can focus on growth.".to_string(),
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
                        h3 { class: "why-title", "Keep it simple." }
                        p { class: "why-description",
                            "Every recommendation we make is grounded in what actually works for your situation, not what sounds impressive on a slide deck."
                        }
                    }
                    div { class: "why-card glass-card",
                        div { class: "why-icon", "🤝" }
                        h3 { class: "why-title", "Midwest Work Ethic" }
                        p { class: "why-description",
                            "We show up, we communicate, and we deliver what we promise."
                        }
                    }
                    div { class: "why-card glass-card",
                        div { class: "why-icon", "📈" }
                        h3 { class: "why-title", "Real Experience" }
                        p { class: "why-description",
                            "Five years solving complex problems in healthcare technology, financial systems, and compliance-heavy environments. We've seen what works and what doesn't."
                        }
                    }
                }
            }
        }

        // CTA Section
        CtaSection {
            title: "Ready to Solve Something?".to_string(),
            description: "Let's have a conversation about what you're building and how we can help.".to_string(),
            button_text: "Schedule a Call".to_string(),
            use_calendar_link: true
        }
    }
}
