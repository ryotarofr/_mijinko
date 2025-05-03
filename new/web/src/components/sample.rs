use dioxus::prelude::*;

use crate::parser::parser::{split_lines, LineType, State};

/// State の構造体を html に変換する
fn render_state(state: &State) -> Element {
    match state.line_type {
        LineType::Cursor | LineType::Paragraph => rsx! {
            p { {state.input} }
        },
        LineType::Hedding => rsx! {
            h2 { {state.input} }
        },
        LineType::Code => rsx! {
            pre { code { {state.input} } }
        },
    }
}

#[component]
pub fn Sample() -> Element {
    let input = use_signal(|| "# 0\n## 1\n### 2\n 3\n");
    let insert_element = split_lines(*input.read()).into_iter().map(|line| {
        let state = State::from(line);
        render_state(&state)
    });

    rsx! {
        div {
            h1 { "Sample" }
            div {
                {insert_element}
            }
        }
    }
}
