use dioxus::prelude::*;

mod components;
mod pages;

use components::{Footer, Header};
use pages::{About, Contact, Home, Portfolio, Services};

const CSS: Asset = asset!("/assets/main.css");

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Layout)]
    #[route("/")]
    Home {},
    #[route("/about")]
    About {},
    #[route("/services")]
    Services {},
    #[route("/portfolio")]
    Portfolio {},
    #[route("/contact")]
    Contact {},
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: CSS }
        document::Link {
            rel: "preconnect",
            href: "https://fonts.googleapis.com"
        }
        document::Link {
            rel: "preconnect",
            href: "https://fonts.gstatic.com",
            crossorigin: "anonymous"
        }
        document::Link {
            rel: "stylesheet",
            href: "https://fonts.googleapis.com/css2?family=Inter:wght@500;600&family=Montserrat:wght@700&family=Open+Sans:wght@400;500&display=swap"
        }
        Router::<Route> {}
    }
}

#[component]
fn Layout() -> Element {
    rsx! {
        Header {}
        main {
            Outlet::<Route> {}
        }
        Footer {}
    }
}
