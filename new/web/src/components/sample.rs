use dioxus::prelude::*;

use crate::parser::parser::{LineType, State};

fn render_state(state: &State) -> Element {
    // 現在のラインを構築
    let this_line = match state.line_type {
        LineType::Cursor | LineType::Paragraph => rsx! {
            p { {state.input.as_str()} }
        },
        LineType::Hedding => rsx! {
            h2 { {state.input.as_str()} }
        },
        LineType::Code => rsx! {
            pre { code { {state.input.as_str()} } }
        },
    };

    // ── child があれば再帰的に下に連結
    if let Some(child) = &state.child {
        rsx! {
            div{
                { this_line }
                { render_state(child) }
            }
        }
    } else {
        this_line
    }
}

#[component]
pub fn Sample() -> Element {
    let parser = State::parse("# 0\n## 1\n### 2\n 3\n");
    let insert_element = render_state(&parser);

    rsx! {
        div {
            h1 { "Sample" }
            div {
                {insert_element}
            }
        }
    }
}
