use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Footer() -> Element {
    rsx! {
        footer { class: "footer",
            div { class: "footer-container",
                div { class: "footer-brand",
                    Link { to: Route::Home {}, class: "footer-logo",
                        span { class: "logo-text", "POUNDS" }
                        span { class: "logo-accent", "CONSULTING" }
                    }
                    p { class: "footer-tagline",
                        "Technical solutions for growing businesses."
                    }
                }

                div { class: "footer-nav",
                    h4 { class: "footer-heading", "Navigation" }
                    Link { to: Route::Home {}, class: "footer-link", "Home" }
                    Link { to: Route::About {}, class: "footer-link", "About" }
                    Link { to: Route::Services {}, class: "footer-link", "Services" }
                    Link { to: Route::Contact {}, class: "footer-link", "Contact" }
                }

                div { class: "footer-contact",
                    h4 { class: "footer-heading", "Contact" }
                    a {
                        href: "mailto:collin@poundsconsulting.net",
                        class: "footer-link",
                        "collin@poundsconsulting.net"
                    }
                    p { class: "footer-location", "Columbia, Missouri" }
                }
            }

            div { class: "footer-bottom",
                p { "© 2025 Pounds Consulting LLC. All rights reserved." }
            }
        }
    }
}
