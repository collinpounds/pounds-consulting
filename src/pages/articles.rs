use crate::components::CtaSection;
use crate::content::{load_articles, ArticleStatus};
use crate::Route;
use dioxus::prelude::*;

const ARTICLES_PER_PAGE: usize = 10;

#[component]
pub fn Articles() -> Element {
    let articles_data = load_articles();
    let published: Vec<_> = articles_data
        .articles
        .into_iter()
        .filter(|a| matches!(a.status, ArticleStatus::Published))
        .collect();

    // Get unique categories
    let categories: Vec<String> = {
        let mut cats: Vec<String> = published.iter().map(|a| a.category.clone()).collect();
        cats.sort();
        cats.dedup();
        cats
    };

    let mut selected_category = use_signal(|| Option::<String>::None);
    let mut current_page = use_signal(|| 1usize);

    // Reset to page 1 when category changes
    let filtered_articles: Vec<_> = if let Some(ref cat) = selected_category() {
        published.iter().filter(|a| &a.category == cat).collect()
    } else {
        published.iter().collect()
    };

    // Pagination calculations
    let total_articles = filtered_articles.len();
    let total_pages = total_articles.div_ceil(ARTICLES_PER_PAGE);
    let page = current_page().min(total_pages.max(1));
    let start_idx = (page - 1) * ARTICLES_PER_PAGE;
    let paginated_articles: Vec<_> = filtered_articles
        .into_iter()
        .skip(start_idx)
        .take(ARTICLES_PER_PAGE)
        .collect();

    rsx! {
        // Hero Section
        section { class: "hero hero-short",
            div { class: "hero-content",
                h1 { class: "hero-title", "Articles" }
                p { class: "hero-subtitle",
                    "Insights, updates, and perspectives on technology and business."
                }
            }
        }

        // Articles Section
        section { class: "section articles-section",
            div { class: "container",
                // Category Filter
                if !categories.is_empty() {
                    div { class: "articles-filter",
                        button {
                            class: if selected_category().is_none() { "filter-btn filter-btn-active" } else { "filter-btn" },
                            onclick: move |_| {
                                selected_category.set(None);
                                current_page.set(1);
                            },
                            "All"
                        }
                        for cat in categories.iter() {
                            button {
                                key: "{cat}",
                                class: if selected_category().as_ref() == Some(cat) { "filter-btn filter-btn-active" } else { "filter-btn" },
                                onclick: {
                                    let cat = cat.clone();
                                    move |_| {
                                        selected_category.set(Some(cat.clone()));
                                        current_page.set(1);
                                    }
                                },
                                "{cat}"
                            }
                        }
                    }
                }

                // Articles Grid
                if paginated_articles.is_empty() {
                    div { class: "articles-empty glass-card",
                        h3 { "No articles yet" }
                        p { "Check back soon for new content." }
                    }
                } else {
                    div { class: "articles-grid",
                        for article in paginated_articles {
                            Link {
                                key: "{article.id}",
                                to: Route::ArticleDetail { slug: article.slug.clone() },
                                class: "article-card glass-card",

                                div { class: "article-card-header",
                                    span { class: "article-category", "{article.category}" }
                                    span { class: "article-date", "{article.date}" }
                                }

                                h2 { class: "article-card-title", "{article.title}" }

                                p { class: "article-card-excerpt", "{article.excerpt}" }

                                span { class: "article-read-more", "Read more →" }
                            }
                        }
                    }

                    // Pagination Controls
                    if total_pages > 1 {
                        div { class: "articles-pagination",
                            button {
                                class: "pagination-btn",
                                disabled: page <= 1,
                                onclick: move |_| {
                                    if page > 1 {
                                        current_page.set(page - 1);
                                    }
                                },
                                "← Previous"
                            }
                            span { class: "pagination-info",
                                "Page {page} of {total_pages}"
                            }
                            button {
                                class: "pagination-btn",
                                disabled: page >= total_pages,
                                onclick: move |_| {
                                    if page < total_pages {
                                        current_page.set(page + 1);
                                    }
                                },
                                "Next →"
                            }
                        }
                    }
                }
            }
        }

        // CTA Section
        CtaSection {
            title: "Have a Question?".to_string(),
            description: "We're always happy to discuss technology, business, or potential projects.".to_string(),
            button_text: "Get in Touch".to_string(),
            use_calendar_link: true
        }
    }
}
