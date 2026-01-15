use super::icon::{parse_icon_name, Icon};
use dioxus::prelude::*;

#[component]
pub fn ServiceCard(title: String, description: String, icon: String) -> Element {
    rsx! {
        div { class: "service-card",
            div { class: "service-icon",
                if let Some(icon_name) = parse_icon_name(&icon) {
                    Icon { name: icon_name, size: 40, color: "var(--color-secondary)".to_string() }
                }
            }
            h3 { class: "service-title", "{title}" }
            p { class: "service-description", "{description}" }
        }
    }
}
