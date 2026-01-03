use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Header() -> Element {
    let mut mobile_menu_open = use_signal(|| false);

    rsx! {
        header { class: "header",
            div { class: "header-container",
                Link { to: Route::Home {}, class: "logo",
                    span { class: "logo-text", "POUNDS" }
                    span { class: "logo-accent", "CONSULTING" }
                }

                nav { class: "nav-desktop",
                    Link { to: Route::Home {}, class: "nav-link", "Home" }
                    Link { to: Route::About {}, class: "nav-link", "About" }
                    Link { to: Route::Services {}, class: "nav-link", "Services" }
                    Link { to: Route::Portfolio {}, class: "nav-link", "Portfolio" }
                    Link { to: Route::Articles {}, class: "nav-link", "Articles" }
                    Link { to: Route::Contact {}, class: "nav-link nav-cta", "Contact" }
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
                        class: "nav-link",
                        onclick: move |_| mobile_menu_open.set(false),
                        "Home"
                    }
                    Link {
                        to: Route::About {},
                        class: "nav-link",
                        onclick: move |_| mobile_menu_open.set(false),
                        "About"
                    }
                    Link {
                        to: Route::Services {},
                        class: "nav-link",
                        onclick: move |_| mobile_menu_open.set(false),
                        "Services"
                    }
                    Link {
                        to: Route::Portfolio {},
                        class: "nav-link",
                        onclick: move |_| mobile_menu_open.set(false),
                        "Portfolio"
                    }
                    Link {
                        to: Route::Articles {},
                        class: "nav-link",
                        onclick: move |_| mobile_menu_open.set(false),
                        "Articles"
                    }
                    Link {
                        to: Route::Contact {},
                        class: "nav-link nav-cta",
                        onclick: move |_| mobile_menu_open.set(false),
                        "Contact"
                    }
                }
            }
        }
    }
}
