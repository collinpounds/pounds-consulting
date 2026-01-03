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

                    // Why I Do This
                    div { class: "timeline-item",
                        div { class: "timeline-marker" }
                        div { class: "timeline-content glass-card",
                            div { class: "timeline-header",
                                h3 { class: "timeline-title", "Why I Do This" }
                            }
                            ul { class: "timeline-list",
                                li { "Too many businesses get overcharged for confusing technology" }
                                li { "I like solving problems and explaining things in plain English" }
                                li { "Small businesses deserve the same quality work as big companies" }
                                li { "It's satisfying to build something that actually helps people" }
                            }
                        }
                    }
                }
            }
        }

        // How I Work Section
        section { class: "section philosophy-section",
            div { class: "container",
                div { class: "philosophy-content glass-card",
                    h2 { class: "philosophy-title", "How I Work" }
                    div { class: "philosophy-grid",
                        div { class: "philosophy-item",
                            span { class: "philosophy-number", "01" }
                            h3 { "I Listen First" }
                            p { "Before talking technology, I want to understand your business. What are you trying to accomplish? What's getting in the way?" }
                        }
                        div { class: "philosophy-item",
                            span { class: "philosophy-number", "02" }
                            h3 { "I Explain Everything" }
                            p { "No jargon. No confusing tech-speak. If you don't understand something, that's my fault, not yours." }
                        }
                        div { class: "philosophy-item",
                            span { class: "philosophy-number", "03" }
                            h3 { "I Keep It Simple" }
                            p { "The best solution is usually the simplest one. I won't sell you something complicated when something simple will work." }
                        }
                        div { class: "philosophy-item",
                            span { class: "philosophy-number", "04" }
                            h3 { "I Know People" }
                            p { "Need a designer? A security expert? I have a network of brilliant people I can call. Whatever the problem, we can figure it out." }
                        }
                    }
                }
            }
        }

        // CTA Section
        CtaSection {
            title: "Not Sure Where to Start?".to_string(),
            description: "That's okay. Most people who call me aren't sure what they need yet. Let's just talk and figure it out together.".to_string(),
            button_text: "Schedule a Free Call".to_string(),
            use_calendar_link: true
        }
    }
}
