use crate::content::{
    is_authenticated, load_articles, save_articles, set_authenticated, ArticleStatus,
};
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn AdminArticles() -> Element {
    let navigator = use_navigator();

    // Redirect if not authenticated
    use_effect(move || {
        if !is_authenticated() {
            navigator.push(Route::AdminLogin {});
        }
    });

    let mut articles_data = use_signal(load_articles);
    let mut delete_confirm = use_signal(|| Option::<String>::None);

    let mut handle_delete = move |id: String| {
        articles_data.with_mut(|data| {
            data.articles.retain(|a| a.id != id);
        });
        save_articles(&articles_data());
        delete_confirm.set(None);
    };

    let handle_logout = move |_| {
        set_authenticated(false);
        navigator.push(Route::AdminLogin {});
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
                    li { class: "admin-nav-item",
                        Link { to: Route::AdminSettings {}, class: "admin-nav-link", "⚙️ Settings" }
                    }
                    li { class: "admin-nav-item active",
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
                    h1 { "Articles" }
                    div { class: "admin-header-actions",
                        Link {
                            to: Route::AdminArticleNew {},
                            class: "btn btn-primary",
                            "+ New Article"
                        }
                    }
                }

                // Delete Confirmation Modal
                if let Some(id) = delete_confirm() {
                    div { class: "admin-modal-overlay",
                        div { class: "admin-modal glass-card",
                            h3 { "Delete Article?" }
                            p { "Are you sure you want to delete this article? This action cannot be undone." }
                            div { class: "admin-modal-actions",
                                button {
                                    class: "btn btn-secondary",
                                    onclick: move |_| delete_confirm.set(None),
                                    "Cancel"
                                }
                                button {
                                    class: "btn btn-danger",
                                    onclick: move |_| handle_delete(id.clone()),
                                    "Delete"
                                }
                            }
                        }
                    }
                }

                // Articles Table
                div { class: "admin-section",
                    div { class: "admin-table-container glass-card",
                        if articles_data().articles.is_empty() {
                            div { class: "admin-empty-state",
                                div { class: "admin-empty-icon", "📝" }
                                h3 { "No articles yet" }
                                p { "Create your first article to get started." }
                                Link {
                                    to: Route::AdminArticleNew {},
                                    class: "btn btn-primary",
                                    "Create Article"
                                }
                            }
                        } else {
                            table { class: "admin-table",
                                thead {
                                    tr {
                                        th { "Title" }
                                        th { "Category" }
                                        th { "Date" }
                                        th { "Status" }
                                        th { "Actions" }
                                    }
                                }
                                tbody {
                                    for article in articles_data().articles.iter() {
                                        tr { key: "{article.id}",
                                            td {
                                                div { class: "article-title-cell",
                                                    strong { "{article.title}" }
                                                    span { class: "article-slug", "/{article.slug}" }
                                                }
                                            }
                                            td { "{article.category}" }
                                            td { "{article.date}" }
                                            td {
                                                span {
                                                    class: if matches!(article.status, ArticleStatus::Published) {
                                                        "status-badge status-published"
                                                    } else {
                                                        "status-badge status-draft"
                                                    },
                                                    {if matches!(article.status, ArticleStatus::Published) {
                                                        "Published"
                                                    } else {
                                                        "Draft"
                                                    }}
                                                }
                                            }
                                            td { class: "admin-table-actions",
                                                Link {
                                                    to: Route::AdminArticleEdit { id: article.id.clone() },
                                                    class: "admin-action-btn",
                                                    "Edit"
                                                }
                                                button {
                                                    class: "admin-action-btn admin-action-delete",
                                                    onclick: {
                                                        let id = article.id.clone();
                                                        move |_| delete_confirm.set(Some(id.clone()))
                                                    },
                                                    "Delete"
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
