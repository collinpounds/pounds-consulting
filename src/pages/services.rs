use crate::components::CtaSection;
use dioxus::prelude::*;

#[component]
pub fn Services() -> Element {
    rsx! {
        // Hero Section
        section { class: "hero hero-short",
            div { class: "hero-content",
                h1 { class: "hero-title", "Services Built Around Your Needs" }
                p { class: "hero-subtitle",
                    "From website builds to complex technical strategy, we offer flexible consulting services designed to solve real problems not make expensive busywork."
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
                        "A website is just the beginning. To grow, you need systems that capture leads, nurture relationships, and convert interest into revenue—without requiring you to manually manage every touchpoint."
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
                        "We build websites that do exactly that. No bloated templates. No cookie-cutter designs. Every site is crafted with your specific goals in mind—whether that's generating leads, showcasing your work, or selling products online. User-friendly by default."
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
                        "Initial conversations are always free. We want to understand a problem in its entirety before taking payment."
                    }
                    div { class: "pricing-rate",
                        span { class: "rate-amount", "$100" }
                        span { class: "rate-period", "/hour" }
                    }
                    p { class: "pricing-description",
                        "No hidden fees. No surprise charges. You'll always know what you're paying for and why."
                    }
                    p { class: "pricing-description",
                        "For larger projects, we can provide fixed-price quotes after a few discovery conversations. Either way, you'll have complete clarity on costs before any work begins."
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

        // CTA Section
        CtaSection {
            title: "Not Sure What You Need?".to_string(),
            description: "That's okay. Most clients come to us with a problem, not a predefined solution. Let's talk through what you're trying to accomplish and figure out the right path forward together.".to_string(),
            button_text: "Start a Conversation".to_string(),
            use_calendar_link: true
        }
    }
}
