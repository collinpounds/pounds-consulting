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

/// Represents a content block in an article
enum ContentBlock {
    H2(String),
    H3(String),
    Paragraph(String),
    List(Vec<String>),
    BoldLine(String),
}

/// Parse article content into structured blocks
fn parse_content(content: &str) -> Vec<ContentBlock> {
    let mut blocks = Vec::new();

    for paragraph in content.split("\n\n") {
        if paragraph.starts_with("### ") {
            // Handle h3 headings (may have body text on next line)
            let lines: Vec<&str> = paragraph.lines().collect();
            let heading = lines[0].trim_start_matches("### ");
            blocks.push(ContentBlock::H3(process_bold_markers(heading)));
            // Add remaining lines as paragraphs
            for line in lines.iter().skip(1) {
                if !line.trim().is_empty() {
                    blocks.push(ContentBlock::Paragraph(process_bold_markers(line)));
                }
            }
        } else if paragraph.starts_with("## ") {
            let lines: Vec<&str> = paragraph.lines().collect();
            let heading = lines[0].trim_start_matches("## ");
            blocks.push(ContentBlock::H2(process_bold_markers(heading)));
            for line in lines.iter().skip(1) {
                if !line.trim().is_empty() {
                    blocks.push(ContentBlock::Paragraph(process_bold_markers(line)));
                }
            }
        } else if paragraph.starts_with("# ") {
            let lines: Vec<&str> = paragraph.lines().collect();
            let heading = lines[0].trim_start_matches("# ");
            blocks.push(ContentBlock::H2(process_bold_markers(heading)));
            for line in lines.iter().skip(1) {
                if !line.trim().is_empty() {
                    blocks.push(ContentBlock::Paragraph(process_bold_markers(line)));
                }
            }
        } else if paragraph.starts_with("- ") || paragraph.starts_with("* ") {
            let items: Vec<String> = paragraph
                .lines()
                .filter(|line| line.starts_with("- ") || line.starts_with("* "))
                .map(|line| {
                    process_bold_markers(line.trim_start_matches("- ").trim_start_matches("* "))
                })
                .collect();
            blocks.push(ContentBlock::List(items));
        } else if paragraph.starts_with("**") && paragraph.ends_with("**") {
            blocks.push(ContentBlock::BoldLine(
                paragraph.trim_matches('*').to_string(),
            ));
        } else if !paragraph.trim().is_empty() {
            blocks.push(ContentBlock::Paragraph(process_bold_markers(paragraph)));
        }
    }

    blocks
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
            let content_blocks = parse_content(&article.content);

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
                            for (i, block) in content_blocks.iter().enumerate() {
                                match block {
                                    ContentBlock::H2(text) => rsx! {
                                        h2 { key: "{i}", class: "article-h2", dangerous_inner_html: "{text}" }
                                    },
                                    ContentBlock::H3(text) => rsx! {
                                        h3 { key: "{i}", class: "article-h3", dangerous_inner_html: "{text}" }
                                    },
                                    ContentBlock::Paragraph(text) => rsx! {
                                        p { key: "{i}", dangerous_inner_html: "{text}" }
                                    },
                                    ContentBlock::List(items) => rsx! {
                                        ul { key: "{i}", class: "article-list",
                                            for (j, item) in items.iter().enumerate() {
                                                li { key: "{j}", dangerous_inner_html: "{item}" }
                                            }
                                        }
                                    },
                                    ContentBlock::BoldLine(text) => rsx! {
                                        p { key: "{i}", class: "article-bold", dangerous_inner_html: "{text}" }
                                    },
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
