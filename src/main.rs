use components::{editor::Editor, home_editor::HomeEditor};
use context::theme_context::ThemeProvider;
use dioxus::prelude::*;

mod components;
mod config;
mod context;
mod r#fn;
mod lib;
mod types;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    // #[layout(Navbar)]
    #[route("/")]
    Home {},
    #[route("/memo/:id")]
    Editor {id: i32},
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");

fn main() {
    dioxus::logger::initialize_default();

    tracing::info!("Logs received!");

    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        ThemeProvider { Router::<Route> {} }
    }
}

#[component]
pub fn Hero() -> Element {
    rsx! {
        div { id: "hero",
            img { src: HEADER_SVG, id: "header" }
            div { id: "links",
                a { href: "https://dioxuslabs.com/learn/0.6/", "📚 Learn Dioxus" }
                a { href: "https://dioxuslabs.com/awesome", "🚀 Awesome Dioxus" }
                a { href: "https://github.com/dioxus-community/", "📡 Community Libraries" }
                a { href: "https://github.com/DioxusLabs/sdk", "⚙️ Dioxus Development Kit" }
                a { href: "https://marketplace.visualstudio.com/items?itemName=DioxusLabs.dioxus",
                    "💫 VSCode Extension"
                }
                a { href: "https://discord.gg/XgGxMSkvUM", "👋 Community Discord" }
            }
        }
    }
}

/// Home page
#[component]
fn Home() -> Element {
    rsx! {
        HomeEditor {}
    }
}

/// Blog page
// #[component]
// pub fn Blog(id: i32) -> Element {
//     rsx! {
//         div { id: "blog",

//             // Content
//             h1 { "This is blog #{id}!" }
//             p {
//                 "In blog #{id}, we show how the Dioxus router works and how URL parameters can be passed as props to our route components."
//             }

//             // Navigation links
//             Link { to: Route::Blog { id: id - 1 }, "Previous" }
//             span { " <---> " }
//             Link { to: Route::Blog { id: id + 1 }, "Next" }
//         }
//     }
// }

/// Shared navbar component.
#[component]
fn Navbar() -> Element {
    rsx! {
        div { id: "navbar",
            Link { to: Route::Home {},
                {}
                "Home"
            }
            Link { to: Route::Editor { id: 1 }, "Blog" }
        }

        Outlet::<Route> {}
    }
}
