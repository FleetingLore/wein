// src/web_collections/atom.rs

use crate::core::lore::Lore;
use crate::core::line::Line;
use crate::core::error::LoreError;

pub struct AtomLine<'a> { pub indent: usize, pub content: Lore<'a> }

impl<'a> AtomLine<'a> {
    pub fn new(indent: usize, content: &'a str) -> Self {
        Self { indent, content: Lore::from(content) }
    }
}

impl<'a> Line<'a> for AtomLine<'a> {
    fn get_indent(&self) -> usize { self.indent }

    fn get_content(&self) -> String { format!("{}", &self.content) }
}

impl<'a> From<(usize, &'a str)> for AtomLine<'a> {
    fn from(input: (usize, &'a str)) -> Self {
        Self { indent: input.0, content: input.1.into() }
    }
}

impl Default for AtomLine<'_> {
    fn default() -> Self {
        Self { indent: 0, content: Lore::default() }
    }
}

impl<'a> TryFrom<&'a str> for AtomLine<'a> {
    type Error = LoreError;

    fn try_from(raw: &'a str) -> Result<Self, Self::Error> {
        if raw.is_empty() { return Err(LoreError::InvalidFormat) }

        let indent_count = raw.chars()
            .take_while(|c| *c == ' ')
            .count();

        if indent_count % 2 != 0 { return Err(LoreError::IndentIsNotEven); }

        let content = &raw[indent_count..];
        
        let content = if content.is_empty() {
            return Err(LoreError::InvalidFormat);
        } else {
            Some(content)
        };

        Ok(AtomLine { indent: indent_count / 2, content: Lore::from(content.unwrap()) })
    }
}

impl std::fmt::Display for AtomLine<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f, "{}{}", "  ".repeat(self.indent),
            self.content
        )
    }
}
