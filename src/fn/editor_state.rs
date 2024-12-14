use dioxus::prelude::*;

use crate::r#fn::editor_data::EditorData;
use crate::r#fn::line::Line;
use crate::types::enums::{Direction, Glyph};

#[derive(Debug)]
pub struct EditorState {
    pub contents: EditorData,
    pub current_line: usize,
    pub cursor_position: usize,
    pub selection_start: usize,
    pub selection_end: usize,
}

impl From<&'_ str> for EditorState {
    fn from(value: &'_ str) -> EditorState {
        let data: EditorData = value.into();
        EditorState {
            cursor_position: 1,
            current_line: 1,
            selection_start: 0, // 選択範囲の初期位置（カーソルと同じ）
            selection_end: 0,   // 選択範囲の終了位置（カーソルと同じ）
            contents: data,
        }
    }
}

macro_rules! with_cursor {
    (|$self:ident| $e:tt) => {
        let cursor_position = ($self.current_line, $self.cursor_position);
        $self.clear_cursor(&cursor_position);
        $e;
        $self.insert_cursor()
    };
}
impl EditorState {
    pub fn next_line_or_new(&mut self) {
        self.contents.add_empty_line(self.current_line);

        with_cursor!(|self| {
            self.current_line += 1;
            self.cursor_position = 1;
        });
    }

    pub fn clear_cursor(&mut self, (line, pos): &(usize, usize)) {
        self.contents.lines[line - 1].as_vec_mut().remove(pos - 1);
    }
    pub fn insert_cursor(&mut self) {
        self.contents.lines[self.current_line - 1]
            .as_vec_mut()
            .insert(self.cursor_position - 1, Glyph::Cursor)
    }
    pub fn insert_char(&mut self, ch: char) {
        println!("Inserting char!");
        let position = self.cursor_position;
        self.cursor_position += 1;
        self.contents
            .insert(self.current_line, position, Glyph::Char(ch));
    }
    pub fn insert_text(&mut self, text: &str) {
        println!("Inserting text!");
        let position = self.cursor_position;
        self.cursor_position += 1;
        self.contents
            .insert(self.current_line, position, Glyph::Text(text.to_string()));
    }

    pub fn insert(&mut self, string: &str) {
        for ch in string.chars() {
            self.insert_text(&ch.to_string())
        }
    }
    pub fn current_line_length(&self) -> usize {
        self.contents.lines[self.current_line - 1].as_vec().len()
    }
    pub fn go_to_line(&mut self, rel: Direction) {
        let mut current_line = self.current_line;
        current_line = (current_line as isize + rel as isize) as usize;

        if current_line < 1 || current_line > self.contents.len() {
            return;
        }

        with_cursor!(|self| {
            self.current_line = current_line;
            self.set_cursor_start_of_line();
        });
    }

    pub fn set_cursor_end_of_line(&mut self) {
        self.cursor_position = self.current_line().as_vec().len() + 1;
    }

    pub fn set_cursor_start_of_line(&mut self) {
        self.cursor_position = 1;
    }

    pub fn move_cursor(&mut self, rel: Direction) {
        if let Direction::Backward = rel {
            let new_position = self.cursor_position - 1;
            if new_position < 1 && self.current_line < 2 {
                return;
            }

            with_cursor!(|self| {
                if new_position >= 1 {
                    self.cursor_position = new_position
                }
                if new_position < 1 {
                    self.current_line -= 1;
                    self.set_cursor_end_of_line();
                }
            });
        }

        if let Direction::Forward = rel {
            let new_position = self.cursor_position + 1;
            let end_of_line = self.cursor_position == self.current_line().as_vec().len();
            let last_line = self.current_line == self.contents.lines.len();
            if end_of_line && last_line {
                return;
            }
            with_cursor!(|self| {
                if end_of_line {
                    self.cursor_position = 1;
                    self.current_line += 1;
                }
                if !end_of_line {
                    self.cursor_position = new_position;
                }
            });
        }
    }

    pub fn move_cursor_selection(&mut self, rel: Direction, rel2: Direction) {
        let extend_selection = matches!(rel2, Direction::Forward); // シフトキーが押された状態を仮定

        if let Direction::Backward = rel {
            let new_position = self.cursor_position.saturating_sub(1); // 範囲外アクセスを防ぐ
            if new_position < 1 && self.current_line < 2 {
                return;
            }

            with_cursor!(|self| {
                if new_position >= 1 {
                    self.cursor_position = new_position;
                    if extend_selection {
                        self.selection_end = self.cursor_position;
                    } else {
                        self.selection_start = self.cursor_position;
                        self.selection_end = self.cursor_position;
                    }
                }
                if new_position < 1 {
                    self.current_line = self.current_line.saturating_sub(1);
                    self.set_cursor_end_of_line();
                    if extend_selection {
                        self.selection_end = self.cursor_position;
                    } else {
                        self.selection_start = self.cursor_position;
                        self.selection_end = self.cursor_position;
                    }
                }
            });
        }

        if let Direction::Forward = rel {
            let new_position = self.cursor_position + 1;
            let end_of_line = self.cursor_position == self.current_line().as_vec().len();
            let last_line = self.current_line == self.contents.lines.len();
            if end_of_line && last_line {
                return;
            }
            with_cursor!(|self| {
                if end_of_line {
                    self.current_line += 1;
                    self.cursor_position = 1;
                    if extend_selection {
                        self.selection_end = self.cursor_position;
                    } else {
                        self.selection_start = self.cursor_position;
                        self.selection_end = self.cursor_position;
                    }
                }
                if !end_of_line {
                    self.cursor_position = new_position;
                    if extend_selection {
                        self.selection_end = self.cursor_position;
                    } else {
                        self.selection_start = self.cursor_position;
                        self.selection_end = self.cursor_position;
                    }
                }
            });
        }
    }

