// src/web_collections/title_link.rs

use crate::core::line::Line;
use crate::core::lore::Lore;
use crate::web_collections::error::LoreError;
use crate::web_collections::traits::title::Title;

pub struct TitleLinkLine<'a> { pub indent: usize, pub title: Lore<'a>, pub url: Lore<'a> }

impl<'a> TitleLinkLine<'a> {
    pub fn new(indent: usize, title: &'a str, url: &'a str) -> Self {
        Self { indent, title: title.into(), url: url.into() }
    }
}

impl<'a> Line<'a> for TitleLinkLine<'a> {
    fn get_indent(&self) -> usize { self.indent }

    fn get_content(&self) -> String { format!("+ {} = {}", &self.title, &self.url) }
}

impl<'a> Title<'a> for TitleLinkLine<'a> {
    fn get_title(&self) -> String { self.title.tokens.to_string() }
}

impl Default for TitleLinkLine<'_> {
    fn default() -> Self { Self::new(0, "default title", "default url") }
}

impl<'a> TryFrom<&'a str> for TitleLinkLine<'a> {
    type Error = LoreError;

    fn try_from(raw: &'a str) -> Result<Self, Self::Error> {
        if raw.is_empty() { return Err(LoreError::InvalidFormat) }

        let indent_count = raw.chars()
            .take_while(|c| *c == ' ')
            .count();

        if indent_count % 2 != 0 { return Err(LoreError::IndentIsNotEven); }

        let content = raw[indent_count..].trim_start();

        if !content.starts_with("+ ") { return Err(LoreError::InvalidFormat) }

        let link = content[2..].trim_start();

        match link.split_once(" = ") {
            Some((info, url)) => {
                let info = info.trim();
                let url = url.trim();

                if info.is_empty() || url.is_empty() {
                    Err(LoreError::InvalidFormat)
                } else {
                    Ok(TitleLinkLine::new(indent_count / 2, info, url))
                }
            }
            None => Err(LoreError::InvalidFormat)
        }
    }
}

impl std::fmt::Display for TitleLinkLine<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f, "{}+ {} = {}",
            "  ".repeat(self.indent),
            self.title,
            self.indent
        )
    }
}
