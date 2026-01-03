use crate::content::{
    is_authenticated, load_articles, load_settings, set_authenticated, ArticleStatus,
};
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn AdminDashboard() -> Element {
    let navigator = use_navigator();

    // Redirect if not authenticated
    use_effect(move || {
        if !is_authenticated() {
            navigator.push(Route::AdminLogin {});
        }
    });

    let settings = load_settings();
    let articles = load_articles();

    let published_count = articles
        .articles
        .iter()
        .filter(|a| matches!(a.status, ArticleStatus::Published))
        .count();
    let draft_count = articles
        .articles
        .iter()
        .filter(|a| matches!(a.status, ArticleStatus::Draft))
        .count();
    let enabled_pages = settings.pages.iter().filter(|p| p.enabled).count();

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
                    li { class: "admin-nav-item active",
                        Link { to: Route::AdminDashboard {}, class: "admin-nav-link", "📊 Dashboard" }
                    }
                    li { class: "admin-nav-item",
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
                    h1 { "Dashboard" }
                    p { class: "admin-subtitle", "Welcome back, {settings.brand.name}" }
                }

                div { class: "admin-stats-grid",
                    // Articles Stats
                    div { class: "admin-stat-card glass-card",
                        div { class: "admin-stat-icon", "📝" }
                        div { class: "admin-stat-content",
                            h3 { class: "admin-stat-value", "{articles.articles.len()}" }
                            p { class: "admin-stat-label", "Total Articles" }
                        }
                    }

                    div { class: "admin-stat-card glass-card",
                        div { class: "admin-stat-icon", "✅" }
                        div { class: "admin-stat-content",
                            h3 { class: "admin-stat-value", "{published_count}" }
                            p { class: "admin-stat-label", "Published" }
                        }
                    }

                    div { class: "admin-stat-card glass-card",
                        div { class: "admin-stat-icon", "📋" }
                        div { class: "admin-stat-content",
                            h3 { class: "admin-stat-value", "{draft_count}" }
                            p { class: "admin-stat-label", "Drafts" }
                        }
                    }

                    div { class: "admin-stat-card glass-card",
                        div { class: "admin-stat-icon", "📄" }
                        div { class: "admin-stat-content",
                            h3 { class: "admin-stat-value", "{enabled_pages}" }
                            p { class: "admin-stat-label", "Active Pages" }
                        }
                    }
                }

                // Quick Actions
                div { class: "admin-section",
                    h2 { "Quick Actions" }
                    div { class: "admin-actions-grid",
                        Link {
                            to: Route::AdminArticleNew {},
                            class: "admin-action-card glass-card",
                            div { class: "admin-action-icon", "✏️" }
                            span { "New Article" }
                        }
                        Link {
                            to: Route::AdminSettings {},
                            class: "admin-action-card glass-card",
                            div { class: "admin-action-icon", "🎨" }
                            span { "Edit Theme" }
                        }
                        Link {
                            to: Route::AdminArticles {},
                            class: "admin-action-card glass-card",
                            div { class: "admin-action-icon", "📚" }
                            span { "Manage Articles" }
                        }
                    }
                }

                // Recent Articles
                div { class: "admin-section",
                    h2 { "Recent Articles" }
                    div { class: "admin-table-container glass-card",
                        if articles.articles.is_empty() {
                            p { class: "admin-empty", "No articles yet. Create your first one!" }
                        } else {
                            table { class: "admin-table",
                                thead {
                                    tr {
                                        th { "Title" }
                                        th { "Date" }
                                        th { "Status" }
                                        th { "Actions" }
                                    }
                                }
                                tbody {
                                    for article in articles.articles.iter().take(5) {
                                        tr { key: "{article.id}",
                                            td { "{article.title}" }
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
                                            td {
                                                Link {
                                                    to: Route::AdminArticleEdit { id: article.id.clone() },
                                                    class: "admin-table-action",
                                                    "Edit"
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
