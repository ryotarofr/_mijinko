/// 参考
/// https://github.com/biomejs/biome/blob/main/crates/biome_markdown_parser/src/syntax/thematic_break_block.rs
/// 構造について
/// 入力情報の接頭辞から html を追加するだけの実装
///
/// ブロックインクリメントにする

/// \n で区切る関数
/// これは複数行のコピペや削除の時に使う
pub fn split_lines(input: &str) -> Vec<&str> {
    input.split('\n').collect()
}

#[derive(Debug, Clone)]
pub enum LineType {
    /// 空の行などで使う
    Cursor = 0,
    Paragraph = 1,
    Hedding = 2,
    Code = 3,
    // add todo ...
}

impl LineType {
    /// 文字列先頭で判定
    pub fn detect(s: &str) -> Self {
        if s.is_empty() {
            LineType::Cursor
        } else if s.starts_with("```") {
            LineType::Code
        } else if s.starts_with('#') {
            LineType::Hedding
        } else {
            LineType::Paragraph
        }
    }
}

#[derive(Debug, Clone)]
pub struct State<'text> {
    /// 入力テキスト
    pub input: &'text str,
    /// 行の形式判定用 default: 0, 特定され次第即座に挿入される値
    pub line_type: LineType,
    /// 行番号。
    /// 行の削除や追加の際に更新対象がどこかを知るための値。
    /// インサート個所を line_type で連続した挿入をする場合に、先頭空白をインサートするために使う。
    pub offset: usize,
    /// 現在の列
    pub pos: usize,
}

impl<'text> From<&'text str> for State<'text> {
    fn from(text: &'text str) -> Self {
        Self {
            input: text,
            line_type: LineType::detect(text),
            offset: text.len(),
            pos: 0,
        }
    }
}

/// 確実に行単位でわたってくる想定。
/// 特定のルール(リストやコードブロック)では、ルールを受け継ぐように工夫する。
impl<'text> State<'text> {}
