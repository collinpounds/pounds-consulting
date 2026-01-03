use crate::content::{
    is_authenticated, load_articles, save_articles, set_authenticated, Article, ArticleStatus,
};
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn AdminArticleNew() -> Element {
    rsx! {
        ArticleEditorInner { article_id: None }
    }
}

#[component]
pub fn AdminArticleEdit(id: String) -> Element {
    rsx! {
        ArticleEditorInner { article_id: Some(id) }
    }
}

#[component]
fn ArticleEditorInner(article_id: Option<String>) -> Element {
    let navigator = use_navigator();
    let is_new = article_id.is_none();

    // Redirect if not authenticated
    use_effect(move || {
        if !is_authenticated() {
            navigator.push(Route::AdminLogin {});
        }
    });

    // Load existing article or create new one
    let initial_article = {
        let articles_data = load_articles();
        if let Some(ref id) = article_id {
            articles_data
                .articles
                .iter()
                .find(|a| &a.id == id)
                .cloned()
                .unwrap_or_else(Article::new)
        } else {
            Article::new()
        }
    };

    let mut article = use_signal(|| initial_article);
    let mut saved_message = use_signal(|| Option::<String>::None);

    let mut do_save = move || {
        let mut articles_data = load_articles();

        // Update slug if title changed
        if article().slug.is_empty() {
            article.with_mut(|a| {
                a.slug = Article::generate_slug(&a.title);
            });
        }

        // Find and update or add
        if let Some(pos) = articles_data
            .articles
            .iter()
            .position(|a| a.id == article().id)
        {
            articles_data.articles[pos] = article();
        } else {
            articles_data.articles.push(article());
        }

        save_articles(&articles_data);
        saved_message.set(Some("Article saved!".to_string()));

        // Clear message after delay
        spawn(async move {
            gloo_timers::future::TimeoutFuture::new(1500).await;
            saved_message.set(None);
        });
    };

    let handle_save = move |_: MouseEvent| {
        do_save();
    };

    let handle_publish = move |_: MouseEvent| {
        article.with_mut(|a| a.status = ArticleStatus::Published);
        do_save();
    };

    let handle_logout = move |_| {
        set_authenticated(false);
        navigator.push(Route::AdminLogin {});
    };

    let update_title = move |evt: FormEvent| {
        let title = evt.value();
        article.with_mut(|a| {
            a.title = title.clone();
            // Auto-generate slug if it hasn't been manually set
            if a.slug.is_empty() || a.slug == Article::generate_slug(&a.title) {
                a.slug = Article::generate_slug(&title);
            }
        });
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
                    div { class: "admin-header-back",
                        Link { to: Route::AdminArticles {}, class: "admin-back-btn", "← Back to Articles" }
                    }
                    h1 { {if is_new { "New Article" } else { "Edit Article" }} }
                    div { class: "admin-header-actions",
                        if let Some(msg) = saved_message() {
                            span { class: "admin-success-message", "{msg}" }
                        }
                        button {
                            class: "btn btn-secondary",
                            onclick: handle_save,
                            "Save Draft"
                        }
                        button {
                            class: "btn btn-primary",
                            onclick: handle_publish,
                            "Publish"
                        }
                    }
                }

                div { class: "admin-editor-layout",
                    // Main Editor
                    div { class: "admin-editor-main",
                        div { class: "admin-form-card glass-card",
                            div { class: "form-group",
                                label { class: "form-label", "Title" }
                                input {
                                    class: "form-input form-input-large",
                                    r#type: "text",
                                    placeholder: "Article title",
                                    value: "{article().title}",
                                    oninput: update_title
                                }
                            }

                            div { class: "form-group",
                                label { class: "form-label", "Content" }
                                textarea {
                                    class: "form-textarea form-textarea-large",
                                    placeholder: "Write your article content here...\n\nYou can use plain text or basic formatting.",
                                    rows: "20",
                                    value: "{article().content}",
                                    oninput: move |evt: FormEvent| {
                                        article.with_mut(|a| a.content = evt.value());
                                    }
                                }
                            }
                        }
                    }

                    // Sidebar Meta
                    div { class: "admin-editor-sidebar",
                        div { class: "admin-form-card glass-card",
                            h3 { "Article Details" }

                            div { class: "form-group",
                                label { class: "form-label", "Slug" }
                                input {
                                    class: "form-input",
                                    r#type: "text",
                                    placeholder: "article-url-slug",
                                    value: "{article().slug}",
                                    oninput: move |evt: FormEvent| {
                                        article.with_mut(|a| a.slug = evt.value());
                                    }
                                }
                            }

                            div { class: "form-group",
                                label { class: "form-label", "Date" }
                                input {
                                    class: "form-input",
                                    r#type: "date",
                                    value: "{article().date}",
                                    oninput: move |evt: FormEvent| {
                                        article.with_mut(|a| a.date = evt.value());
                                    }
                                }
                            }

                            div { class: "form-group",
                                label { class: "form-label", "Category" }
                                input {
                                    class: "form-input",
                                    r#type: "text",
                                    placeholder: "e.g., Technology, News",
                                    value: "{article().category}",
                                    oninput: move |evt: FormEvent| {
                                        article.with_mut(|a| a.category = evt.value());
                                    }
                                }
                            }

                            div { class: "form-group",
                                label { class: "form-label", "Excerpt" }
                                textarea {
                                    class: "form-textarea",
                                    placeholder: "Brief description for article previews",
                                    rows: "3",
                                    value: "{article().excerpt}",
                                    oninput: move |evt: FormEvent| {
                                        article.with_mut(|a| a.excerpt = evt.value());
                                    }
                                }
                            }

                            div { class: "form-group",
                                label { class: "form-label", "Status" }
                                div { class: "status-toggle",
                                    button {
                                        class: if matches!(article().status, ArticleStatus::Draft) {
                                            "status-btn status-btn-active"
                                        } else {
                                            "status-btn"
                                        },
                                        onclick: move |_| {
                                            article.with_mut(|a| a.status = ArticleStatus::Draft);
                                        },
                                        "Draft"
                                    }
                                    button {
                                        class: if matches!(article().status, ArticleStatus::Published) {
                                            "status-btn status-btn-active"
                                        } else {
                                            "status-btn"
                                        },
                                        onclick: move |_| {
                                            article.with_mut(|a| a.status = ArticleStatus::Published);
                                        },
                                        "Published"
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
