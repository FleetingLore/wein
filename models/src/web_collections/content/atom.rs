// src/web_collections/content/atom.rs

use crate::core::content::Content;

pub struct AtomContent<'a> {
    pub content: &'a str
}

impl<'a> AtomContent<'a> {
    pub fn new(content: &'a str) -> Self {
        Self { content }
    }
}

impl<'a> Content<'a> for AtomContent<'a> {
    fn get_content(&self) -> String {
        format!("{}", &self.content)
    }
}

impl<'a> From<&'a str> for AtomContent<'a> {
    fn from(input: &'a str) -> Self {
        Self { content: input.into() }
    }
}

impl Default for AtomContent<'_> {
    fn default() -> Self {
        Self { content: "default content" }
    }
}

impl std::fmt::Display for AtomContent<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f, "{}",
            self.content
        )
    }
}