    pub fn set_cursor(&mut self, line: usize, cursor: usize) {
        with_cursor!(|self| {
            self.current_line = line;
            self.cursor_position = cursor;
        });
    }

    #[allow(unreachable_code)]
    pub fn delete(&mut self, dir: Direction) {
        match dir {
            Direction::Forward => {
                todo!(); // TODO
                if self.cursor_position == (self.current_line_length() + 1) {
                    return;
                }
                self.current_line()
                    .as_vec_mut()
                    .remove(self.cursor_position - 1);
            }
            Direction::Backward => match (self.cursor_position, self.current_line) {
                (pos, lin) if pos <= 1 && lin > 1 => self.join_lines(),
                (pos, _) if pos > 1 => self.remove_char(),
                _ => (),
            },
        }
    }
    pub fn join_lines(&mut self) {
        with_cursor!(|self| {
            let mut line = self.contents.lines.remove(self.current_line - 1);
            self.current_line -= 1;
            let position = self.current_line().as_vec().len() + 1;
            self.current_line_mut()
                .as_vec_mut()
                .append(line.as_vec_mut());
            self.cursor_position = position;
        });
    }

    pub fn remove_char(&mut self) {
        self.contents.lines[self.current_line - 1]
            .as_vec_mut()
            .remove(self.cursor_position - 2);
        self.cursor_position -= 1;
    }

    pub fn current_line(&mut self) -> &Line {
        &mut self.contents.lines[self.current_line - 1]
    }

    pub fn current_line_mut(&mut self) -> &mut Line {
        &mut self.contents.lines[self.current_line - 1]
    }

    pub fn iter(&self) -> std::vec::IntoIter<(usize, &Line)> {
        let mut line_number = 0;

        let result: Vec<(usize, &Line)> = self
            .contents
            .lines
            .iter()
            .map(|line| {
                line_number += 1;
                (line_number, line)
            })
            .collect();
        result.into_iter()
    }

    pub fn insert_pill(&mut self, text: &str) {
        let position = self.cursor_position;
        self.cursor_position += 1;
        self.contents.insert(
            self.current_line,
            position,
            Glyph::HTMLNode(format!(r#"
<button style="appearance: none; border: none; background: none; padding: 0; margin: 0; font: inherit; border-radius: 4px; background-color: #7f00ff; color: #fff; text-transform: uppercase; font-weight: bold; letter-spacing: 1px; padding: 6px 12px; font-size: 12px; line-height: 1.5; cursor: pointer; transition: background-color 0.2s ease-in-out;" onmouseover="this.style.backgroundColor='#5f00cc'" onmouseout="this.style.backgroundColor='#7f00ff'">{text}</button>
"#))
        );
    }

    // TODO: 2024/12/7 これ不要(のはず)
    pub fn insert_element(&mut self) {
        let position = self.cursor_position;
        self.cursor_position += 1;

        self.contents.insert(
            self.current_line,
            position,
            // 引数一個追加してここでmatch式入れる
            Glyph::Component(Sample), // Sampleコンポーネントを追加
        );
    }

    // pub fn line_length(&self, line_number: usize) -> usize {
    //     self.contents[line_number].len()
    // }

    // pub fn set_cursor_start_of_line(&mut self) {
    //     self.cursor_position = 0; // 行の先頭に移動
    // }

    // pub fn set_cursor_end_of_line(&mut self) {
    //     if let Some(line) = self.contents.get(self.current_line) {
    //         self.cursor_position = line.len(); // 行の末尾に移動
    //     }
    // }
    // pub fn update_selection(&mut self, direction: Direction, extend: bool) {
    //     if extend {
    //         // シフトキーが押されている場合、選択範囲を拡張
    //         match direction {
    //             Direction::Forward => {
    //                 self.selection_end += 1;
    //             }
    //             Direction::Backward => {
    //                 if self.selection_end > self.selection_start {
    //                     self.selection_end -= 1;
    //                 }
    //             }
    //         }
    //     } else {
    //         // シフトキーが押されていない場合、カーソルだけを移動
    //         self.selection_start = self.cursor_position;
    //         self.selection_end = self.cursor_position;
    //         match direction {
    //             Direction::Forward => self.cursor_position += 1,
    //             Direction::Backward => {
    //                 if self.cursor_position > 0 {
    //                     self.cursor_position -= 1;
    //                 }
    //             }
    //         }
    //     }
    // }

    // pub fn get_line(&self, line_number: usize) -> Option<&str> {
    //     self.contents.lines().nth(line_number)
    // }

    // pub fn get_selected_text(&self) -> String {
    //     if self.selection_start == self.selection_end {
    //         return "".to_string();
    //     }

    //     if let Some(line) = self.contents.get_line(self.current_line) {
    //         let start = self.selection_start.min(self.selection_end);
    //         let end = self.selection_start.max(self.selection_end);
    //         if start < line.len() && end <= line.len() {
    //             return line[start..end].to_string();
    //         }
    //     }

    //     "".to_string()
    // }
}

fn Sample() -> Element {
    let mut count = use_signal(|| 0);

    rsx! {
        h1 { "High-Five counter: {count}" }
        button { onclick: move |_| count += 1, "Up high!" }
        button { onclick: move |_| count -= 1, "Down low!" }
    }
}
