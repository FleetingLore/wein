use std::fmt::Display;

pub struct Lore<'a> {
    pub tokens: &'a str
}

impl<'a> From<&'a str> for Lore<'a> {
    fn from(value: &'a str) -> Self {
        Lore { tokens: value }
    }
}

impl Default for Lore<'_> {
    fn default() -> Self {
        Lore { tokens: "_" }
    }
}

impl Display for Lore<'_> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(formatter, "{}", self.tokens)
    }
}