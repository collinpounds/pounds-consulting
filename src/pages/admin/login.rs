use crate::content::{is_authenticated, load_settings, set_authenticated, verify_password};
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn AdminLogin() -> Element {
    let mut password = use_signal(String::new);
    let mut error = use_signal(|| Option::<String>::None);
    let navigator = use_navigator();

    // Redirect if already authenticated
    use_effect(move || {
        if is_authenticated() {
            navigator.push(Route::AdminDashboard {});
        }
    });

    let handle_login = move |evt: FormEvent| {
        evt.prevent_default();
        let settings = load_settings();

        if verify_password(&password(), &settings) {
            set_authenticated(true);
            navigator.push(Route::AdminDashboard {});
        } else {
            error.set(Some("Invalid password".to_string()));
        }
    };

    rsx! {
        div { class: "admin-login-page",
            div { class: "admin-login-card glass-card",
                h1 { class: "admin-login-title", "Admin Login" }
                p { class: "admin-login-subtitle", "Enter your password to access the dashboard" }

                form {
                    onsubmit: handle_login,
                    class: "admin-login-form",

                    if let Some(err) = error() {
                        div { class: "admin-error", "{err}" }
                    }

                    div { class: "form-group",
                        label { class: "form-label", r#for: "password", "Password" }
                        input {
                            class: "form-input",
                            r#type: "password",
                            id: "password",
                            placeholder: "Enter admin password",
                            value: "{password}",
                            oninput: move |evt| password.set(evt.value())
                        }
                    }

                    button {
                        r#type: "submit",
                        class: "btn btn-primary btn-full",
                        "Login"
                    }
                }

                div { class: "admin-login-footer",
                    a { href: "/", class: "admin-back-link", "← Back to site" }
                }
            }
        }
    }
}
