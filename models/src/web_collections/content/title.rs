// src/web_collections/content/title.rs

use crate::core::content::Content;

pub struct TitleContent<'a> {
    pub title: &'a str
}

impl<'a> TitleContent<'a> {
    pub fn new(title: &'a str) -> Self {
        Self { title }
    }
}

impl<'a> Content<'a> for TitleContent<'a> {
    fn get_content(&self) -> String {
        format!("+ {}", &self.title)
    }
}

impl<'a> From<&'a str> for TitleContent<'a> {
    fn from(input: &'a str) -> Self {
        let title = input.strip_prefix("+ ").unwrap_or("[ ? ]");
        Self { title }
    }
}

impl Default for TitleContent<'_> {
    fn default() -> Self {
        Self { title: "default title" }
    }
}

impl std::fmt::Display for TitleContent<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f, "+ {}",
            self.title
        )
    }
}
