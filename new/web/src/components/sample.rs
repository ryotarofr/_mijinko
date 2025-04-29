use dioxus::prelude::*;

use crate::parser::parser::{LineType, State};

#[component]
pub fn Sample() -> Element {
    let parser = State::new("## 見出し(H2)");
    let insert_element = match parser.line_type {
        LineType::Cursor => rsx! { p { {parser.input} } },
        LineType::Paragraph => rsx! { p { {parser.input} } },
        LineType::Hedding => rsx! { h2 { {parser.input} } },
        LineType::Code => rsx! { pre { code { {parser.input} } } },
        _ => rsx! { p    { {parser.input} } },
    };

    rsx! {
        div {
            h1 { "Sample" }
            div {
                {insert_element}
            }
        }
    }
}
