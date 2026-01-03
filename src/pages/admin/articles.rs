use crate::content::{
    is_authenticated, load_articles, save_articles, set_authenticated, Article, ArticleStatus,
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
    let mut selected_article_id = use_signal(|| Option::<String>::None);
    let mut show_trash = use_signal(|| false);
    let mut delete_confirm = use_signal(|| Option::<String>::None);
    let mut permanent_delete_confirm = use_signal(|| Option::<String>::None);

    // Get the currently selected article
    let selected_article = {
        let data = articles_data();
        let id = selected_article_id();
        if let Some(ref article_id) = id {
            data.articles.iter().find(|a| &a.id == article_id).cloned()
        } else {
            None
        }
    };

    // Filter articles based on trash view
    let visible_articles: Vec<Article> = {
        let data = articles_data();
        if show_trash() {
            data.articles
                .iter()
                .filter(|a| matches!(a.status, ArticleStatus::Trashed))
                .cloned()
                .collect()
        } else {
            data.articles
                .iter()
                .filter(|a| !matches!(a.status, ArticleStatus::Trashed))
                .cloned()
                .collect()
        }
    };

    // Move to trash (soft delete)
    let mut move_to_trash = move |id: String| {
        articles_data.with_mut(|data| {
            if let Some(article) = data.articles.iter_mut().find(|a| a.id == id) {
                article.status = ArticleStatus::Trashed;
            }
        });
        save_articles(&articles_data());
        selected_article_id.set(None);
        delete_confirm.set(None);
    };

    // Restore from trash
    let mut restore_from_trash = move |id: String| {
        articles_data.with_mut(|data| {
            if let Some(article) = data.articles.iter_mut().find(|a| a.id == id) {
                article.status = ArticleStatus::Draft;
            }
        });
        save_articles(&articles_data());
        selected_article_id.set(None);
    };

    // Permanent delete
    let mut permanent_delete = move |id: String| {
        articles_data.with_mut(|data| {
            data.articles.retain(|a| a.id != id);
        });
        save_articles(&articles_data());
        selected_article_id.set(None);
        permanent_delete_confirm.set(None);
    };

    // Empty trash
    let empty_trash = move |_| {
        articles_data.with_mut(|data| {
            data.articles
                .retain(|a| !matches!(a.status, ArticleStatus::Trashed));
        });
        save_articles(&articles_data());
        selected_article_id.set(None);
    };

    let handle_logout = move |_: MouseEvent| {
        set_authenticated(false);
        navigator.push(Route::AdminLogin {});
    };

    let trash_count = articles_data()
        .articles
        .iter()
        .filter(|a| matches!(a.status, ArticleStatus::Trashed))
        .count();

    rsx! {
        div { class: "admin-layout",
            // Main Sidebar (Navigation)
            nav { class: "admin-sidebar",
                div { class: "admin-sidebar-header",
                    h2 { class: "admin-logo", "Admin" }
                }

                ul { class: "admin-nav",
                    li { class: "admin-nav-item",
                        Link { to: Route::AdminDashboard {}, class: "admin-nav-link", "Dashboard" }
                    }
                    li { class: "admin-nav-item",
                        Link { to: Route::AdminSettings {}, class: "admin-nav-link", "Settings" }
                    }
                    li { class: "admin-nav-item active",
                        Link { to: Route::AdminArticles {}, class: "admin-nav-link", "Articles" }
                    }
                }

                div { class: "admin-sidebar-footer",
                    a { href: "/", class: "admin-nav-link", "View Site" }
                    button {
                        class: "btn btn-secondary btn-full",
                        onclick: handle_logout,
                        "Logout"
                    }
                }
            }

            // Articles List Sidebar
            aside { class: "articles-list-sidebar",
                div { class: "articles-list-header",
                    h3 { "Articles" }
                    Link {
                        to: Route::AdminArticleNew {},
                        class: "btn btn-primary btn-sm",
                        "+ New"
                    }
                }

                // View toggle
                div { class: "articles-view-toggle",
                    button {
                        class: if !show_trash() { "view-toggle-btn active" } else { "view-toggle-btn" },
                        onclick: move |_| show_trash.set(false),
                        "All Articles"
                    }
                    button {
                        class: if show_trash() { "view-toggle-btn active" } else { "view-toggle-btn" },
                        onclick: move |_| {
                            show_trash.set(true);
                            selected_article_id.set(None);
                        },
                        "Trash ({trash_count})"
                    }
                }

                // Articles list
                div { class: "articles-list",
                    if visible_articles.is_empty() {
                        div { class: "articles-list-empty",
                            if show_trash() {
                                p { "Trash is empty" }
                            } else {
                                p { "No articles yet" }
                                Link {
                                    to: Route::AdminArticleNew {},
                                    class: "btn btn-primary btn-sm",
                                    "Create your first article"
                                }
                            }
                        }
                    } else {
                        for article in visible_articles.iter() {
                            div {
                                key: "{article.id}",
                                class: if selected_article_id() == Some(article.id.clone()) {
                                    "article-list-item selected"
                                } else {
                                    "article-list-item"
                                },
                                onclick: {
                                    let id = article.id.clone();
                                    move |_| selected_article_id.set(Some(id.clone()))
                                },

                                div { class: "article-list-item-title",
                                    "{article.title}"
                                    if article.title.is_empty() {
                                        span { class: "untitled", "(Untitled)" }
                                    }
                                }
                                div { class: "article-list-item-meta",
                                    span { class: "article-list-date", "{article.date}" }
                                    span {
                                        class: if matches!(article.status, ArticleStatus::Published) {
                                            "article-list-status published"
                                        } else if matches!(article.status, ArticleStatus::Trashed) {
                                            "article-list-status trashed"
                                        } else {
                                            "article-list-status draft"
                                        },
                                        {match article.status {
                                            ArticleStatus::Published => "Published",
                                            ArticleStatus::Draft => "Draft",
                                            ArticleStatus::Trashed => "Trashed",
                                        }}
                                    }
                                }
                            }
                        }
                    }
                }

                // Empty trash button
                if show_trash() && trash_count > 0 {
                    div { class: "articles-list-footer",
                        button {
                            class: "btn btn-danger btn-sm btn-full",
                            onclick: empty_trash,
                            "Empty Trash"
                        }
                    }
                }
            }

            // Main Content - Article Preview/Actions
            main { class: "admin-main articles-main",
                if let Some(article) = selected_article {
                    div { class: "article-preview-panel",
                        div { class: "article-preview-header",
                            h2 { class: "article-preview-title",
                                if article.title.is_empty() {
                                    "(Untitled)"
                                } else {
                                    "{article.title}"
                                }
                            }
                            div { class: "article-preview-meta",
                                span { class: "preview-category", "{article.category}" }
                                span { class: "preview-date", "{article.date}" }
                                span {
                                    class: if matches!(article.status, ArticleStatus::Published) {
                                        "preview-status published"
                                    } else if matches!(article.status, ArticleStatus::Trashed) {
                                        "preview-status trashed"
                                    } else {
                                        "preview-status draft"
                                    },
                                    {match article.status {
                                        ArticleStatus::Published => "Published",
                                        ArticleStatus::Draft => "Draft",
                                        ArticleStatus::Trashed => "Trashed",
                                    }}
                                }
                            }
                        }

                        div { class: "article-preview-actions",
                            if matches!(article.status, ArticleStatus::Trashed) {
                                // Trash actions
                                button {
                                    class: "btn btn-primary",
                                    onclick: {
                                        let id = article.id.clone();
                                        move |_| restore_from_trash(id.clone())
                                    },
                                    "Restore"
                                }
                                button {
                                    class: "btn btn-danger",
                                    onclick: {
                                        let id = article.id.clone();
                                        move |_| permanent_delete_confirm.set(Some(id.clone()))
                                    },
                                    "Delete Permanently"
                                }
                            } else {
                                // Normal actions
                                Link {
                                    to: Route::AdminArticleEdit { id: article.id.clone() },
                                    class: "btn btn-primary",
                                    "Edit Article"
                                }
                                button {
                                    class: "btn btn-danger-outline",
                                    onclick: {
                                        let id = article.id.clone();
                                        move |_| delete_confirm.set(Some(id.clone()))
                                    },
                                    "Move to Trash"
                                }
                            }
                        }

                        div { class: "article-preview-content",
                            if !article.excerpt.is_empty() {
                                div { class: "article-preview-excerpt",
                                    h4 { "Excerpt" }
                                    p { "{article.excerpt}" }
                                }
                            }

                            div { class: "article-preview-body",
                                h4 { "Content Preview" }
                                div { class: "content-preview-text",
                                    {if article.content.is_empty() {
                                        "No content yet..."
                                    } else if article.content.len() > 500 {
                                        &article.content[..500]
                                    } else {
                                        &article.content
                                    }}
                                    if article.content.len() > 500 {
                                        span { class: "content-ellipsis", "..." }
                                    }
                                }
                            }

                            div { class: "article-preview-info",
                                h4 { "Details" }
                                dl { class: "info-list",
                                    dt { "URL Slug" }
                                    dd { "/{article.slug}" }
                                    dt { "Word Count" }
                                    dd { "{article.content.split_whitespace().count()} words" }
                                }
                            }
                        }
                    }
                } else {
                    div { class: "article-no-selection",
                        div { class: "no-selection-content",
                            div { class: "no-selection-icon", "📝" }
                            h3 { "Select an article" }
                            p { "Choose an article from the list to view details and actions" }
                            Link {
                                to: Route::AdminArticleNew {},
                                class: "btn btn-primary",
                                "Create New Article"
                            }
                        }
                    }
                }
            }
        }

        // Move to Trash Confirmation Modal
        if let Some(id) = delete_confirm() {
            div { class: "admin-modal-overlay",
                onclick: move |_| delete_confirm.set(None),
                div {
                    class: "admin-modal glass-card",
                    onclick: move |e| e.stop_propagation(),
                    div { class: "modal-icon warning", "🗑️" }
                    h3 { "Move to Trash?" }
                    p { "This article will be moved to the trash. You can restore it later." }
                    div { class: "admin-modal-actions",
                        button {
                            class: "btn btn-secondary",
                            onclick: move |_| delete_confirm.set(None),
                            "Cancel"
                        }
                        button {
                            class: "btn btn-danger",
                            onclick: move |_| move_to_trash(id.clone()),
                            "Move to Trash"
                        }
                    }
                }
            }
        }

        // Permanent Delete Confirmation Modal
        if let Some(id) = permanent_delete_confirm() {
            div { class: "admin-modal-overlay",
                onclick: move |_| permanent_delete_confirm.set(None),
                div {
                    class: "admin-modal glass-card",
                    onclick: move |e| e.stop_propagation(),
                    div { class: "modal-icon danger", "⚠️" }
                    h3 { "Delete Permanently?" }
                    p { "This action cannot be undone. The article will be permanently deleted." }
                    div { class: "admin-modal-actions",
                        button {
                            class: "btn btn-secondary",
                            onclick: move |_| permanent_delete_confirm.set(None),
                            "Cancel"
                        }
                        button {
                            class: "btn btn-danger",
                            onclick: move |_| permanent_delete(id.clone()),
                            "Delete Forever"
                        }
                    }
                }
            }
        }
    }
}
