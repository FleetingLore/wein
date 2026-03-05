// src/web_collections/title.rs

use crate::core::line::Line;
use crate::core::lore::Lore;
use crate::web_collections::error::LoreError;
use crate::web_collections::traits::title::Title;

pub struct TitleLine<'a> { pub indent: usize, pub title: Lore<'a> }

impl<'a> TitleLine<'a> {
    pub fn new(indent: usize, title: &'a str) -> Self {
        Self { indent, title: title.into() }
    }
}

impl<'a> Line<'a> for TitleLine<'a> {
    fn get_indent(&self) -> usize {
        self.indent
    }

    fn get_content(&self) -> String {
        format!("+ {}", &self.title)
    }
}

impl<'a> Title<'a> for TitleLine<'a> {
    fn get_title(&self) -> String {
        self.title.tokens.to_string()
    }
}

impl Default for TitleLine<'_> {
    fn default() -> Self { Self::new(0, "default title", ) }
}

impl<'a> TryFrom<&'a str> for TitleLine<'a> {
    type Error = LoreError;

    fn try_from(raw: &'a str) -> Result<Self, Self::Error> {
        let indent_count = raw.chars().take_while(|c| *c == ' ').count();

        if indent_count % 2 != 0 { return Err(LoreError::IndentIsNotEven); }

        let content = raw[indent_count..].trim_start();

        if !content.starts_with("+ ") { return Err(LoreError::InvalidFormat); }

        let title = content[2..].trim_start();
        Ok(TitleLine::new(indent_count / 2, title))
    }
}

impl std::fmt::Display for TitleLine<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}+ {}",
            "  ".repeat(self.indent),
            self.title
        )
    }
}
