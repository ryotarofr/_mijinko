use dioxus::{history, prelude::*};

use crate::parser::parser::{LineState, LineType, LocalLineHistory};
use crate::parser::utils::split_lines;

/// State の構造体を html に変換する
fn render_state(state: &LineState) -> Element {
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
        LineType::Quote => rsx! {
            blockquote { {state.input} }
        },
    }
}

#[component]
pub fn Sample() -> Element {
    let input = use_signal(|| "# h1\n## h2\n### h3\n paraglaph\n> hello\n");
    let mut history = use_signal(LocalLineHistory::default);

    let insert_element = split_lines(*input.read()).into_iter().map(|line| {
        let state = LineState::from(line);
        history.write().insert(state.clone());

        render_state(&state)
    });

    rsx! {
        div {
            h1 { "Sample" }
            div {
                {insert_element}
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
