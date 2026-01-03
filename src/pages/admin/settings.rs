use dioxus::prelude::*;
use crate::content::{load_settings, save_settings, is_authenticated, set_authenticated, SiteSettings, PageConfig};
use crate::Route;

#[component]
pub fn AdminSettings() -> Element {
    let navigator = use_navigator();

    // Redirect if not authenticated
    use_effect(move || {
        if !is_authenticated() {
            navigator.push(Route::AdminLogin {});
        }
    });

    let mut settings = use_signal(load_settings);
    let mut saved_message = use_signal(|| Option::<String>::None);

    let handle_save = move |_| {
        save_settings(&settings());
        saved_message.set(Some("Settings saved successfully!".to_string()));

        // Clear message after 3 seconds
        spawn(async move {
            gloo_timers::future::TimeoutFuture::new(3000).await;
            saved_message.set(None);
        });
    };

    let handle_logout = move |_| {
        set_authenticated(false);
        navigator.push(Route::AdminLogin {});
    };

    let update_brand_name = move |evt: FormEvent| {
        settings.with_mut(|s| s.brand.name = evt.value());
    };

    let update_tagline = move |evt: FormEvent| {
        settings.with_mut(|s| s.brand.tagline = evt.value());
    };

    let update_primary_color = move |evt: FormEvent| {
        settings.with_mut(|s| s.brand.primary_color = evt.value());
    };

    let update_accent_color = move |evt: FormEvent| {
        settings.with_mut(|s| s.brand.accent_color = evt.value());
    };

    let update_password = move |evt: FormEvent| {
        settings.with_mut(|s| s.admin_password_hash = evt.value());
    };

    rsx! {
        div { class: "admin-layout",
            // Sidebar
            nav { class: "admin-sidebar",
                div { class: "admin-sidebar-header",
                    h2 { class: "admin-logo", "Admin" }
                }

                ul { class: "admin-nav",
                    li { class: "admin-nav-item",
                        Link { to: Route::AdminDashboard {}, class: "admin-nav-link", "📊 Dashboard" }
                    }
                    li { class: "admin-nav-item active",
                        Link { to: Route::AdminSettings {}, class: "admin-nav-link", "⚙️ Settings" }
                    }
                    li { class: "admin-nav-item",
                        Link { to: Route::AdminArticles {}, class: "admin-nav-link", "📝 Articles" }
                    }
                }

                div { class: "admin-sidebar-footer",
                    a { href: "/", class: "admin-nav-link", "🏠 View Site" }
                    button {
                        class: "btn btn-secondary btn-full",
                        onclick: handle_logout,
                        "Logout"
                    }
                }
            }

            // Main Content
            main { class: "admin-main",
                div { class: "admin-header",
                    h1 { "Settings" }
                    div { class: "admin-header-actions",
                        if let Some(msg) = saved_message() {
                            span { class: "admin-success-message", "{msg}" }
                        }
                        button {
                            class: "btn btn-primary",
                            onclick: handle_save,
                            "Save Changes"
                        }
                    }
                }

                // Brand Settings
                div { class: "admin-section",
                    h2 { "Brand Settings" }
                    div { class: "admin-form-card glass-card",
                        div { class: "form-group",
                            label { class: "form-label", "Site Name" }
                            input {
                                class: "form-input",
                                r#type: "text",
                                value: "{settings().brand.name}",
                                oninput: update_brand_name
                            }
                        }

                        div { class: "form-group",
                            label { class: "form-label", "Tagline" }
                            input {
                                class: "form-input",
                                r#type: "text",
                                value: "{settings().brand.tagline}",
                                oninput: update_tagline
                            }
                        }

                        div { class: "form-row",
                            div { class: "form-group",
                                label { class: "form-label", "Primary Color" }
                                div { class: "color-input-group",
                                    input {
                                        class: "form-input form-input-color",
                                        r#type: "color",
                                        value: "{settings().brand.primary_color}",
                                        oninput: update_primary_color
                                    }
                                    input {
                                        class: "form-input",
                                        r#type: "text",
                                        value: "{settings().brand.primary_color}",
                                        oninput: update_primary_color
                                    }
                                }
                            }

                            div { class: "form-group",
                                label { class: "form-label", "Accent Color" }
                                div { class: "color-input-group",
                                    input {
                                        class: "form-input form-input-color",
                                        r#type: "color",
                                        value: "{settings().brand.accent_color}",
                                        oninput: update_accent_color
                                    }
                                    input {
                                        class: "form-input",
                                        r#type: "text",
                                        value: "{settings().brand.accent_color}",
                                        oninput: update_accent_color
                                    }
                                }
                            }
                        }
                    }
                }

                // Feature Toggles
                div { class: "admin-section",
                    h2 { "Features" }
                    div { class: "admin-form-card glass-card",
                        div { class: "toggle-group",
                            label { class: "toggle-label",
                                input {
                                    r#type: "checkbox",
                                    class: "toggle-input",
                                    checked: settings().features.services,
                                    onchange: move |evt: FormEvent| {
                                        settings.with_mut(|s| s.features.services = evt.checked());
                                    }
                                }
                                span { class: "toggle-switch" }
                                span { class: "toggle-text", "Services Page" }
                            }
                        }

                        div { class: "toggle-group",
                            label { class: "toggle-label",
                                input {
                                    r#type: "checkbox",
                                    class: "toggle-input",
                                    checked: settings().features.portfolio,
                                    onchange: move |evt: FormEvent| {
                                        settings.with_mut(|s| s.features.portfolio = evt.checked());
                                    }
                                }
                                span { class: "toggle-switch" }
                                span { class: "toggle-text", "Portfolio Page" }
                            }
                        }

                        div { class: "toggle-group",
                            label { class: "toggle-label",
                                input {
                                    r#type: "checkbox",
                                    class: "toggle-input",
                                    checked: settings().features.articles,
                                    onchange: move |evt: FormEvent| {
                                        settings.with_mut(|s| s.features.articles = evt.checked());
                                    }
                                }
                                span { class: "toggle-switch" }
                                span { class: "toggle-text", "Articles Section" }
                            }
                        }

                        div { class: "toggle-group",
                            label { class: "toggle-label",
                                input {
                                    r#type: "checkbox",
                                    class: "toggle-input",
                                    checked: settings().features.contact,
                                    onchange: move |evt: FormEvent| {
                                        settings.with_mut(|s| s.features.contact = evt.checked());
                                    }
                                }
                                span { class: "toggle-switch" }
                                span { class: "toggle-text", "Contact Page" }
                            }
                        }

                        div { class: "toggle-group",
                            label { class: "toggle-label",
                                input {
                                    r#type: "checkbox",
                                    class: "toggle-input",
                                    checked: settings().features.testimonials,
                                    onchange: move |evt: FormEvent| {
                                        settings.with_mut(|s| s.features.testimonials = evt.checked());
                                    }
                                }
                                span { class: "toggle-switch" }
                                span { class: "toggle-text", "Testimonials" }
                            }
                        }
                    }
                }

                // Page Management
                div { class: "admin-section",
                    h2 { "Page Visibility" }
                    div { class: "admin-form-card glass-card",
                        for (idx, page) in settings().pages.iter().enumerate() {
                            div { class: "toggle-group", key: "{page.id}",
                                label { class: "toggle-label",
                                    input {
                                        r#type: "checkbox",
                                        class: "toggle-input",
                                        checked: page.enabled,
                                        onchange: move |evt: FormEvent| {
                                            settings.with_mut(|s| {
                                                if let Some(p) = s.pages.get_mut(idx) {
                                                    p.enabled = evt.checked();
                                                }
                                            });
                                        }
                                    }
                                    span { class: "toggle-switch" }
                                    span { class: "toggle-text", "{page.label}" }
                                    span { class: "toggle-path", "{page.path}" }
                                }
                            }
                        }
                    }
                }

                // Security
                div { class: "admin-section",
                    h2 { "Security" }
                    div { class: "admin-form-card glass-card",
                        div { class: "form-group",
                            label { class: "form-label", "Admin Password" }
                            input {
                                class: "form-input",
                                r#type: "password",
                                placeholder: "Enter new password",
                                value: "{settings().admin_password_hash}",
                                oninput: update_password
                            }
                            p { class: "form-hint", "Change the admin password here" }
                        }
                    }
                }
            }
        }
    }
}
