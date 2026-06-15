use dioxus::prelude::*;

#[component]
pub fn Contact() -> Element {
    rsx! {
        // Hero Section
        section { class: "hero hero-short",
            div { class: "hero-content",
                h1 { class: "hero-title", "Let's Talk" }
                p { class: "hero-subtitle",
                    "Have a project in mind? Questions? Just want to network? Book some time."
                }
            }
        }

        // Contact Section
        section { class: "section contact-section",
            div { class: "container",
                div { class: "contact-centered",
                    // Schedule CTA
                    div { class: "schedule-card glass-card",
                        h2 { class: "schedule-title", "Schedule a Call" }
                        p { class: "schedule-description",
                            "Book a free 30-minute discovery call. We'll discuss your project, answer questions, and figure out if we're a good fit."
                        }
                        a {
                            href: "https://calendar.app.google/LNasBbmDr8LXNEuu5",
                            target: "_blank",
                            rel: "noopener noreferrer",
                            class: "btn btn-primary btn-large",
                            "Book a Time"
                        }
                    }

                    // Contact Info
                    div { class: "contact-details glass-card",
                        h3 { class: "contact-heading", "Other Ways to Reach Us" }
                        div { class: "contact-item",
                            span { class: "contact-label", "Email" }
                            a {
                                href: "mailto:collin@poundsconsulting.net",
                                class: "contact-value contact-link",
                                "collin@poundsconsulting.net"
                            }
                        }
                        div { class: "contact-item",
                            span { class: "contact-label", "Location" }
                            span { class: "contact-value", "San Francisco Bay Area" }
                            span { class: "contact-note", }
                        }
                    }
                }

                // FAQ Section
                div { class: "faq-section",
                    h2 { class: "section-title", "Common Questions" }
                    div { class: "faq-grid",
                        div { class: "faq-item glass-card",
                            h3 { class: "faq-question", "What's a typical project timeline?" }
                            p { class: "faq-answer",
                                "Simple websites can launch in 2-4 weeks. More complex projects vary based on scope, but we'll provide a realistic timeline during our initial conversation."
                            }
                        }
                        div { class: "faq-item glass-card",
                            h3 { class: "faq-question", "Do you work with clients outside the Bay Area?" }
                            p { class: "faq-answer",
                                "Absolutely. We're based in the San Francisco Bay Area, but most of our work is remote. We work with clients across the country."
                            }
                        }
                        div { class: "faq-item glass-card",
                            h3 { class: "faq-question", "What if I'm not sure what I need?" }
                            p { class: "faq-answer",
                                "That's completely fine. Most conversations start with a problem, not a solution. Book a call and we'll help you figure out the right approach."
                            }
                        }
                        div { class: "faq-item glass-card",
                            h3 { class: "faq-question", "How does billing work?" }
                            p { class: "faq-answer",
                                "Most work is quoted by project, so you know the full cost before we start. For smaller or open-ended work billed hourly, the rate runs $71 to $100 per hour depending on scope. We offer a 50% discount for military, veterans, law enforcement, first responders, and healthcare workers."
                            }
                        }
                    }
                }
            }
        }
    }
}
