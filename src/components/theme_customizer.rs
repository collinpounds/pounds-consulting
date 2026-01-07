use crate::content::{apply_theme_to_dom, load_theme, save_theme, ThemeConfig};
use dioxus::prelude::*;

#[component]
pub fn ThemeCustomizer(is_open: Signal<bool>) -> Element {
    let mut current_theme = use_signal(load_theme);

    // Apply theme to DOM whenever it changes
    use_effect(move || {
        apply_theme_to_dom(&current_theme());
    });

    let mut apply_preset = move |preset: ThemeConfig| {
        current_theme.set(preset.clone());
        save_theme(&preset);
        apply_theme_to_dom(&preset);
    };

    let mut update_color = move |field: &'static str, value: String| {
        current_theme.with_mut(|theme| {
            match field {
                "primary" => theme.primary = value,
                "secondary" => theme.secondary = value,
                "accent" => theme.accent = value,
                "background" => theme.background = value,
                "surface" => theme.surface = value,
                "text_primary" => theme.text_primary = value,
                "text_secondary" => theme.text_secondary = value,
                "border" => theme.border = value,
                _ => {}
            }
            theme.name = "Custom".to_string();
        });
        save_theme(&current_theme());
        apply_theme_to_dom(&current_theme());
    };

    let reset_to_default = move |_| {
        let default = ThemeConfig::default_gold();
        current_theme.set(default.clone());
        save_theme(&default);
        apply_theme_to_dom(&default);
    };

    let close_panel = move |_| {
        is_open.set(false);
    };

    if !is_open() {
        return rsx! {};
    }

    rsx! {
        // Backdrop
        div {
            class: "theme-customizer-backdrop",
            onclick: close_panel
        }

        // Sidebar panel
        div { class: "theme-customizer-panel",
            div { class: "theme-customizer-header",
                h3 { "Customize Theme" }
                button {
                    class: "theme-customizer-close",
                    onclick: close_panel,
                    "×"
                }
            }

            div { class: "theme-customizer-content",
                // Preset themes section
                div { class: "theme-customizer-section",
                    h4 { "Presets" }
                    div { class: "theme-presets-grid",
                        for preset in ThemeConfig::all_presets() {
                            button {
                                key: "{preset.name}",
                                class: if current_theme().name == preset.name { "theme-preset-btn active" } else { "theme-preset-btn" },
                                onclick: {
                                    let preset = preset.clone();
                                    move |_| apply_preset(preset.clone())
                                },
                                div {
                                    class: "theme-preset-swatch",
                                    style: "background: linear-gradient(135deg, {preset.background} 50%, {preset.secondary} 50%);"
                                }
                                span { "{preset.name}" }
                            }
                        }
                    }
                }

                // Custom colors section
                div { class: "theme-customizer-section",
                    h4 { "Custom Colors" }
                    div { class: "theme-color-inputs",
                        ColorInput {
                            label: "Primary",
                            value: current_theme().primary.clone(),
                            on_change: move |v| update_color("primary", v)
                        }
                        ColorInput {
                            label: "Accent (Gold)",
                            value: current_theme().secondary.clone(),
                            on_change: move |v| update_color("secondary", v)
                        }
                        ColorInput {
                            label: "Accent Hover",
                            value: current_theme().accent.clone(),
                            on_change: move |v| update_color("accent", v)
                        }
                        ColorInput {
                            label: "Background",
                            value: current_theme().background.clone(),
                            on_change: move |v| update_color("background", v)
                        }
                        ColorInput {
                            label: "Surface",
                            value: current_theme().surface.clone(),
                            on_change: move |v| update_color("surface", v)
                        }
                        ColorInput {
                            label: "Text Primary",
                            value: current_theme().text_primary.clone(),
                            on_change: move |v| update_color("text_primary", v)
                        }
                        ColorInput {
                            label: "Text Secondary",
                            value: current_theme().text_secondary.clone(),
                            on_change: move |v| update_color("text_secondary", v)
                        }
                        ColorInput {
                            label: "Border",
                            value: current_theme().border.clone(),
                            on_change: move |v| update_color("border", v)
                        }
                    }
                }

                // Actions
                div { class: "theme-customizer-actions",
                    button {
                        class: "btn btn-secondary",
                        onclick: reset_to_default,
                        "Reset to Default"
                    }
                }

                // Current theme indicator
                div { class: "theme-customizer-current",
                    "Current: {current_theme().name}"
                }
            }
        }
    }
}

#[component]
fn ColorInput(label: String, value: String, on_change: EventHandler<String>) -> Element {
    rsx! {
        div { class: "theme-color-input",
            label { "{label}" }
            div { class: "theme-color-input-row",
                input {
                    r#type: "color",
                    value: "{value}",
                    oninput: move |evt| on_change.call(evt.value())
                }
                input {
                    r#type: "text",
                    value: "{value}",
                    oninput: move |evt| on_change.call(evt.value()),
                    maxlength: "7",
                    placeholder: "#000000"
                }
            }
        }
    }
}

/// Toggle button component for the footer
#[component]
pub fn ThemeToggleButton(is_open: Signal<bool>) -> Element {
    rsx! {
        button {
            class: "theme-toggle-btn",
            title: "Customize Theme",
            onclick: move |_| is_open.set(!is_open()),
            svg {
                xmlns: "http://www.w3.org/2000/svg",
                width: "20",
                height: "20",
                view_box: "0 0 24 24",
                fill: "currentColor",
                path {
                    d: "M12 22C6.49 22 2 17.51 2 12S6.49 2 12 2s10 4.04 10 9c0 3.31-2.69 6-6 6h-1.77c-.28 0-.5.22-.5.5 0 .12.05.23.13.33.41.47.64 1.06.64 1.67A2.5 2.5 0 0 1 12 22zm0-18c-4.41 0-8 3.59-8 8s3.59 8 8 8c.28 0 .5-.22.5-.5a.54.54 0 0 0-.14-.35c-.41-.46-.63-1.05-.63-1.65a2.5 2.5 0 0 1 2.5-2.5H16c2.21 0 4-1.79 4-4 0-3.86-3.59-7-8-7z"
                }
                circle { cx: "6.5", cy: "11.5", r: "1.5" }
                circle { cx: "9.5", cy: "7.5", r: "1.5" }
                circle { cx: "14.5", cy: "7.5", r: "1.5" }
                circle { cx: "17.5", cy: "11.5", r: "1.5" }
            }
        }
    }
}
