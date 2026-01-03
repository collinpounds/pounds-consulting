use dioxus::prelude::*;
use crate::content::{load_articles, ArticleStatus};
use crate::Route;
use crate::components::CtaSection;

#[component]
pub fn Articles() -> Element {
    let articles_data = load_articles();
    let published: Vec<_> = articles_data.articles
        .into_iter()
        .filter(|a| matches!(a.status, ArticleStatus::Published))
        .collect();

    // Get unique categories
    let categories: Vec<String> = {
        let mut cats: Vec<String> = published.iter()
            .map(|a| a.category.clone())
            .collect();
        cats.sort();
        cats.dedup();
        cats
    };

    let mut selected_category = use_signal(|| Option::<String>::None);

    let filtered_articles: Vec<_> = if let Some(ref cat) = selected_category() {
        published.iter().filter(|a| &a.category == cat).collect()
    } else {
        published.iter().collect()
    };

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
                            onclick: move |_| selected_category.set(None),
                            "All"
                        }
                        for cat in categories.iter() {
                            button {
                                key: "{cat}",
                                class: if selected_category().as_ref() == Some(cat) { "filter-btn filter-btn-active" } else { "filter-btn" },
                                onclick: {
                                    let cat = cat.clone();
                                    move |_| selected_category.set(Some(cat.clone()))
                                },
                                "{cat}"
                            }
                        }
                    }
                }

                // Articles Grid
                if filtered_articles.is_empty() {
                    div { class: "articles-empty glass-card",
                        h3 { "No articles yet" }
                        p { "Check back soon for new content." }
                    }
                } else {
                    div { class: "articles-grid",
                        for article in filtered_articles {
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
