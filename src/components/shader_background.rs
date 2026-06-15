use dioxus::prelude::*;

/// Full-bleed background `<canvas>` that the inline shader driver in `index.html`
/// mounts a gold-tinted, slowed WebGL shader onto. Sits behind content, decorative
/// only (aria-hidden), and pauses to a static frame under prefers-reduced-motion.
///
/// `variant` selects the effect: "smoke" (fbm haze) or "lines" (radiating lines).
#[component]
pub fn ShaderBackground(variant: String) -> Element {
    rsx! {
        canvas {
            class: "shader-bg",
            "data-shader": "{variant}",
            "aria-hidden": "true",
        }
    }
}
