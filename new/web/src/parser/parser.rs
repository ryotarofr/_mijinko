use dioxus::prelude::Element as DioxusElement;
use dioxus::prelude::*;
/// 参考
/// https://github.com/biomejs/biome/blob/main/crates/biome_markdown_parser/src/syntax/thematic_break_block.rs
/// 構造について
/// 入力情報の接頭辞から html を追加するだけの実装
use std::collections::HashMap;
use web_sys::window;
use web_sys::Element as WebSysElement;

/// ブロックインクリメントにする
#[derive(Debug, Clone)]
pub enum LineType {
    /// 空の行などで使う
    Cursor = 0,
    Paragraph = 1,
    Hedding = 2,
    Code = 3,
    Quote = 4,
    // add todo ...
}

impl From<&str> for LineType {
    /// 文字列先頭で判定
    /// 一旦 replace はしない
    fn from(s: &str) -> Self {
        if s.is_empty() {
            LineType::Cursor
        } else if s.starts_with("```") {
            LineType::Code
        } else if s.starts_with('#') {
            LineType::Hedding
        } else if s.starts_with('>') {
            LineType::Quote
        } else {
            LineType::Paragraph
        }
    }
}

#[derive(Debug, Clone)]
pub struct LineState {
    /// 入力テキスト
    /// 構造体内部でデータの変更はできない(暗黙の更新を排除したほうがクリーンなコードになる)
    pub input: String,
    /// 行の形式判定用 default: 0, 特定され次第即座に挿入される値
    pub line_type: LineType,
    // 履歴管理用(lexicalでやっているノードに紐づく値)
    // pub history: usize,
}

impl From<String> for LineState {
    fn from(text: String) -> Self {
        let line_type = LineType::from(text.as_str());
        Self {
            input: text,
            line_type,
        }
    }
}

// とりあえず、HashMap で実装する(最終的な目標はコンテンツによって HashMap と BTreeMap を使い分けれるようにすること)
// → たくさんの人が編集する場合や、大量のデータを安定して管理したい時は BTreeMap
// → それ以外は HashMap
// という区別にできたらうれしい
// これを作る場合に、 はじめ、HashMap で設定していたけどやっぱり BTreeMap にしたいなーってなったときに
// 相互変換できるような実装が必要。

/// 世代管理用の構造体
/// ローカル履歴のみを管理
/// 初期表示で過去のデータがあったとしても 0 で初期化される
#[derive(Debug)]
pub struct _LocalLineHistory {
    pub generations: HashMap<usize, LineState>,
    pub current: usize,
}

impl _LocalLineHistory {
    pub fn default() -> Self {
        Self {
            generations: HashMap::new(),
            current: 0,
        }
    }
    pub fn insert(&mut self, state: LineState) {
        self.current += 1;
        self.generations.insert(self.current, state);
    }
    pub fn get(&self, gen: usize) -> Option<&LineState> {
        self.generations.get(&gen)
    }
}

pub struct RootNode {
    pub nodes: _LocalLineHistory,
}

/// root で 一括管理している Node
impl RootNode {}

/// Render trait
/// LineTypeの設定
/// Element の動的生成
pub trait Render {
    fn split_lines(value: String) -> Vec<String>;
    fn render_state_rsx(&self) -> DioxusElement;
    fn render_state_jsx() -> WebSysElement;
}

impl Render for LineState {
    fn split_lines(value: String) -> Vec<String> {
        value.split('\n').map(|s| s.to_string()).collect()
    }
    fn render_state_rsx(&self) -> DioxusElement {
        match &self.line_type {
            LineType::Cursor | LineType::Paragraph => rsx! {
                p { {&self.input.as_str()} }
            },
            LineType::Hedding => rsx! {
                h2 { {&self.input.as_str()} }
            },
            LineType::Code => rsx! {
                pre { code { {&self.input.as_str()} } }
            },
            LineType::Quote => rsx! {
                blockquote { {&self.input.as_str()} }
            },
        }
    }
    fn render_state_jsx() -> WebSysElement {
        // ts で使うときにテストする
        let doc = window().unwrap().document().unwrap();
        doc.create_element("p").unwrap()
    }
}
