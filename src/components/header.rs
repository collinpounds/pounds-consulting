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
