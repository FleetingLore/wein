// src/web_collections/content/link.rs

use crate::core::content::Content;
use crate::web_collections::content::traits::link::Link;

pub struct LinkContent<'a> {
    pub info: &'a str,
    pub url: &'a str
}

impl<'a> LinkContent<'a> {
    pub fn new(info: &'a str, url: &'a str) -> Self {
        Self {
            info,
            url
        }
    }
}

impl<'a> Content<'a> for LinkContent<'a> {
    fn get_content(&self) -> String {
        format!("{} = {}", &self.info, &self.url)
    }
}

impl<'a> Link<'a> for LinkContent<'a> {
    fn get_info(&self) -> String {
        self.info.to_string()
    }

    fn get_url(&self) -> String {
        self.url.to_string()
    }
}

impl Default for LinkContent<'_> {
    fn default() -> Self { Self::new("default info", "default url") }
}

impl std::fmt::Display for LinkContent<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f, "{} = {}",
            self.info,
            self.url
        )
    }
}
