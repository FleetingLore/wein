// src/models/web_collections/empty.rs

/// 空
pub struct Empty;

impl Empty {
    pub fn new() -> Self {
        Self {}
    }
}

impl Into<String> for Empty {
    fn into(self) -> String {
        "".to_string()
    }
}

impl From<String> for Empty {

    fn from(_: String) -> Self {
        Self {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let example = Empty::new();
        let example: String = example.into();
        assert_eq!("", example);

        let example = Empty::from("".to_string());
        let example: String = example.into();
        assert_eq!("", example);
    }
}