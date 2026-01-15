use crate::components::CtaSection;
use dioxus::prelude::*;

#[component]
pub fn About() -> Element {
    rsx! {
        // Hero Section
        section { class: "hero hero-short",
            div { class: "hero-content",
                h1 { class: "hero-title", "We Make Tech Less Painful" }
                p { class: "hero-subtitle",
                    "You shouldn't need a computer science degree to run your business. We handle the technical stuff so you can focus on what you're good at."
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
                            "I started Pounds Consulting because too many businesses get overcharged for confusing technology. "
                            "Whether it's your first website, a system to capture leads, or figuring out which tools you actually need, "
                            "I'm here to help you make sense of it all."
                        }
                        p {
                            "My background includes building products at Paytient (a healthcare fintech startup serving hundreds of thousands of users), "
                            "running digital marketing for Club Car Wash during their rapid expansion, and implementing enterprise integrations "
                            "for clients like Hyatt and Commerce Bank. I have a Computer Science degree and stay current with AI/ML developments "
                            "by building with tools like Claude and Llama in my spare time."
                        }
                        p {
                            "But what I enjoy most is working directly with people in my community. There's something rewarding about "
                            "helping a local gym owner get their scheduling system working, or building a website for a business "
                            "that's been growing through word-of-mouth for years and is finally ready to scale."
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
                    h3 { class: "clients-title", "Experience Across Industries" }
                    p { class: "clients-subtitle", "From local businesses to venture-backed startups" }
                    div { class: "clients-logos",
                        span { class: "client-name", "Healthcare Fintech" }
                        span { class: "client-divider", "•" }
                        span { class: "client-name", "Regional Retail" }
                        span { class: "client-divider", "•" }
                        span { class: "client-name", "Professional Services" }
                        span { class: "client-divider", "•" }
                        span { class: "client-name", "Local Small Business" }
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
                                span { class: "timeline-company", "5+ Years Building Software" }
                            }
                            ul { class: "timeline-list",
                                li { "Paytient: Healthcare fintech serving 300,000+ users. Led MFA implementation, redesigned member portal" }
                                li { "Club Car Wash: Built website and employee portal, ran Google Ads for ~3 store openings/month" }
                                li { "Enterprise clients: Integrations for Hyatt, Commerce Bank, and others" }
                                li { "Learned what works at scale, and how to keep things simple" }
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
                            h3 { "We Listen" }
                            p { "Every project starts with a conversation. What's working? What's not? What would make your life easier? We go from there." }
                        }
                        div { class: "philosophy-item",
                            span { class: "philosophy-number", "02" }
                            h3 { "We Speak Your Language" }
                            p { "No buzzwords. No alphabet soup of acronyms. Just clear explanations of what we're doing and why it matters." }
                        }
                        div { class: "philosophy-item",
                            span { class: "philosophy-number", "03" }
                            h3 { "We Don't Overdo It" }
                            p { "The goal is to solve your problem, not to build the fanciest thing possible. We recommend what actually makes sense for your situation." }
                        }
                        div { class: "philosophy-item",
                            span { class: "philosophy-number", "04" }
                            h3 { "We've Got Backup" }
                            p { "Some problems need specialists. We work with designers, security experts, and other talented pros. You deal with one person, but you get a whole network when you need it." }
                        }
                    }
                }
            }
        }

        // CTA Section
        CtaSection {
            title: "Not Sure What You Need?".to_string(),
            description: "That's fine. Most people aren't when they first reach out. Let's have a conversation and figure it out.".to_string(),
            button_text: "Schedule a Free Call".to_string(),
            use_calendar_link: true
        }
    }
}
