use crate::components::CtaSection;
use dioxus::prelude::*;

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
                    // Paytient
                    div { class: "portfolio-card glass-card",
                        div { class: "portfolio-header",
                            h3 { class: "portfolio-title", "Paytient" }
                            span { class: "portfolio-type", "Product Development" }
                        }
                        p { class: "portfolio-description",
                            "Contributed to a healthcare fintech startup serving hundreds of thousands of users. Removed friction from the onboarding flow, redesigned the my.paytient.com landing page, and led a team of 6 engineers implementing multi-factor authentication."
                        }
                        div { class: "portfolio-tech",
                            span { class: "tech-tag", "React" }
                            span { class: "tech-tag", "UX" }
                            span { class: "tech-tag", "MFA" }
                            span { class: "tech-tag", "Team Lead" }
                        }
                        ul { class: "portfolio-scope",
                            li { "Streamlined onboarding by removing unnecessary friction step" }
                            li { "Redesigned member landing page for better engagement" }
                            li { "Led 6-person team implementing MFA across the platform" }
                        }
                        div { class: "portfolio-actions",
                            a { href: "https://my.paytient.com", target: "_blank", rel: "noopener noreferrer", class: "btn btn-primary",
                                "Visit Site →"
                            }
                        }
                    }

                    // Club Car Wash
                    a { href: "https://clubcarwash.com", target: "_blank", rel: "noopener noreferrer", class: "portfolio-card-link",
                        div { class: "portfolio-card glass-card",
                            div { class: "portfolio-header",
                                h3 { class: "portfolio-title", "Club Car Wash" }
                                span { class: "portfolio-type", "Website + Portal + Digital Marketing" }
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
                            span { class: "portfolio-link btn btn-primary",
                                "Visit Site →"
                            }
                        }
                    }

                    // Old Hawthorne
                    a { href: "https://oldhawthorne.com", target: "_blank", rel: "noopener noreferrer", class: "portfolio-card-link",
                        div { class: "portfolio-card glass-card",
                            div { class: "portfolio-header",
                                h3 { class: "portfolio-title", "Old Hawthorne Country Club" }
                                span { class: "portfolio-type", "Website Consulting" }
                            }
                            p { class: "portfolio-description",
                                "Consulting work for a local country club community in Columbia, Missouri. Made targeted adjustments to improve the site's look and navigation, including replacing the dated beige wallpaper background with a cleaner design."
                            }
                            div { class: "portfolio-tech",
                                span { class: "tech-tag", "Consulting" }
                                span { class: "tech-tag", "UI Cleanup" }
                                span { class: "tech-tag", "UX" }
                            }
                            ul { class: "portfolio-scope",
                                li { "Replaced dated beige wallpaper background" }
                                li { "Improved site navigation and layout" }
                            }
                            span { class: "portfolio-link btn btn-primary",
                                "Visit Site →"
                            }
                        }
                    }

                    // Gracie Humaita Columbia
                    div { class: "portfolio-card glass-card",
                        div { class: "portfolio-header",
                            h3 { class: "portfolio-title", "Gracie Humaita Columbia" }
                            span { class: "portfolio-type", "Website + SMTP Integration" }
                        }
                        p { class: "portfolio-description",
                            "Website for a Brazilian Jiu-Jitsu academy with integrated email automation. Designed to showcase class schedules, instructor profiles, and drive new student sign-ups with automated follow-up."
                        }
                        div { class: "portfolio-tech",
                            span { class: "tech-tag", "Mobile-First" }
                            span { class: "tech-tag", "Lead Capture" }
                            span { class: "tech-tag", "SMTP" }
                        }
                        ul { class: "portfolio-scope",
                            li { "Automated email follow-up for new leads" }
                            li { "Class schedule and instructor profiles" }
                        }
                        div { class: "portfolio-actions",
                            a { href: "https://graciehumaitacolumbia.com", target: "_blank", rel: "noopener noreferrer", class: "btn btn-primary",
                                "Visit Site →"
                            }
                            a { href: "https://web.archive.org/web/20190723164913/http://www.graciehumaitacolumbia.com/", target: "_blank", rel: "noopener noreferrer", class: "btn btn-secondary",
                                "Before →"
                            }
                        }
                    }

                    // American Top Team Indianapolis
                    div { class: "portfolio-card glass-card",
                        div { class: "portfolio-header",
                            h3 { class: "portfolio-title", "American Top Team Indianapolis" }
                            span { class: "portfolio-type", "Website Replacement" }
                        }
                        p { class: "portfolio-description",
                            "Replaced a broken, outdated website for a martial arts training facility. Built a clean, professional site with focus on easy navigation for prospective students."
                        }
                        div { class: "portfolio-tech",
                            span { class: "tech-tag", "Responsive" }
                            span { class: "tech-tag", "SEO" }
                        }
                        ul { class: "portfolio-scope",
                            li { "Replaced old broken website" }
                            li { "Clean, professional design" }
                        }
                        div { class: "portfolio-actions",
                            a { href: "https://attindianapolis.com", target: "_blank", rel: "noopener noreferrer", class: "btn btn-primary",
                                "Visit Site →"
                            }
                            a { href: "https://web.archive.org/web/20200530220933/http://www.attindianapolis.com/", target: "_blank", rel: "noopener noreferrer", class: "btn btn-secondary",
                                "Before →"
                            }
                        }
                    }

                    // APEX Earthworks
                    a { href: "https://apexearthwork.com", target: "_blank", rel: "noopener noreferrer", class: "portfolio-card-link",
                        div { class: "portfolio-card glass-card",
                            div { class: "portfolio-header",
                                h3 { class: "portfolio-title", "APEX Earthworks" }
                                span { class: "portfolio-type", "Website + Lead Generation" }
                            }
                            p { class: "portfolio-description",
                                "Business website for an earthwork and excavation company. Professional presentation with automated customer lead generation to capture and follow up with potential clients."
                            }
                            div { class: "portfolio-tech",
                                span { class: "tech-tag", "Lead Gen Automation" }
                                span { class: "tech-tag", "Mobile" }
                            }
                            ul { class: "portfolio-scope",
                                li { "Automated lead capture and follow-up" }
                                li { "Professional company showcase" }
                            }
                            span { class: "portfolio-link btn btn-primary",
                                "Visit Site →"
                            }
                        }
                    }

                    // Missouri Jiu Jitsu
                    a { href: "https://missourijiujitsu.com", target: "_blank", rel: "noopener noreferrer", class: "portfolio-card-link",
                        div { class: "portfolio-card glass-card",
                            div { class: "portfolio-header",
                                h3 { class: "portfolio-title", "Missouri Jiu Jitsu" }
                                span { class: "portfolio-type", "Website Development" }
                            }
                            p { class: "portfolio-description",
                                "Demo website with a mock jiu-jitsu academy featuring class information, instructor bios, and signup flow automation."
                            }
                            div { class: "portfolio-tech",
                                span { class: "tech-tag", "Responsive" }
                                span { class: "tech-tag", "Forms" }
                            }
                            ul { class: "portfolio-scope",
                                li { "Class schedules and instructor bios" }
                                li { "Membership inquiry forms" }
                            }
                            span { class: "portfolio-link btn btn-primary",
                                "Visit Site →"
                            }
                        }
                    }

                    // Delaware Krav Maga
                    a { href: "https://delawarekravmaga.com", target: "_blank", rel: "noopener noreferrer", class: "portfolio-card-link",
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
                            ul { class: "portfolio-scope",
                                li { "Program showcase with clear CTAs" }
                                li { "New student sign-up flow" }
                            }
                            span { class: "portfolio-link btn btn-primary",
                                "Visit Site →"
                            }
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
