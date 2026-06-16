use dioxus::prelude::*;

/// Pounds Consulting wordmark lockup: a radiating triangle mark next to
/// "Pounds Consulting". The triangle is drawn in `currentColor`; the wordmark
/// styling is controlled by CSS per variant:
/// - "header": white triangle + shiny gold (gradient) wordmark
/// - "footer": all white
#[component]
pub fn BrandLogo(variant: String) -> Element {
    rsx! {
        span {
            class: "brand-logo brand-logo--{variant}",
            role: "img",
            "aria-label": "Pounds Consulting",
            svg {
                class: "brand-logo__mark",
                xmlns: "http://www.w3.org/2000/svg",
                view_box: "0 0 64 64",
                "aria-hidden": "true",
                style: "stroke: currentColor; fill: none; stroke-width: 1.4px; stroke-linecap: square;",
                line { x1: "8", y1: "54", x2: "54", y2: "10" }
                line { x1: "8", y1: "54", x2: "54", y2: "21" }
                line { x1: "8", y1: "54", x2: "54", y2: "32" }
                line { x1: "8", y1: "54", x2: "54", y2: "43" }
                line { x1: "8", y1: "54", x2: "54", y2: "54" }
                line { x1: "54", y1: "10", x2: "54", y2: "54" }
            }
            span { class: "brand-logo__word", "Pounds Consulting" }
        }
    }
}
