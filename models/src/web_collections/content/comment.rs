// src/web_collections/content/comment.rs

use crate::core::content::Content;

pub struct CommentContent<'a> {
    pub intro: &'a str
}

impl<'a> CommentContent<'a> {
    pub fn new(intro: &'a str) -> Self {
        Self {
            intro
        }
    }
}

impl<'a> Content<'a> for CommentContent<'a> {
    fn get_content(&self) -> String { format!("# {}", self.intro) }
}

impl<'a> From<&'a str> for CommentContent<'a> {
    fn from(input: &'a str) -> Self {
        let intro = input.strip_prefix("# ").unwrap_or("[ ? ]");
        Self {
            intro
        }
    }
}

impl Default for CommentContent<'_> {
    fn default() -> Self {
        Self::new("default intro" )
    }
}

impl std::fmt::Display for CommentContent<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f, "# {}",
            self.intro
        )
    }
}
