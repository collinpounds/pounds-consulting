use crate::content::{load_articles, ArticleStatus};
use crate::Route;
use dioxus::prelude::*;

/// Convert **bold** markers to <strong> tags for HTML rendering
fn process_bold_markers(text: &str) -> String {
    let mut result = text.to_string();
    while let (Some(start), Some(end)) = (
        result.find("**"),
        result[result.find("**").unwrap_or(0) + 2..].find("**"),
    ) {
        let end_pos = start + 2 + end + 2;
        let bold_text = &result[start + 2..start + 2 + end];
        result = format!(
            "{}<strong>{}</strong>{}",
            &result[..start],
            bold_text,
            &result[end_pos..]
        );
    }
    result
}

#[component]
pub fn ArticleDetail(slug: String) -> Element {
    let articles_data = load_articles();
    let article = articles_data
        .articles
        .iter()
        .find(|a| a.slug == slug && matches!(a.status, ArticleStatus::Published));

    match article {
        Some(article) => {
            rsx! {
                // Article Header
                section { class: "article-hero",
                    div { class: "container",
                        Link { to: Route::Articles {}, class: "article-back-link", "← Back to Articles" }

                        div { class: "article-meta",
                            span { class: "article-category-badge", "{article.category}" }
                            span { class: "article-date", "{article.date}" }
                        }

                        h1 { class: "article-title", "{article.title}" }

                        if !article.excerpt.is_empty() {
                            p { class: "article-excerpt", "{article.excerpt}" }
                        }
                    }
                }

                // Article Content
                section { class: "article-content-section",
                    div { class: "container",
                        div { class: "article-body glass-card",
                            // Render content with markdown-like formatting
                            for paragraph in article.content.split("\n\n") {
                                if paragraph.starts_with("### ") {
                                    h3 { class: "article-h3",
                                        dangerous_inner_html: "{process_bold_markers(paragraph.trim_start_matches(\"### \"))}"
                                    }
                                } else if paragraph.starts_with("## ") {
                                    h2 { class: "article-h2",
                                        dangerous_inner_html: "{process_bold_markers(paragraph.trim_start_matches(\"## \"))}"
                                    }
                                } else if paragraph.starts_with("# ") {
                                    h2 { class: "article-h2",
                                        dangerous_inner_html: "{process_bold_markers(paragraph.trim_start_matches(\"# \"))}"
                                    }
                                } else if paragraph.starts_with("- ") || paragraph.starts_with("* ") {
                                    // Render as a list
                                    ul { class: "article-list",
                                        for line in paragraph.lines() {
                                            if line.starts_with("- ") || line.starts_with("* ") {
                                                li {
                                                    dangerous_inner_html: "{process_bold_markers(line.trim_start_matches(\"- \").trim_start_matches(\"* \"))}"
                                                }
                                            }
                                        }
                                    }
                                } else if paragraph.starts_with("**") && paragraph.ends_with("**") {
                                    // Bold standalone line (like "**How to avoid it:**")
                                    p { class: "article-bold",
                                        dangerous_inner_html: "{paragraph.trim_matches('*')}"
                                    }
                                } else if !paragraph.trim().is_empty() {
                                    // Handle bold text and links within paragraph
                                    p { dangerous_inner_html: "{process_bold_markers(paragraph)}" }
                                }
                            }
                        }

                        // Share Section
                        div { class: "article-share",
                            span { class: "share-label", "Share this article:" }
                            div { class: "share-buttons",
                                a {
                                    href: "https://twitter.com/intent/tweet?text={article.title}&url=",
                                    target: "_blank",
                                    rel: "noopener noreferrer",
                                    class: "share-btn",
                                    "Twitter"
                                }
                                a {
                                    href: "https://www.linkedin.com/sharing/share-offsite/?url=",
                                    target: "_blank",
                                    rel: "noopener noreferrer",
                                    class: "share-btn",
                                    "LinkedIn"
                                }
                                button {
                                    class: "share-btn",
                                    onclick: move |_| {
                                        // Copy URL to clipboard using eval for simplicity
                                        #[cfg(target_arch = "wasm32")]
                                        {
                                            let _ = js_sys::eval("navigator.clipboard.writeText(window.location.href)");
                                        }
                                    },
                                    "Copy Link"
                                }
                            }
                        }
                    }
                }

                // More Articles
                section { class: "section related-articles",
                    div { class: "container",
                        h2 { class: "section-title", "More Articles" }
                        div { class: "articles-grid",
                            for other in articles_data.articles.iter()
                                .filter(|a| a.slug != slug && matches!(a.status, ArticleStatus::Published))
                                .take(3)
                            {
                                Link {
                                    key: "{other.id}",
                                    to: Route::ArticleDetail { slug: other.slug.clone() },
                                    class: "article-card glass-card",

                                    div { class: "article-card-header",
                                        span { class: "article-category", "{other.category}" }
                                        span { class: "article-date", "{other.date}" }
                                    }

                                    h3 { class: "article-card-title", "{other.title}" }

                                    p { class: "article-card-excerpt", "{other.excerpt}" }

                                    span { class: "article-read-more", "Read more →" }
                                }
                            }
                        }
                    }
                }
            }
        }
        None => {
            rsx! {
                section { class: "hero hero-short",
                    div { class: "hero-content",
                        h1 { class: "hero-title", "Article Not Found" }
                        p { class: "hero-subtitle", "The article you're looking for doesn't exist." }
                        Link { to: Route::Articles {}, class: "btn btn-primary", "View All Articles" }
                    }
                }
            }
        }
    }
}
