// src/models/empty.rs

use crate::core::line::Line;
use crate::core::error::LoreError;

/// 表示一个空行或只有空格的行
///
/// Empty 用于表示空行或只有空格的行，不包含任何实际内容。
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct EmptyLine;

impl EmptyLine {
    pub fn new() -> Self {
        Self {}
    }
}

impl Line<'_> for EmptyLine {
    fn get_indent(&self) -> usize {
        0
    }

    fn get_content(&self) -> String {
        String::new()
    }
}

impl std::fmt::Display for EmptyLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

impl<'a> TryFrom<&'a str> for EmptyLine {
    type Error = LoreError;

    fn try_from(raw: &'a str) -> Result<Self, Self::Error> {
        if raw.chars().all(|c| c == ' ') {
            Ok(Self)
        } else {
            Err(LoreError::InvalidFormat)
        }
    }
}

impl PartialEq<usize> for EmptyLine {
    fn eq(&self, _other: &usize) -> bool {
        true
    }
}