/// 構造について
/// 入力情報の接頭辞から html を追加するだけの実装
///
/// ブロックインクリメントにする

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
    /// 今は input に \n があれば true になる想定だが、もっと複雑な条件になった時に直接値指定できるようにするためのもの
    lookahead: bool,
    // lookahead が true なら処理を実行する関数型
    // child: fn(Looker) -> Looker,
    pub child: Option<Box<State>>,
}

impl State {
    pub fn parse(input: &str) -> Self {
        let mut lines = input.split('\n');
        // ── 1行目
        let first = lines.next().unwrap_or("");
        let mut root = State::new_node(first, 0);

        // ── 2行目以降をループで child にリンク
        let mut cursor = &mut root;
        let mut offset = first.len() + 1; // 次行開始オフセット
        for line in lines {
            let mut node: State = State::new_node(line, offset);
            offset += line.len() + 1;

            cursor.child = Some(Box::new(node));
            // 直下の child を次の cursor に
            cursor = cursor.child.as_mut().unwrap();
        }

        root
    }

    fn new_node(text: &str, offset: usize) -> Self {
        State {
            input: text.to_string(),
            line_type: LineType::detect(text),
            offset,
            pos: 0,
            lookahead: text.contains('\n'),
            // parse 側で次行の有無を見てください
            child: None,
        }
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
