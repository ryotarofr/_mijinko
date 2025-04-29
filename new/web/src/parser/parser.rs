/// 構造について
/// 入力情報の接頭辞から html を追加するだけの実装
///
/// ブロックインクリメントにする
#[derive(Debug)]
struct Looker {
    /// true なら先読みが必要
    lookahead: bool,
    // 再帰的に更新をする関数。複数行の削除とか複数行のペーストで使うはず。
    // child: to(Looker) -> Looker,
}

#[derive(Debug)]
pub enum LineType {
    /// 空の行などで使う
    Cursor = 0,
    Paragraph = 1,
    Hedding = 2,
    Code = 3,
    // add todo ...
}

#[derive(Debug)]
pub struct State {
    /// 入力テキスト
    pub input: String,
    /// 行の形式判定用 default: 0, 特定され次第即座に挿入される値
    pub line_type: LineType,
    /// 行番号。
    /// 行の削除や追加の際に更新対象がどこかを知るための値。
    /// インサート個所を line_type で連続した挿入をする場合に、先頭空白をインサートするために使う。
    offset: usize,
    /// 現在の列
    pos: usize,
    /// TODO(後から実装): 先読み関連 , default: false,
    lookahead: Looker,
    // lookahead が true なら処理を実行する関数型
    // child: fn(Looker) -> Looker,
    child: Vec<State>,
}

impl State {
    pub fn new(input: &str) -> Self {
        State {
            input: input.to_string(),
            line_type: State::get_line_type(input),
            offset: 0,
            pos: 0,
            lookahead: Looker { lookahead: false },
            child: Vec::new(),
        }
    }

    fn add_child(&mut self, child: State) {
        self.child.push(child);
    }

    pub fn insert(&mut self, pos: usize, text: &str) {
        self.input.insert_str(pos, text);
        self.offset += text.len();
        // 挿入後にline_typeを再判定したい場合
        self.line_type = State::get_line_type(&self.input);
    }

    /// 指定位置から指定長さを削除
    pub fn delete(&mut self, pos: usize, len: usize) {
        let end = pos + len;
        if end <= self.input.len() {
            self.input.replace_range(pos..end, "");
            self.offset = self.offset.saturating_sub(len);
            // 削除後にline_typeを再判定したい場合
            self.line_type = State::get_line_type(&self.input);
        }
    }

    /// line_type の判定を行う
    fn get_line_type(input: &str) -> LineType {
        match input {
            "" => LineType::Cursor,
            s if s.starts_with("```") => LineType::Code,
            s if s.starts_with("#") => LineType::Hedding,
            _ => LineType::Paragraph,
        }
    }
    fn length(&self) -> usize {
        self.input.len()
    }

    fn get_offset(&mut self) -> usize {
        self.offset += self.length();
        self.offset
    }
}
