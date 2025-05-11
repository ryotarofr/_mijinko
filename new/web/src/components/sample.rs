use dioxus::prelude::*;

use crate::parser::parser::{LineState, Render, _LocalLineHistory};

#[component]
pub fn Sample() -> Element {
    let mut input = use_signal(|| "# h1\n## h2\n### h3\n paraglaph\n> hello\n".to_string());
    let mut history = use_signal(_LocalLineHistory::default);

    let insert_elements = LineState::split_lines(input.read().to_string())
        .into_iter()
        .map(|line| {
            let state = LineState::from(line.clone());
            history.write().insert(state.clone());

            LineState::render_state_rsx(&state)
        });

    // ↓ discord にあったサンプル
    // let mut nodes = match dynamic_nodes[0] {
    //     DynamicNode::Fragment(ref nodes) => nodes.clone(),
    //     _ => Vec::with_capacity(1),
    // };
    // let node = rsx! {
    //     div { "hello" }
    // }
    // .unwrap();
    // nodes.push(node);
    // let dnodes = Box::new([DynamicNode::Fragment(nodes)]);
    // let new_node = VNode::new(key, template, dnodes, dynamic_attrs);

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
