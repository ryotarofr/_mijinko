use crate::types::enums::Glyph;

/// Represents sningle line in editor state.
/// Line is a simple vector of `Renderable` structs,
/// But also supports reporting length, inserting and removing elements at positio
pub struct Line(Vec<Glyph>);

impl Line {
    /// Get length of line
    ///
    ///# Examples:
    /// ```
    /// let line = Line(vec![String::from("hello"), " ".into(), String::from("world")]);
    /// assert_eq!(line.len(), 11)
    /// ```
    pub fn as_vec(&self) -> &Vec<Glyph> {
        &self.0
    }

    pub fn as_vec_mut(&mut self) -> &mut Vec<Glyph> {
        &mut self.0
    }

    pub fn new() -> Line {
        Line(vec![])
    }
}

impl From<String> for Line {
    fn from(value: String) -> Self {
        let vec: Vec<Glyph> = value.chars().map(Glyph::Char).collect();

        Line(vec)
    }
}
impl From<&str> for Line {
    fn from(value: &str) -> Self {
        let vec: Vec<Glyph> = value.chars().map(Glyph::Char).collect();

        Line(vec)
    }
}
impl std::fmt::Debug for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LINE<")?;
        for item in &self.0 {
            write!(f, "{:?}", item)?;
        }
        write!(f, ">")
    }
}
impl AsRef<Line> for Line {
    fn as_ref(&self) -> &Line {
        self
    }
}

// line type追加版
// use crate::types::enums::Glyph;

// #[derive(Debug, Clone, PartialEq)]
// pub enum LineType {
//     Normal,
//     Header(usize), // ヘッダーレベルを持てるようにする (#=1, ##=2など)
//     List,
//     CodeBlock,
//     Table,
//     ComponentLine,
// }

// /// Represents sningle line in editor state.
// /// Line is a simple vector of `Renderable` structs,
// /// But also supports reporting length, inserting and removing elements at position
// #[derive(Clone)]
// pub struct Line {
//     glyphs: Vec<Glyph>,
//     line_type: LineType,
// }
// // pub struct Line(Vec<Glyph>);
// impl Line {
//     pub fn new() -> Line {
//         Line {
//             glyphs: Vec::new(),
//             line_type: LineType::Normal,
//         }
//     }

//     pub fn as_vec(&self) -> &Vec<Glyph> {
//         &self.glyphs
//     }

//     pub fn as_vec_mut(&mut self) -> &mut Vec<Glyph> {
//         &mut self.glyphs
//     }

//     pub fn update_line_type(&mut self) {
//         let text: String = self
//             .glyphs
//             .iter()
//             .map(|g| match g {
//                 Glyph::Text(t) => t.clone(),
//                 Glyph::Char(c) => c.to_string(),
//                 Glyph::Cursor => "❮".to_string(),
//                 Glyph::HTMLNode(v) => v.clone(),
//                 Glyph::Component(_) => "<Component>".to_string(),
//             })
//             .collect();

//         // パターンマッチで行種別を決める
//         self.line_type = if text.starts_with("####") {
//             LineType::Header(4)
//         } else if text.starts_with("###") {
//             LineType::Header(3)
//         } else if text.starts_with("##") {
//             LineType::Header(2)
//         } else if text.starts_with("#") {
//             LineType::Header(1)
//         } else if text.starts_with("- ") {
//             LineType::List
//         } else if text.starts_with("```") {
//             // ここを "```" にすることで、コードブロックを判定
//             LineType::CodeBlock
//         } else if text.contains("|") {
//             // 簡易的な判定。実際には表記法を細かく検証。
//             LineType::Table
//         } else if text.contains("<Component>") {
//             LineType::ComponentLine
//         } else {
//             LineType::Normal
//         };
//     }

//     // pub fn update_line_type(&mut self) {
//     //     let text: String = self
//     //         .glyphs
//     //         .iter()
//     //         .map(|g| match g {
//     //             Glyph::Text(t) => t.clone(),
//     //             Glyph::Char(c) => c.to_string(),
//     //             Glyph::Cursor => "❮".to_string(),
//     //             Glyph::HTMLNode(v) => v.clone(),
//     //             Glyph::Component(_) => "<Component>".to_string(),
//     //         })
//     //         .collect();

//     //     // パターンマッチで行種別を決める
//     //     self.line_type = if text.starts_with("####") {
//     //         LineType::Header(4)
//     //     } else if text.starts_with("###") {
//     //         LineType::Header(3)
//     //     } else if text.starts_with("##") {
//     //         LineType::Header(2)
//     //     } else if text.starts_with("#") {
//     //         LineType::Header(1)
//     //     } else if text.starts_with("-") {
//     //         LineType::List
//     //     } else if text.starts_with("```") {
//     //         LineType::CodeBlock
//     //     } else if text.contains("|") {
//     //         // 簡易的な判定。実際には表記法を細かく検証。
//     //         LineType::Table
//     //     } else if text.contains("<Component>") {
//     //         LineType::ComponentLine
//     //     } else {
//     //         LineType::Normal
//     //     };
//     // }
// }

// // impl Line {
// //     /// Get length of line
// //     ///
// //     ///# Examples:
// //     /// ```
// //     /// let line = Line(vec![String::from("hello"), " ".into(), String::from("world")]);
// //     /// assert_eq!(line.len(), 11)
// //     /// ```
// //     pub fn as_vec(&self) -> &Vec<Glyph> {
// //         &self.0
// //     }

// //     pub fn as_vec_mut(&mut self) -> &mut Vec<Glyph> {
// //         &mut self.0
// //     }

// //     pub fn new() -> Line {
// //         Line(vec![])
// //     }
// // }

// impl From<&str> for Line {
//     fn from(value: &str) -> Self {
//         let glyphs: Vec<Glyph> = value.chars().map(Glyph::Char).collect();
//         let mut line = Line {
//             glyphs,
//             line_type: LineType::Normal,
//         };
//         line.update_line_type();
//         line
//     }
// }

// // impl From<String> for Line {
// //     fn from(value: String) -> Self {
// //         let vec: Vec<Glyph> = value.chars().map(Glyph::Char).collect();

// //         Line {
// //             glyphs: vec,
// //             line_type: LineType::Normal,
// //         }
// //     }
// // }
// // impl From<&str> for Line {
// //     fn from(value: &str) -> Self {
// //         let vec: Vec<Glyph> = value.chars().map(Glyph::Char).collect();

// //         Line(vec)
// //     }
// // }
// impl std::fmt::Debug for Line {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "LINE<")?;
//         for item in &self.glyphs {
//             write!(f, "{:?}", item)?;
//         }
//         // line_typeも表示する
//         write!(f, " line_type: {:?}>", self.line_type)
//     }
// }
// // impl std::fmt::Debug for Line {
// //     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
// //         write!(f, "LINE<")?;
// //         for item in &self.glyphs {
// //             write!(f, "{:?}", item)?;
// //         }
// //         write!(f, ">")
// //     }
// // }
// impl AsRef<Line> for Line {
//     fn as_ref(&self) -> &Line {
//         self
//     }
// }
