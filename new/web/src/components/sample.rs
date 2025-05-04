use dioxus::prelude::*;

use crate::parser::parser::{LineState, LineType, LocalLineHistory};
use crate::parser::utils::split_lines;

/// State の構造体を html に変換する
fn render_state(state: &LineState) -> Element {
    match state.line_type {
        LineType::Cursor | LineType::Paragraph => rsx! {
            p { {state.input.as_str()} }
        },
        LineType::Hedding => rsx! {
            h2 { {state.input.as_str()} }
        },
        LineType::Code => rsx! {
            pre { code { {state.input.as_str()} } }
        },
        LineType::Quote => rsx! {
            blockquote { {state.input.as_str()} }
        },
    }
}

#[component]
pub fn Sample() -> Element {
    let mut input = use_signal(|| "# h1\n## h2\n### h3\n paraglaph\n> hello\n".to_string());
    let mut history = use_signal(LocalLineHistory::default);

    let insert_elements = split_lines(input.read().to_string())
        .into_iter()
        .map(|line| {
            let state = LineState::from(line.clone());
            history.write().insert(state.clone());

            render_state(&state)
        });

    rsx! {
        div {
            div {
                contenteditable: "true",
                oninput: move |e| {
                    input.set(e.value());
                },
            }
            input {
            }
            div {
                {insert_elements}
            }
            div {
                p { "History" }
                span {
                    {history.read().current.to_string()}
                }
                div {
                    "History: "
                    {
                        history.read().generations.iter().map(|(k, v)| {
                            rsx!{
                                div {
                                    span { {k.to_string()} }
                                    span { " : " }
                                    span { {v.clone().input.to_string()} }
                                    span { ", " }
                                 }
                            }
                        })
                    }
                }
            }
        }
    }
}
