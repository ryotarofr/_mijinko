use crate::{r#fn::line::Line, types::enums::Glyph};

pub fn cursorView(line: &Line, is_ime: bool) -> String {
    line.as_vec()
        .iter()
        .map(|glyph| match glyph {
            Glyph::Text(text) => text.clone(),
            Glyph::Char(c) => c.to_string(),
            Glyph::Cursor => {
                if is_ime {
                    "❮:IME".to_string()
                } else {
                    "❮".to_string()
                }
            }
            Glyph::HTMLNode(value) => value.clone(),
            Glyph::Component(_) => "<Component>".to_string(),
        })
        .collect()
}
