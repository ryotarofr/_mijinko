use dioxus::prelude::*;
use keyboard_types::{Code, Key, Modifiers};
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;

use crate::components::editor_plugin::{
    command_fn::apply_cd_command, cursor_view::cursorView, next_list_item::nextListItem,
};
use crate::config::constants::LOREM_IPSUM;
use crate::context::theme_context::Theme;
use crate::r#fn::editor_state::EditorState;
use crate::types::enums::{Direction, Glyph};

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

#[component]
pub fn HomeEditor() -> Element {
    // navigater
    let navigator = use_navigator();

    let mut editor_state = use_signal(|| EditorState::from(LOREM_IPSUM));
    tracing::info!("editor_state :{:?}", editor_state.read());
    let mut theme = use_context::<Signal<Theme>>();
    let mut is_ime = use_signal(|| false);
    // let mut last_keys_vec: Signal<Vec<Code>> = use_signal(|| Vec::new());

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

    // # ディレクトリ情報管理用変数
    //
    // DBからディレクトリ取得データ取得
    //
    // TODO: 変数名 allocate_dir_map に変更
    // TODO: use_resource使う。初期表示 or データ変更時のみ取得(deps情報)
    //   - ディレクトリ情報全てを取得
    // # データ取得タイミング
    //   - 初期表示
    //   - mkdir成功時
    //   - touch成功時
    //
    // # 構造
    //   [{memo_id: "file_name"}, {memo_id: [{memo_id: ""file_name""}, {memo_id: [...]}]}]
    //
    //
    // ファイル名に拡張子いらないかな
    //
    //
    // # 以下はシュミレーション用のサンプルデータ-----------------------------------------
    // memo_idのHashMap
    // dbからはこの形式で取り出す
    let db_memo_data: HashMap<i32, String> = HashMap::from([
        (1, "memo1".to_string()),
        (2, "memo2".to_string()),
        (3, "memo3".to_string()),
        (4, "memo4".to_string()),
    ]);

    // メモの中身はredisからmemo_idをキーにjsonで取り出す。
    #[derive(Serialize, Deserialize, Debug)]
    struct Line {
        text: Option<String>,
        cursor: Option<bool>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    struct Memo {
        oya_memo_id: Option<i32>,
        memo_id: i32,
        lines: Vec<Line>,
    }

    // 以下が現在のEditorStateの構造
    // EditorState { contents: EditorData { lines: [LINE<Text("h")Text("e")Text("l")Text("l")Text("l")Text("o")>, LINE<Cursor>, LINE<>] }, current_line: 2, cursor_position: 1 }
    // ここから{ lines : [...]}を抜き出す -> とりあえずこの部分のみをjsonで保存する
    // 保存はmemo_id(PK)毎で管理
    // dioxus_editor_dataに抽出したデータを定義
    //
    // crdtと組み合わせる方法
    // 安価でdiamond_typesのデータ構造をMemoの構造に展開できるなら問題ない(逆も然り)
    //
    //
    let dioxus_editor_data = Memo {
        oya_memo_id: None, // Noneの場合はこのファイルはトップレベルになる
        memo_id: 1,
        lines: vec![
            Line {
                text: Some("hello".to_string()), // テキストが含まれる行
                cursor: None,
            },
            Line {
                text: None, // カーソルがある行
                cursor: Some(true),
            },
            Line {
                text: None, // 空行
                cursor: None,
            },
        ],
    };

    // JSONにシリアライズ
    // 以下の様になる
    // {
    //     "lines": [
    //       {
    //         "text": "hello",
    //         "cursor": null
    //       },
    //       {
    //         "text": null,
    //         "cursor": true
    //       },
    //       {
    //         "text": null,
    //         "cursor": null
    //       }
    //     ]
    //   }
    // ↓とりあえずDBではなくローカルストレージに保存
    //   ローカルストレージは未ログインユーザで使用
    let dioxus_editor_json_data =
        serde_json::to_string_pretty(&dioxus_editor_data).expect("Failed to serialize to JSON");

    //
    // # データ取得後の流れ
    // MongoDBからデータを持ってくる(とりあえず全部)
    // 取得時のキーは""user_id"
    // TODO(後から追加): "最終更新が新しいもの", "更新頻度が多いもの", "閲覧頻度が高いもの" を優先して取り出す処理にしたい
    //
    //
    //
    // 複数の要素を持つ最上位レベル
    // 全ての階層データを全て取得
    // (これも疑似ツリーをローカルストレージに保存)
    // これは階層情報として別で保存しておく
    #[derive(Debug)]
    enum MemoIdTree {
        File(i32),
        Directory(Vec<HashMap<i32, MemoIdTree>>),
    }

    let sample_allocate_dir_map: Vec<HashMap<i32, MemoIdTree>> = vec![
        {
            let mut map = HashMap::new();
            map.insert(1, MemoIdTree::File(4));
            map
        },
        {
            let mut map = HashMap::new();
            map.insert(
                5,
                MemoIdTree::Directory(vec![
                    {
                        let mut submap = HashMap::new();
                        submap.insert(2, MemoIdTree::File(5));
                        submap
                    },
                    {
                        // 今回はさらに下があるかもしれないので、空の配列を入れている
                        let mut submap = HashMap::new();
                        submap.insert(3, MemoIdTree::Directory(vec![]));
                        submap
                    },
                ]),
            );
            map
        },
    ];

    tracing::info!("sample_allocate_dir_map: {:#?}", sample_allocate_dir_map);

    // これいらなくなる予定
    let sample_current_allocate_dir_map =
        vec![HashMap::from([(1, "memo1")]), HashMap::from([(2, "memo2")])];

    // lsコマンドで使う。HashMapをVecに変更
    // ```rs
    // println!("{:?}", converted); // ["memo1", "memo2"]
    // ```
    let converted: Vec<String> = sample_current_allocate_dir_map
        .iter()
        .flat_map(|map| map.values())
        .map(|v| v.to_string())
        .collect();

    // --------------------------------------------------------------------------------------------------

    // pwd情報保持変数
    // グローバル管理した方が良い？
    let mut pwd_info = use_signal(|| "".to_string());

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

    // TODO
    fn is_numbered_list(line: &str) -> bool {
        let number_list_re = Regex::new(r"^\d+\.\u{00A0}").unwrap();
        number_list_re.is_match(line)
    }

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

        // IME mode
        if *is_ime.read() {
            // IME使わない想定
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
                        for Code::Enter => {
                            // 改行前に現在行を確認
                            let current_line_idx = e.current_line;
                            let current_line_content = e.get_line_content(current_line_idx);

                            // # コマンド関連
                            //    - ls
                            //    - pwd
                            //    - cd
                            //    - vim
                            //    - mkdir
                            //    - touch
                            if current_line_content == "ls❮" {
                              e.insert_ls(&converted);
                            }
                            if current_line_content == "pwd❮" {
                              todo!()
                            }
                            if current_line_content.starts_with("cd\u{00A0}") {
                              if let Some(stripped) = current_line_content.strip_prefix("cd\u{00A0}") {
                                  if let Some(idx) = stripped.find('❮') {
                                      // ❮より前の部分を抜き出し (例: ".", "..", "../..", "some_dir", "dir/subdir", etc.)
                                      let extracted = &stripped[..idx];

                                      // 現在のパスを取得
                                      let current = pwd_info.read().to_string();

                                      // 新しいパスを計算
                                      let new_path = apply_cd_command(&current, extracted);

                                      // pwd_info を更新
                                      pwd_info.set(new_path);
                                  }
                              }
                            }
                            if current_line_content.starts_with("vim\u{00A0}") {
                                if let Some(stripped) = current_line_content.strip_prefix("vim\u{00A0}") {
                                    if let Some(idx) = stripped.find('❮') {
                                        // 'vim\u{00A0}'から'❮' シンボルまでの文字抽出
                                        let extracted = &stripped[..idx];

                                        if let Some(found_key) = sample_current_allocate_dir_map.iter().find_map(|hm| {
                                            hm.iter()
                                                .find_map(|(k, v)| if *v == extracted { Some(k) } else { None })
                                        }) {
                                            navigator.push(format!("memo/{}", found_key));
                                        } else {
                                            e.insert_text_not_match(extracted ," is no match found");
                                        }
                                    }
                                }
                            }
                            if current_line_content == "mkdir\u{00A0}❮" {
                              todo!()
                            }
                            if current_line_content == "touch\u{00A0}❮" {
                              todo!()
                            }

                            // 改行処理
                            e.next_line_or_new();

                            // 改行時に前の行がリストなら、次の行もリストを挿入
                            if let Some(next_item) = nextListItem(&current_line_content) {
                                e.insert_text(&next_item);
                            }
                        },
                        for Code::Tab => {
                            // Tabキー押下時の処理を改修
                            let current_line_idx = e.current_line;
                            let current_line_content = e.get_line_content(current_line_idx);

                            // 箇条書き行または数字付きリスト行判定
                            let is_bullet_list = current_line_content.starts_with("-\u{00A0}");
                            let is_num_list = is_numbered_list(&current_line_content);

                            if is_bullet_list || is_num_list {
                                // リスト行の場合はインデントを深くする
                                // ここではpositionを+2するイメージで、行頭へ2つのノーブレークスペースを挿入
                                // まずカーソルを行頭へ移動してから挿入する必要がある場合は、
                                // それに対応したEditorStateの機能を使います。
                                // ここでは簡易的にカーソルを前へ戻したり、
                                // もしくは行頭まで戻してからスペースを挿入するなどの処理が必要となるかもしれません。
                                // EditorStateのAPIに応じて調整してください。

                                // 例: カーソルを先頭へ移動する場合(仮)
                                // e.set_cursor(current_line_idx, 0);

                                e.insert_char(char::from_u32(0x00A0).unwrap());
                                e.insert_char(char::from_u32(0x00A0).unwrap());
                            } else {
                                // 通常の行の場合は既存処理（4つのノーブレークスペース挿入）
                                for _ in 0..4 {
                                    e.insert_char(char::from_u32(0x00A0).unwrap());
                                }
                            }
                            let eval = document::eval("window.event.preventDefault();");
                    eval.send(serde_json::Value::Null).unwrap();
                        }
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
                            let line_content = cursorView(line, *is_ime.read());
                            let (line_text, combined_style) = markdown_view(&line_content);
                            (line_text, combined_style)
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
        // ------------デバッグ用-------------------
        div { "pwd_info : {pwd_info}" }
        // ---------------------------------------
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

// TODO : home_editorでマークダウン使わないのでいらない
// markdown_viewを使っている部分を修正が必要なのでとりあえず残す
fn markdown_view(line_content: &str) -> (Vec<(String, String)>, String) {
    let mut styled_lines = Vec::new();
    let mut combined_style = String::new();

    // ヘッダ行など共通処理をまとめるためのヘルパー
    fn push_styled_line(
        styled_lines: &mut Vec<(String, String)>,
        combined_style: &mut String,
        transformed_line: String,
        text_style: &str,
        container_style: &str,
    ) {
        styled_lines.push((transformed_line, text_style.to_string()));
        combined_style.push_str(container_style);
    }

    // インデント（先頭スペース）をカウント
    let mut spaces = 0;
    for ch in line_content.chars() {
        if ch == ' ' {
            spaces += 1;
        } else {
            break;
        }
    }
    // インデントレベル(4スペース毎に一段下げる例)
    let indent_level = spaces / 4;

    // 行の先頭スペースを除いた文字列（リスト記号検出用）
    let trimmed = &line_content[spaces..];

    // 数字付きリスト用正規表現
    let number_list_re = Regex::new(r"^\d+\.\u{00A0}").unwrap();
    let is_numbered_list = number_list_re.is_match(trimmed);

    // インデントに応じたスタイル
    let indent_px = 8 * (indent_level + 1);
    let list_text_style = format!("padding-left: {}px;", indent_px);
    let list_container_style = list_text_style.clone();

    match line_content {
        // h1
        line if line.starts_with("#\u{00A0}") => {
            let transformed_line = line_content.replacen("#\u{00A0}", "", 1);
            push_styled_line(
                &mut styled_lines,
                &mut combined_style,
                transformed_line,
                "font-size: 36px; font-weight: bold;",
                "font-size: 28px; margin-bottom: 8px; padding-left: 16px; border-bottom: 0.5px solid rgba(0, 0, 0, 0.5);"
            );
        }
        // h2
        line if line.starts_with("##\u{00A0}") => {
            let transformed_line = line_content.replacen("##\u{00A0}", "", 2);
            push_styled_line(
                &mut styled_lines,
                &mut combined_style,
                transformed_line,
                "font-size: 30px; font-weight: bold;",
                "font-size: 24px; margin-bottom: 4px; padding-left: 8px;",
            );
        }
        // h3
        line if line.starts_with("###\u{00A0}") => {
            let transformed_line = line_content.replacen("###\u{00A0}", "", 3);
            push_styled_line(
                &mut styled_lines,
                &mut combined_style,
                transformed_line,
                "font-size: 24px; font-weight: bold;",
                "font-size: 20px; padding-left: 4px;",
            );
        }
        // h4
        line if line.starts_with("####\u{00A0}") => {
            let transformed_line = line_content.replacen("####\u{00A0}", "", 4);
            push_styled_line(
                &mut styled_lines,
                &mut combined_style,
                transformed_line,
                "font-size: 16px; font-weight: bold;",
                "font-size: 16px; padding-left: 2px;",
            );
        }
        // buble list
        line if line.starts_with("-\u{00A0}") => {
            let transformed_line = line_content.replacen("-\u{00A0}", "・", 1);
            push_styled_line(
                &mut styled_lines,
                &mut combined_style,
                transformed_line,
                "padding-left: 8px;",
                "padding-left: 8px;",
            );
        }
        // number list
        _ if is_numbered_list => {
            // 数字リストはそのまま表示(必要なら書き換え可)
            let transformed_line = line_content.to_string();
            push_styled_line(
                &mut styled_lines,
                &mut combined_style,
                transformed_line,
                &list_text_style,
                &list_container_style,
            )
        }
        // warn
        line if line.contains("WARNING") => {
            push_styled_line(
                &mut styled_lines,
                &mut combined_style,
                line_content.to_string(),
                "color: red; font-weight: bold;",
                "color: red;",
            );
        }
        _ => {
            // 特別な書式なしの場合
            styled_lines.push((line_content.to_string(), "".to_string()));
        }
    }

    (styled_lines, combined_style)
}
