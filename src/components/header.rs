use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Header() -> Element {
    let mut mobile_menu_open = use_signal(|| false);
    let current_route = use_route::<Route>();

    // Helper to determine if a route is active
    let is_active = |route: &Route| -> bool {
        std::mem::discriminant(&current_route) == std::mem::discriminant(route)
    };

    let home_class = if is_active(&Route::Home {}) {
        "nav-link active"
    } else {
        "nav-link"
    };
    let about_class = if is_active(&Route::About {}) {
        "nav-link active"
    } else {
        "nav-link"
    };
    let services_class = if is_active(&Route::Services {}) {
        "nav-link active"
    } else {
        "nav-link"
    };
    let portfolio_class = if is_active(&Route::Portfolio {}) {
        "nav-link active"
    } else {
        "nav-link"
    };
    let articles_class = if is_active(&Route::Articles {}) {
        "nav-link active"
    } else {
        "nav-link"
    };
    let contact_class = if is_active(&Route::Contact {}) {
        "nav-link nav-cta active"
    } else {
        "nav-link nav-cta"
    };

    rsx! {
        header { class: "header",
            div { class: "header-container",
                Link { to: Route::Home {}, class: "footer-logo",
                    svg {
                        class: "logo-mark",
                        xmlns: "http://www.w3.org/2000/svg",
                        view_box: "0 0 64 64",
                        width: "26",
                        height: "26",
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
                    span { class: "logo-accent", "/ CONSULTING" }
                }

                nav { class: "nav-desktop",
                    Link { to: Route::Home {}, class: "{home_class}", "Home" }
                    Link { to: Route::About {}, class: "{about_class}", "About" }
                    Link { to: Route::Services {}, class: "{services_class}", "Services" }
                    Link { to: Route::Portfolio {}, class: "{portfolio_class}", "Portfolio" }
                    Link { to: Route::Articles {}, class: "{articles_class}", "Articles" }
                    Link { to: Route::Contact {}, class: "{contact_class}", "Contact" }
                }

                button {
                    class: "mobile-menu-toggle",
                    "aria-label": "Toggle navigation menu",
                    "aria-expanded": "{mobile_menu_open()}",
                    onclick: move |_| mobile_menu_open.set(!mobile_menu_open()),
                    span { class: "hamburger-line" }
                    span { class: "hamburger-line" }
                    span { class: "hamburger-line" }
                }
            }

            if mobile_menu_open() {
                nav { class: "nav-mobile",
                    Link {
                        to: Route::Home {},
                        class: "{home_class}",
                        onclick: move |_| mobile_menu_open.set(false),
                        "Home"
                    }
                    Link {
                        to: Route::About {},
                        class: "{about_class}",
                        onclick: move |_| mobile_menu_open.set(false),
                        "About"
                    }
                    Link {
                        to: Route::Services {},
                        class: "{services_class}",
                        onclick: move |_| mobile_menu_open.set(false),
                        "Services"
                    }
                    Link {
                        to: Route::Portfolio {},
                        class: "{portfolio_class}",
                        onclick: move |_| mobile_menu_open.set(false),
                        "Portfolio"
                    }
                    Link {
                        to: Route::Articles {},
                        class: "{articles_class}",
                        onclick: move |_| mobile_menu_open.set(false),
                        "Articles"
                    }
                    Link {
                        to: Route::Contact {},
                        class: "{contact_class}",
                        onclick: move |_| mobile_menu_open.set(false),
                        "Contact"
                    }
                }
            }
        }
    }
}
