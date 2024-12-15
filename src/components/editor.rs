use crate::config::constants::LOREM_IPSUM;
use crate::context::theme_context::Theme;
use crate::r#fn::editor_state::EditorState;
use crate::types::enums::{Direction, Glyph};
use dioxus::prelude::*;
use keyboard_types::{Code, Key, Modifiers};
use serde_json::Value;

macro_rules! code_events {
    ($event:ident, $editor:ident as $alias:ident,
     $(
         $type:ident => [
             $(for $match:pat => $fun:expr),+
         ]

     ),+) => {
        $(
        match $event.data.$type() {
            $(
                $match => {
                    $editor.with_mut(|$alias| { $fun });
                    $event.stop_propagation();
                    return;
                }
            )+
            _ => (),
        })+
    }
}

pub fn Editor() -> Element {
    let mut editor_state = use_signal(|| EditorState::from(LOREM_IPSUM));
    let mut theme = use_context::<Signal<Theme>>();
    let mut is_ime = use_signal(|| false);
    let mut last_key: Signal<Option<Code>> = use_signal(|| None);

    let mut ime_buffer = use_signal(String::new); // IMEの入力を一時的に保持するバッファ

    let editor_style = r#"
        flex: 1;
        outline: none;
        font-family: Courier; 
        display: grid;
        padding: 3px;
        /* border: 1px solid black; */
        margin: 5px;
        grid-column-gap: 5px;

        grid-template-columns: minmax(40px,max-content) auto;
        grid-template_areas: "l c";
        "#;

    let handle_composition_start = move |event: CompositionEvent| {
        let start_data = event.data().data();
        tracing::info!("IME Start: {:?}", start_data);
        ime_buffer.set(start_data);
    };

    let handle_composition_update = {
        to_owned![editor_state];
        move |event: CompositionEvent| {
            editor_state.with_mut(|e| {
                e.insert_text(&event.data().data());
            });
        }
    };

    let handle_composition_end = move |event: CompositionEvent| {
        tracing::info!("IME End: {:?}", event.data);
        let final_text = event.data().data();
        editor_state.with_mut(|e| e.insert_text(&final_text));
        ime_buffer.set(String::new());
    };

    let handle_clicks = move |event: Event<MouseData>| {
        // Use `use_eval` to create a runner for JavaScript execution
        let mut eval = document::eval(
            r#"
            let ran = document.caretRangeFromPoint({x},{y});
            let el = ran.startContainer;
            let par = el.parentElement;
            return [
                parseInt(par.getAttribute('line')) || -1,
                [...par.childNodes].indexOf(el) + 1,
            ];
            "#,
        );

        let coords = event.page_coordinates();
        let x = coords.x;
        let y = coords.y;

        eval.send(format!("{} {}", x, y).to_string()).unwrap();

        spawn(async move {
            if let Ok(res) = eval.recv::<Value>().await {
                let line = res.get(0).unwrap().as_i64().unwrap();
                let cursor = res.get(1).unwrap().as_i64().unwrap();

                if line < 0 {
                    return;
                }

                editor_state.with_mut(|e| e.set_cursor(line as usize, cursor as usize));

                println!("{line}x{cursor}");
            }
        });
    };

    let handle_global_keys = move |event: Event<KeyboardData>| {
        // switch themes with Cmd + K
        if event.modifiers().contains(Modifiers::META | Modifiers::ALT)
            && event.code() == Code::KeyK
        {
            let new_theme = match *theme.read() {
                Theme::Default => Theme::Readonly,
                Theme::Readonly => Theme::Dev,
                Theme::Dev => Theme::Default,
            };
            theme.set(new_theme);
            event.stop_propagation();
            return;
        }
        // toggle IME
        if event.modifiers().contains(Modifiers::META) && event.code() == Code::KeyK {
            let toggle = !*is_ime.read();
            is_ime.set(toggle);
            event.stop_propagation();
            return;
        }

        // TODO
        // if event.modifiers().contains(Modifiers::SHIFT) && event.code() == Code::Enter {
        //     editor_state.with_mut(|e| e.insert_element());
        //     // event.stop_propagation();
        //     return;
        // }

        // TODO : maybe del
        if event.modifiers().contains(Modifiers::META) && event.code() == Code::KeyA {
            editor_state.with_mut(|e| e.insert_pill("C-A"));
            event.stop_propagation();
            return;
        }

        if event.modifiers().contains(Modifiers::SHIFT) && event.code() == Code::ArrowLeft {
            // editor_state
            //     .with_mut(|e| e.move_cursor_selection(Direction::Forward, Direction::Forward)); // カーソルを右に移動し選択範囲を拡張
            editor_state
                .with_mut(|e| e.move_cursor_selection(Direction::Backward, Direction::Backward)); // カーソルを左に移動し選択範囲をリセット

            // let toggle = !*is_ime.read();
            // is_ime.set(toggle);
            tracing::info!(
                "Left selection_start: {:?}, selection_end: {:?}",
                editor_state.read().selection_start,
                editor_state.read().selection_end,
            );
            event.stop_propagation();
            return;
        }
        if event.modifiers().contains(Modifiers::SHIFT) && event.code() == Code::ArrowRight {
            editor_state
                .with_mut(|e| e.move_cursor_selection(Direction::Forward, Direction::Forward)); // カーソルを右に移動し選択範囲を拡張
                                                                                                // editor_state
                                                                                                //     .with_mut(|e| e.move_cursor_selection(Direction::Backward, Direction::Backward)); // カーソルを左に移動し選択範囲をリセット

            // let toggle = !*is_ime.read();
            // is_ime.set(toggle);
            tracing::info!(
                "Right selection_start: {:?}, selection_end: {:?}",
                editor_state.read().selection_start,
                editor_state.read().selection_end,
            );
            event.stop_propagation();
            return;
        }

        // info!(
        //     "editor_state.selection_start: {:?}, editor_state.selection_end: {:?}",
        //     editor_state.read().selection_start,
        //     editor_state.read().selection_end,
        // );

        // IME mode
        if *is_ime.read() {
            if *is_ime.read() {
                let kana_map: std::collections::HashMap<(Option<Code>, Code), &str> = {
                    use Code::*;
                    [
                        ((Some(KeyK), KeyA), "か"),
                        ((Some(KeyK), KeyI), "き"),
                        ((Some(KeyK), KeyU), "く"),
                        ((Some(KeyK), KeyE), "け"),
                        ((Some(KeyK), KeyO), "こ"),
                        ((Some(KeyS), KeyA), "さ"),
                        ((Some(KeyS), KeyI), "し"),
                        ((Some(KeyS), KeyU), "す"),
                        ((Some(KeyS), KeyE), "せ"),
                        ((Some(KeyS), KeyO), "そ"),
                        ((Some(KeyT), KeyA), "た"),
                        ((Some(KeyT), KeyI), "ち"),
                        ((Some(KeyT), KeyU), "つ"),
                        ((Some(KeyT), KeyE), "て"),
                        ((Some(KeyT), KeyO), "と"),
                        ((Some(KeyN), KeyA), "な"),
                        ((Some(KeyN), KeyI), "に"),
                        ((Some(KeyN), KeyU), "ぬ"),
                        ((Some(KeyN), KeyE), "ね"),
                        ((Some(KeyN), KeyO), "の"),
                        ((Some(KeyH), KeyA), "は"),
                        ((Some(KeyH), KeyI), "ひ"),
                        ((Some(KeyH), KeyU), "ふ"),
                        ((Some(KeyH), KeyE), "へ"),
                        ((Some(KeyH), KeyO), "ほ"),
                        ((Some(KeyM), KeyA), "ま"),
                        ((Some(KeyM), KeyI), "み"),
                        ((Some(KeyM), KeyU), "む"),
                        ((Some(KeyM), KeyE), "め"),
                        ((Some(KeyM), KeyO), "も"),
                        ((Some(KeyY), KeyA), "や"),
                        ((Some(KeyY), KeyU), "ゆ"),
                        ((Some(KeyY), KeyO), "よ"),
                        ((Some(KeyR), KeyA), "ら"),
                        ((Some(KeyR), KeyI), "り"),
                        ((Some(KeyR), KeyU), "る"),
                        ((Some(KeyR), KeyE), "れ"),
                        ((Some(KeyR), KeyO), "ろ"),
                        ((Some(KeyW), KeyA), "わ"),
                        ((Some(KeyW), KeyO), "を"),
                        ((Some(KeyN), KeyN), "ん"),
                        ((None, KeyA), "あ"),
                        ((None, KeyI), "い"),
                        ((None, KeyU), "う"),
                        ((None, KeyE), "え"),
                        ((None, KeyO), "お"),
                        ((Some(KeyD), KeyO), "ど"),
                    ]
                    .into_iter()
                    .collect()
                };

                let current_code = event.code();

                let previous_key = *last_key.read();
                if let Some(previous_key) = previous_key {
                    // other than 「母音」
                    if let Some(&kana) = kana_map.get(&(Some(previous_key), current_code)) {
                        editor_state.with_mut(|e| e.insert_text(kana));
                        last_key.set(None); // reset
                        return;
                    }
                }

                // 「母音」(if `last_key` is `None`)
                if let Some(&kana) = kana_map.get(&(None, current_code)) {
                    editor_state.with_mut(|e| e.insert_text(kana));
                    last_key.set(None); // if 「母音」 reset either
                } else {
                    // 「子音」Update `last_key` as waiting for next key
                    last_key.set(Some(current_code));
                }
            }
            code_events![
                event, editor_state as e,

                code => [
                        for Code::F1 => e.insert_pill("F1"),
                        for Code::F2 => e.insert_pill("F2"),
                        for Code::F3 => e.insert_pill("F3"),
                        for Code::F4 => e.insert_pill("F4"),
                        for Code::Delete => e.delete(Direction::Forward),
                        for Code::Backspace => e.delete(Direction::Backward),
                        for Code::Space => {
                            e.insert_char(char::from_u32(0x00A0).unwrap());
                            let eval = document::eval("window.event.preventDefault();");
                            eval.send(serde_json::Value::Null).unwrap();
                        },
                        for Code::ArrowUp => e.go_to_line(Direction::Backward),
                        for Code::ArrowDown => e.go_to_line(Direction::Forward),
                        for Code::ArrowRight => e.move_cursor(Direction::Forward),
                        for Code::ArrowLeft => e.move_cursor(Direction::Backward),
                        for Code::Enter => e.next_line_or_new()
                ]
            ];
        } else {
            // unused IME
            code_events![
                event, editor_state as e,

                code => [
                        for Code::F1 => e.insert_pill("F1"),
                        for Code::F2 => e.insert_pill("F2"),
                        for Code::F3 => e.insert_pill("F3"),
                        for Code::F4 => e.insert_pill("F4"),
                        for Code::Delete => e.delete(Direction::Forward),
                        for Code::Backspace => e.delete(Direction::Backward),
                        for Code::Space => {
                            e.insert_char(char::from_u32(0x00A0).unwrap());
                            let eval = document::eval("window.event.preventDefault();");
                            eval.send(serde_json::Value::Null).unwrap();
                        },
                        for Code::ArrowUp => e.go_to_line(Direction::Backward),
                        for Code::ArrowDown => e.go_to_line(Direction::Forward),
                        for Code::ArrowRight => e.move_cursor(Direction::Forward),
                        for Code::ArrowLeft => e.move_cursor(Direction::Backward),
                        for Code::Enter => e.next_line_or_new()
                ],
                key => [
                    for Key::Character(n) => e.insert(&n)
                    // for  Key::Character(c) => if c == ":" {
                    //     e.insert_element()
                    // }else {
                    //     e.insert(&c)
                    // }
                ]
            ];
        }
    };

    let (current_line, current_position) =
        editor_state.with(|e| (e.current_line, e.cursor_position));

    rsx! {
        div {
            style: "{editor_style}",
            tabindex: 0,
            autofocus: true,
            oncompositionstart: handle_composition_start,
            oncompositionupdate: handle_composition_update,
            oncompositionend: handle_composition_end,
            onkeydown: handle_global_keys,
            {
                editor_state
                    .read()
                    .iter()
                    .map(|(line_number, line)| {
                        let current = current_line == line_number;
                        let background = if current {
                            "background-color: #f6f6f6;"
                        } else {
                            "background-color: white;"
                        };
                        let opacity = if current { "100%" } else { "20%" };
                        let (rendered_line, line_style): (Vec<(String, String)>, String) = {
                            let mut styled_lines: Vec<(String, String)> = Vec::new();
                            let mut combined_style = String::new();
                            let line_content: String = line
                                .as_vec()
                                .iter()
                                .map(|glyph| match glyph {
                                    Glyph::Text(text) => text.clone(),
                                    Glyph::Char(c) => c.to_string(),
                                    Glyph::Cursor => {
                                        if *is_ime.read() {
                                            "❮:IME".to_string()
                                        } else {
                                            "❮".to_string()
                                        }
                                    }
                                    Glyph::HTMLNode(value) => value.clone(),
                                    Glyph::Component(_) => "<Component>".to_string(),
                                })
                                .collect();
                            match line_content.as_str() {
                                line if line.starts_with("#\u{00A0}") => {
                                    let transformed_line = line_content
                                        .replacen("#\u{00A0}", "", 1);
                                    styled_lines
                                        .push((
                                            transformed_line,
                                            "font-size: 36px; font-weight: bold;".to_string(),
                                        ));
                                    combined_style
                                        .push_str(
                                            "font-size: 28px; margin-bottom: 8px; padding-left: 16px; border-bottom: 0.5px solid rgba(0, 0, 0, 0.5);",
                                        );
                                }
                                line if line.starts_with("##\u{00A0}") => {
                                    let transformed_line = line_content
                                        .replacen("##\u{00A0}", "", 2);
                                    styled_lines
                                        .push((
                                            transformed_line,
                                            "font-size: 30px; font-weight: bold;".to_string(),
                                        ));
                                    combined_style
                                        .push_str(
                                            "font-size: 24px; margin-bottom: 4px; padding-left: 8px;",
                                        );
                                }
                                line if line.starts_with("###\u{00A0}") => {
                                    let transformed_line = line_content
                                        .replacen("###\u{00A0}", "", 3);
                                    styled_lines
                                        .push((
                                            transformed_line,
                                            "font-size: 24px; font-weight: bold;".to_string(),
                                        ));
                                    combined_style
                                        .push_str("font-size: 20px; padding-left: 4px;");
                                }
                                line if line.starts_with("####\u{00A0}") => {
                                    let transformed_line = line_content
                                        .replacen("####\u{00A0}", "", 4);
                                    styled_lines
                                        .push((
                                            transformed_line,
                                            "font-size: 16px; font-weight: bold;".to_string(),
                                        ));
                                    combined_style
                                        .push_str("font-size: 16px; padding-left: 2px;");
                                }
                                line if line.starts_with("-\u{00A0}") => {
                                    let transformed_line = line_content
                                        .replacen("-\u{00A0}", "・", 1);
                                    styled_lines
                                        .push((transformed_line, "padding-left: 8px;".to_string()));
                                    combined_style.push_str("padding-left: 8px;");
                                }
                                line if line.starts_with("```\u{00A0}") => {
                                    let transformed_line = line_content
                                        .replacen("```\u{00A0}", "", 1);
                                    /// Example
                                    ///
                                    /// Input
                                    ///     '```rs_(space)'
                                    /// Output:
                                    ///     1: <codeblock />
                                    ///
                                    ///
                                    ///
                                    let html_node = r#"
                                                                                                                                                                                                                                                                                                                                                                                                                        <input style="">
                                                                                                                                                                                                                                                                                                                                                                                                                        </input>"#;
                                    styled_lines.push((html_node.to_string(), "".to_string()));
                                    styled_lines.push((transformed_line, "".to_string()));
                                    combined_style.push_str("");
                                }
                                line if line.contains("WARNING") => {
                                    styled_lines
                                        .push((
                                            line_content,
                                            "color: red; font-weight: bold;".to_string(),
                                        ));
                                    combined_style.push_str("color: red;");
                                }
                                _ => {
                                    styled_lines.push((line_content, "".to_string()));
                                }
                            }
                            (styled_lines, combined_style)
                        };
                        rsx! {
                            div { style: "padding-right: 5px; text-align: right;",
                                span { style: "opacity: {opacity};", "{line_number}" }
                            }
                            div {
                                style: "{line_style} {background}",
                                id: "L{line_number}",
                                "line": "{line_number}",
                                onmousedown: handle_clicks,
                            
                                // view convert TEXT
                                {
                                    rendered_line
                                        .iter()
                                        .map(|(text, style)| {
                                            rsx! {
                                                // TODO: fix
                                                if text == "<Component>" {
                                                    // Sampleコンポーネントを直接描画
                                                    Sample {}
                                                } else if text.contains("<") {
                                                    // HTMLノードを描画
                                                    span { dangerous_inner_html: "{text}" }
                                                } else if text.contains(":\u{00A0}") {
                                                    // 実装方針
                                                    // ':' + '\u{00A0}'でAutocomplete表示
                                                    // Autocomplete表示中はこれにフォーカスを優先
                                                    // -> カーソルは常に1つを維持
                                                    // 選択内容をクリック または enterで任意のコンポーネントを挿入
                                                    // この部分で行う(できれば) -> insert_elementは不要になる想定
                                                    // 2024/12/8 ん、やっぱりinsert_element使ったほうが綺麗な気がしてきた
                                                    span { style: "{style}", "{text}" }
                                                    Sample {}
                                                } else {
                                                    // 通常のテキストを描画
                                                    // Sample {}
                                                    span { style: "{style}", "{text}" }
                                                }
                                            }
                                        })
                                }
                            }
                        }
                    })
            }
        }
        div { "Line: {current_line} Position: {current_position}" }
    }
}

fn Sample() -> Element {
    let mut count = use_signal(|| 0);

    rsx! {
        h1 { "High-Five counter: {count}" }
        button { onclick: move |_| count += 1, "Up high!" }
        button { onclick: move |_| count -= 1, "Down low!" }
    }
}
