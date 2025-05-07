use dioxus::prelude::Navigator;
use regex::Regex;

pub fn markdownView(line_content: &str, navigator: &Navigator) -> (Vec<(String, String)>, String) {
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
        // codeblock
        // line if line.starts_with(":") => {
        //     let transformed_line = line_content.replacen("```\u{00A0}", "", 1);
        //     let html_node = r#"<input style=""></input>"#;
        //     styled_lines.push((html_node.to_string(), "".to_string()));
        //     styled_lines.push((transformed_line, "".to_string()));
        //     // この場合、combined_styleは変更しない
        // }
        // :wq
        // line if line.starts_with(":wq") => {
        //     let transformed_line = line_content.replacen(":wq", "", 2);
        //     push_styled_line(
        //         &mut styled_lines,
        //         &mut combined_style,
        //         transformed_line,
        //         "font-size: 30px; font-weight: bold;",
        //         "font-size: 24px; margin-bottom: 4px; padding-left: 8px;",
        //     );
        //     navigator.push("/");
        // }
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
