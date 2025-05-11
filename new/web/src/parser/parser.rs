use dioxus::prelude::Element as DioxusElement;
use dioxus::prelude::*;
use std::collections::HashMap;
use std::hash::Hash;
use web_sys::window;
use web_sys::Element as WebSysElement;

/// ブロックインクリメントにする
#[derive(Debug, Clone)]
pub enum LineType {
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
            LineType::Paragraph
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
    pub active: bool,
}

impl From<String> for LineState {
    fn from(text: String) -> Self {
        let line_type = LineType::from(text.as_str());
        Self {
            input: text,
            line_type,
            active: true,
        }
    }
}

impl LineState {
    pub fn new() -> Self {
        Self {
            input: String::new(),
            line_type: LineType::Paragraph,
            active: true,
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

/// Render trait
/// LineTypeの設定
/// Element の動的生成
pub trait Render {
    // fn insert() -> LineState;
    fn split_lines(value: String) -> Vec<String>;
    fn render_state_rsx(&self) -> DioxusElement;
    fn render_state_jsx() -> WebSysElement;
}

impl Render for LineState {
    // fn insert() -> LineState {
    //     LineState::new()
    // }
    fn split_lines(value: String) -> Vec<String> {
        value.split('\n').map(|s| s.to_string()).collect()
    }
    fn render_state_rsx(&self) -> DioxusElement {
        match &self.line_type {
            LineType::Paragraph => rsx! {
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

pub struct Editor {
    pub content: LineState,
    pub node: Node,
}

impl Editor {
    pub fn insert(&mut self, key: &usize, node_tree: &NodeTree) -> Self {
        let node = self.node.insert(key, node_tree);
        Editor {
            content: LineState::new(),
            node,
        }
    }
}

/// 現在の位置情報を管理
/// 現状は離散的
pub struct Node {
    pub key: usize,
    pub parent: usize,
    pub prev: Option<usize>,
    pub next: Option<usize>,
}

impl Node {
    fn insert(&mut self, key: &usize, node_tree: &NodeTree) -> Self {
        Node {
            key: node_tree.get_max_key() + 1,
            parent: *key,
            prev: None,
            next: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NodeKey(pub usize);
pub struct LinePos(pub usize);

/// 全部
pub struct NodeTree {
    pub nodes: HashMap<NodeKey, Editor>,
    pub current_key: NodeKey,
    pub current_pos: LinePos,
}

impl NodeTree {
    pub fn get_max_key(&self) -> usize {
        self.nodes.keys().map(|k| k.0).max().unwrap_or(0)
    }
    pub fn set_current_key(&mut self, active_key: usize) {
        self.current_key = NodeKey(active_key);
    }
    pub fn set_current_pos(&mut self, pos: usize) {
        self.current_pos = LinePos(pos);
    }
    // IME 確定状態 or IME じゃないときのすべてで enter 押されたとき
    pub fn insert(&mut self, key: usize) -> &mut Self {
        // key は 現在 active な要素がわたってくる
        let next_key = self.get_max_key() + 1;
        // Node の発行
        let node = Node {
            key: next_key,
            parent: key,
            prev: None,
            next: None,
        };
        // Editor の発行
        let editor = Editor {
            content: LineState::new(),
            node,
        };
        // NodeTree の発行
        self.nodes.insert(NodeKey(next_key), editor);
        self.set_current_key(key);
        self.set_current_pos(0);
        self
    }

    // NodeTree::current の value が 0 のときに backspace が押されたとき
    fn remove(&mut self, key: NodeKey) {
        todo!()
    }

    // アプリケーション側で ref などを使って (ref.current = key など) active なノードを取得し NodeTree 伝番のため毎回発火する
    pub fn go_to_line(self, active_key: &str) {}
}
