use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn CtaSection(
    title: String,
    description: String,
    button_text: String,
    #[props(default = false)] use_calendar_link: bool,
) -> Element {
    rsx! {
        section { class: "cta-section",
            div { class: "cta-container",
                h2 { class: "cta-title", "{title}" }
                p { class: "cta-description", "{description}" }
                if use_calendar_link {
                    a {
                        href: "https://calendar.app.google/LNasBbmDr8LXNEuu5",
                        target: "_blank",
                        rel: "noopener noreferrer",
                        class: "btn btn-primary",
                        "{button_text}"
                    }
                } else {
                    Link { to: Route::Contact {}, class: "btn btn-primary",
                        "{button_text}"
                    }
                }
            }
        }
    }
}
