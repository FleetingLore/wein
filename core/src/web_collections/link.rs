// src/web_collections/link.rs

use crate::core::line::Line;
use crate::core::lore::Lore;
use crate::web_collections::error::LoreError;
use crate::web_collections::traits::link::Link;

pub struct LinkLine<'a> { pub indent: usize, pub info: Lore<'a>, pub url: Lore<'a> }

impl<'a> LinkLine<'a> {
    pub fn new(indent: usize, info: &'a str, url: &'a str) -> Self {
        Self { indent, info: info.into(), url: url.into() }
    }
}

impl<'a> Line<'a> for LinkLine<'a> {
    fn get_indent(&self) -> usize { self.indent }

    fn get_content(&self) -> String { format!("{} = {}", &self.info, &self.url) }
}

impl<'a> Link<'a> for LinkLine<'a> {
    fn get_info(&self) -> String { self.info.to_string() }

    fn get_url(&self) -> String { self.url.to_string() }
}

impl Default for LinkLine<'_> {
    fn default() -> Self { Self::new(0, "default info", "default url") }
}

impl<'a> TryFrom<&'a str> for LinkLine<'a> {
    type Error = LoreError;

    fn try_from(raw: &'a str) -> Result<Self, Self::Error> {
        if raw.is_empty() { return Err(LoreError::InvalidFormat) }

        let indent_count = raw.chars()
            .take_while(|c| *c == ' ')
            .count();
        
        if indent_count % 2 != 0 { return Err(LoreError::IndentIsNotEven) }
        
        let content = raw[indent_count..].trim_start();

        match content.split_once(" = ") {
            Some((info, url)) => {
                let info = info.trim();
                let url = url.trim();

                if info.is_empty() || url.is_empty() {
                    Err(LoreError::InvalidFormat)
                } else {
                    Ok(LinkLine::new(indent_count / 2, info, url))
                }
            }
            None => Err(LoreError::InvalidFormat)
        }
    }
}

impl std::fmt::Display for LinkLine<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f, "{}{} = {}",
            "  ".repeat(self.indent),
            self.info,
            self.url
        )
    }
}
