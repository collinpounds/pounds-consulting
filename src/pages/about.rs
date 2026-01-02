use dioxus::prelude::*;
use crate::components::CtaSection;

#[component]
pub fn About() -> Element {
    rsx! {
        // Hero Section
        section { class: "hero hero-short",
            div { class: "hero-content",
                h1 { class: "hero-title", "Built on Experience, Driven by Results" }
                p { class: "hero-subtitle",
                    "Pounds Consulting was founded on a simple idea: people deserve technical partners who actually understand their problems."
                }
            }
        }

        // Story Section
        section { class: "section story-section",
            div { class: "container",
                div { class: "story-content glass-card",
                    p {
                        "Before this, I spent 4 years at Paytient, a technology company at THE crossroad of healthcare and finance, where I worked my way from product support to leading technical product initiatives. I've evaluated identity verification platforms, managed complex software roadmaps, and helped develop software and financial products used by hundreds of thousands of people."
                    }
                    p {
                        "Along the way, I learned something important: most technical problems aren't actually that complicated. They just need someone willing to listen, apply common sense, and have agency without overcomplicating things."
                    }
                    p {
                        "So that's what I do now. I help businesses build websites, solve technical challenges, and make better decisions about technology. I keep things simple, intuitive, and user-friendly. No massive team. Just direct access to someone who's done the work and knows how to get it completed quickly and exceeding high expectations."
                    }
                    p { class: "story-signature", "- Collin P." }
                }
            }
        }

        // Values Section
        section { class: "section values-section",
            div { class: "container",
                h2 { class: "section-title", "How We Work" }
                div { class: "values-grid",
                    div { class: "value-card glass-card",
                        div { class: "value-icon", "✨" }
                        h3 { class: "value-title", "Keep it simple." }
                        p { class: "value-description",
                            "Every project starts with understanding what you actually need—not what sounds impressive. We build solutions that are intuitive to use, easy to maintain, and powerful enough to grow with you."
                        }
                    }
                    div { class: "value-card glass-card",
                        div { class: "value-icon", "🎯" }
                        h3 { class: "value-title", "Ownership Mentality" }
                        p { class: "value-description",
                            "Your project gets treated like it's our own. That means proactive communication, honest feedback (even when it's uncomfortable), and a genuine investment in your success."
                        }
                    }
                    div { class: "value-card glass-card",
                        div { class: "value-icon", "🏗️" }
                        h3 { class: "value-title", "Built to last." }
                        p { class: "value-description",
                            "We don't build throwaway work. Every website, every system, every recommendation is designed to serve you well for years, not just until the invoice clears."
                        }
                    }
                }
            }
        }

        // Experience Section
        section { class: "section experience-section",
            div { class: "container",
                h2 { class: "section-title", "Background" }
                div { class: "experience-grid",
                    div { class: "experience-card glass-card",
                        h3 { class: "experience-title", "Product & Technology" }
                        ul { class: "experience-list",
                            li { "Product Technical Support and Product Management at Paytient (healthcare fintech)" }
                            li { "Led vendor evaluations for identity verification platforms including Plaid, Persona, and Alloy" }
                            li { "Reduced user onboarding time by 17% through systematic UX improvements" }
                            li { "Managed technical implementations in HIPAA-compliant environments" }
                            li { "Designed, developed, and managed Club Car Wash public website and Employee Portal for 1 year" }
                            li { "Managed Google Ad campaigns for ~3 store openings a month" }
                            li { "Created websites for graciehumaitacolumbia.com, oldhawthorne.com, attindianapolis.com, apexearthwork.com, missourijiujitsu.com, delawarekravmaga.com, and more" }
                        }
                    }
                    div { class: "experience-card glass-card",
                        h3 { class: "experience-title", "Development" }
                        ul { class: "experience-list",
                            li { "React and React Native web and mobile development" }
                            li { "Rust and Dioxus web/mobile development" }
                            li { "Custom website builds for businesses ranging from local car washes to country clubs" }
                            li { "Database design, API integrations, and workflow automation" }
                        }
                    }
                    div { class: "experience-card glass-card",
                        h3 { class: "experience-title", "Industries" }
                        ul { class: "experience-list",
                            li { "Healthcare technology" }
                            li { "Financial services and compliance" }
                            li { "Professional services" }
                            li { "Small/Medium business operations" }
                        }
                    }
                }
            }
        }

        // CTA Section
        CtaSection {
            title: "Let's Work Together".to_string(),
            description: "If you're looking for a technical partner who will treat your business like it matters (because it does), I'd like to hear from you.".to_string(),
            button_text: "Book a time.".to_string(),
            use_calendar_link: true
        }
    }
}
