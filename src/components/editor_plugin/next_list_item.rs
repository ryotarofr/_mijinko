/// リスト行を改行時に連続してリストを表示する関数
pub fn nextListItem(line_content: &str) -> Option<String> {
    // 数字付きリスト判定: "1.\u{00A0}" のようなパターンがあれば次の番号を返す
    if let Some(idx) = line_content.find(".\u{00A0}") {
        // idx以前が数字なら次の数値を生成
        let number_str = &line_content[..idx];
        if let Ok(num) = number_str.parse::<usize>() {
            return Some(format!("{}.\u{00A0}", num + 1));
        }
    }

    // 箇条書きリスト判定: "-\u{00A0}" を含んでいれば同じ記号を返す
    if line_content.contains("-\u{00A0}") {
        return Some("-\u{00A0}".to_string());
    }

    // どちらにも該当しなければNone
    None
}
