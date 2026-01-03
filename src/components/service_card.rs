use dioxus::prelude::*;

#[component]
pub fn ServiceCard(title: String, description: String, icon: String) -> Element {
    rsx! {
        div { class: "service-card",
            div { class: "service-icon", "{icon}" }
            h3 { class: "service-title", "{title}" }
            p { class: "service-description", "{description}" }
        }
    }
}
