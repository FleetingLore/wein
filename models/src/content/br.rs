// src/models/web_collections/br.rs

/// 分页符
pub struct BreakLine;

impl BreakLine {
    pub fn new() -> Self {
        Self {}
    }
}

impl Into<String> for &BreakLine {
    fn into(self) -> String {
        "---".to_string()
    }
}

impl From<String> for BreakLine {

    fn from(_: String) -> Self {
        Self {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_content() {
        let example = BreakLine::from("".to_string());
        let example: String = (&example).into();
        assert_eq!("---", example);
    }
}