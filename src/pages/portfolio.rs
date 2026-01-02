use dioxus::prelude::*;
use crate::components::CtaSection;

#[component]
pub fn Portfolio() -> Element {
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
                    // Club Car Wash
                    div { class: "portfolio-card glass-card",
                        div { class: "portfolio-header",
                            h3 { class: "portfolio-title", "Club Car Wash" }
                            span { class: "portfolio-type", "Website + Portal + Marketing" }
                        }
                        p { class: "portfolio-description",
                            "Complete digital transformation for a growing regional car wash chain. Built and managed the public-facing website, developed an internal employee portal, and ran ongoing Google Ads campaigns to support new store openings."
                        }
                        div { class: "portfolio-tech",
                            span { class: "tech-tag", "React" }
                            span { class: "tech-tag", "Custom CMS" }
                            span { class: "tech-tag", "Google Ads" }
                        }
                        ul { class: "portfolio-scope",
                            li { "Public website design and development" }
                            li { "Employee portal for internal operations" }
                            li { "Google Ad campaign management (~3 store openings/month)" }
                            li { "Ongoing maintenance and support for 1 year" }
                        }
                    }

                    // Old Hawthorne
                    div { class: "portfolio-card glass-card",
                        div { class: "portfolio-header",
                            h3 { class: "portfolio-title", "Old Hawthorne Country Club" }
                            span { class: "portfolio-type", "Website Development" }
                        }
                        p { class: "portfolio-description",
                            "Professional website for an upscale country club community in Columbia, Missouri. Clean, elegant design reflecting the club's brand with member-focused functionality."
                        }
                        div { class: "portfolio-tech",
                            span { class: "tech-tag", "HTML/CSS/JS" }
                            span { class: "tech-tag", "Responsive" }
                            span { class: "tech-tag", "SEO" }
                        }
                        a { href: "https://oldhawthorne.com", target: "_blank", class: "portfolio-link",
                            "Visit Site →"
                        }
                    }

                    // Gracie Humaita Columbia
                    div { class: "portfolio-card glass-card",
                        div { class: "portfolio-header",
                            h3 { class: "portfolio-title", "Gracie Humaita Columbia" }
                            span { class: "portfolio-type", "Website Development" }
                        }
                        p { class: "portfolio-description",
                            "Website for a Brazilian Jiu-Jitsu academy. Designed to showcase class schedules, instructor profiles, and drive new student sign-ups."
                        }
                        div { class: "portfolio-tech",
                            span { class: "tech-tag", "Mobile-First" }
                            span { class: "tech-tag", "Lead Capture" }
                        }
                        a { href: "https://graciehumaitacolumbia.com", target: "_blank", class: "portfolio-link",
                            "Visit Site →"
                        }
                    }

                    // ATT Indianapolis
                    div { class: "portfolio-card glass-card",
                        div { class: "portfolio-header",
                            h3 { class: "portfolio-title", "ATT Indianapolis" }
                            span { class: "portfolio-type", "Website Development" }
                        }
                        p { class: "portfolio-description",
                            "Professional website build for a martial arts training facility. Focus on clean presentation and easy navigation for prospective students."
                        }
                        div { class: "portfolio-tech",
                            span { class: "tech-tag", "Responsive" }
                            span { class: "tech-tag", "SEO" }
                        }
                        a { href: "https://attindianapolis.com", target: "_blank", class: "portfolio-link",
                            "Visit Site →"
                        }
                    }

                    // Apex Earthwork
                    div { class: "portfolio-card glass-card",
                        div { class: "portfolio-header",
                            h3 { class: "portfolio-title", "Apex Earthwork" }
                            span { class: "portfolio-type", "Website Development" }
                        }
                        p { class: "portfolio-description",
                            "Business website for an earthwork and excavation company. Professional presentation designed to generate leads and showcase capabilities."
                        }
                        div { class: "portfolio-tech",
                            span { class: "tech-tag", "Lead Gen" }
                            span { class: "tech-tag", "Mobile" }
                        }
                        a { href: "https://apexearthwork.com", target: "_blank", class: "portfolio-link",
                            "Visit Site →"
                        }
                    }

                    // Missouri Jiu Jitsu
                    div { class: "portfolio-card glass-card",
                        div { class: "portfolio-header",
                            h3 { class: "portfolio-title", "Missouri Jiu Jitsu" }
                            span { class: "portfolio-type", "Website Development" }
                        }
                        p { class: "portfolio-description",
                            "Website for a jiu-jitsu academy featuring class information, instructor bios, and membership inquiry functionality."
                        }
                        div { class: "portfolio-tech",
                            span { class: "tech-tag", "Responsive" }
                            span { class: "tech-tag", "Forms" }
                        }
                        a { href: "https://missourijiujitsu.com", target: "_blank", class: "portfolio-link",
                            "Visit Site →"
                        }
                    }

                    // Delaware Krav Maga
                    div { class: "portfolio-card glass-card",
                        div { class: "portfolio-header",
                            h3 { class: "portfolio-title", "Delaware Krav Maga" }
                            span { class: "portfolio-type", "Website Development" }
                        }
                        p { class: "portfolio-description",
                            "Professional website for a Krav Maga self-defense training center. Clear presentation of programs with strong calls-to-action for new students."
                        }
                        div { class: "portfolio-tech",
                            span { class: "tech-tag", "Mobile-First" }
                            span { class: "tech-tag", "Lead Capture" }
                        }
                        a { href: "https://delawarekravmaga.com", target: "_blank", class: "portfolio-link",
                            "Visit Site →"
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
