use dioxus::prelude::*;
use std::fmt::Display;

pub enum Direction {
    Forward = 1,
    Backward = -1,
}

#[derive(Debug)]
pub enum Glyph {
    Text(String),
    Char(char),
    Cursor,
    HTMLNode(String),
    Component(fn() -> Element), // TODO: delete
}

impl Display for Glyph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Glyph::Text(s) => write!(f, "{}", s),
            Glyph::Char(c) => write!(f, "{}", c),
            Glyph::Cursor => write!(f, "▫️"),
            Glyph::HTMLNode(_s) => write!(f, "HTMLNode"),
            // TODO: delete
            Glyph::Component(_component) => write!(f, "Component"), // Perse Element
        }
    }
}
