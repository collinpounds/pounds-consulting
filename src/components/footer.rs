use crate::components::{ThemeCustomizer, ThemeToggleButton};
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Footer() -> Element {
    let theme_panel_open = use_signal(|| false);

    rsx! {
        // Theme customizer panel (renders when open)
        ThemeCustomizer { is_open: theme_panel_open }

        footer { class: "footer",
            div { class: "footer-container",
                div { class: "footer-brand",
                    Link { to: Route::Home {}, class: "footer-logo",
                        svg {
                            class: "logo-mark",
                            xmlns: "http://www.w3.org/2000/svg",
                            view_box: "0 0 64 64",
                            width: "24",
                            height: "24",
                            fill: "none",
                            line { x1: "8", y1: "56", x2: "56", y2: "8", stroke: "#D4A017", stroke_width: "2.5" }
                            line { x1: "8", y1: "56", x2: "56", y2: "16", stroke: "#D4A017", stroke_width: "2.5" }
                            line { x1: "8", y1: "56", x2: "56", y2: "24", stroke: "#D4A017", stroke_width: "2.5" }
                            line { x1: "8", y1: "56", x2: "56", y2: "32", stroke: "#D4A017", stroke_width: "2.5" }
                            line { x1: "8", y1: "56", x2: "56", y2: "40", stroke: "#D4A017", stroke_width: "2.5" }
                            line { x1: "8", y1: "56", x2: "56", y2: "48", stroke: "#D4A017", stroke_width: "2.5" }
                            line { x1: "8", y1: "56", x2: "56", y2: "56", stroke: "#D4A017", stroke_width: "2.5" }
                            line { x1: "56", y1: "8", x2: "56", y2: "56", stroke: "#D4A017", stroke_width: "2.5" }
                        }
                        span { class: "logo-text", "POUNDS" }
                        span { class: "logo-accent", "CONSULTING" }
                    }
                    p { class: "footer-tagline",
                        "Technical solutions for growing businesses."
                    }
                }

                div { class: "footer-nav",
                    h2 { class: "footer-heading", "Navigation" }
                    Link { to: Route::Home {}, class: "footer-link", "Home" }
                    Link { to: Route::About {}, class: "footer-link", "About" }
                    Link { to: Route::Services {}, class: "footer-link", "Services" }
                    Link { to: Route::Portfolio {}, class: "footer-link", "Portfolio" }
                    Link { to: Route::Articles {}, class: "footer-link", "Articles" }
                    Link { to: Route::Contact {}, class: "footer-link", "Contact" }
                }

                div { class: "footer-contact",
                    h2 { class: "footer-heading", "Contact" }
                    a {
                        href: "mailto:collin@poundsconsulting.net",
                        class: "footer-link",
                        "collin@poundsconsulting.net"
                    }
                    p { class: "footer-location", "San Francisco Bay Area" }
                }
            }

            div { class: "footer-bottom",
                div { class: "footer-bottom-links",
                    a {
                        href: "https://github.com/collinpounds",
                        class: "footer-github-link",
                        target: "_blank",
                        rel: "noopener noreferrer",
                        svg {
                            class: "github-icon",
                            xmlns: "http://www.w3.org/2000/svg",
                            width: "20",
                            height: "20",
                            view_box: "0 0 24 24",
                            fill: "currentColor",
                            path {
                                d: "M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"
                            }
                        }
                        "github.com/collinpounds"
                    }
                    ThemeToggleButton { is_open: theme_panel_open }
                }
                p { "© 2026 Pounds Consulting LLC. All rights reserved." }
            }
        }
    }
}
