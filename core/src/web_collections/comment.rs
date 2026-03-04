// src/web_collections/comment.rs

use crate::core::line::Line;
use crate::core::lore::Lore;
use crate::web_collections::error::LoreError;

pub struct Comment<'a> { pub indent: usize, pub docs: Lore<'a>, }

impl<'a> Comment<'a> {
    pub fn new(indent: usize, docs: &'a str) -> Self {
        Self { indent, docs: docs.into() }
    }
}

impl<'a> Line<'a> for Comment<'a> {
    fn get_indent(&self) -> usize { self.indent }

    fn get_content(&self) -> String { format!("# {}", self.docs.tokens) }
}

impl Default for Comment<'_> {
    fn default() -> Self {
        Self::new(0, "default docs" )
    }
}

impl<'a> TryFrom<&'a str> for Comment<'a> {
    type Error = LoreError;

    fn try_from(raw: &'a str) -> Result<Self, Self::Error> {
        if raw.is_empty() { return Err(LoreError::InvalidFormat) }

        let indent_count = raw.chars()
            .take_while(|c| *c == ' ')
            .count();

        if indent_count % 2 != 0 { return Err(LoreError::IndentIsNotEven); }

        let docs = raw[indent_count..].trim_start();

        if !docs.starts_with("# ") {
            return Err(LoreError::InvalidFormat);
        }

        let docs = docs[2..].trim_start();
        Ok(Comment::new(indent_count / 2, docs))
    }
}

impl std::fmt::Display for Comment<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f, "{}/ {}", "  ".repeat(self.indent),
            self.docs.tokens
        )
    }
}
