// src/models/web_collections/title.rs

/// 起域者
pub struct Title {
    content: String,
}

impl Title {
    pub fn new(content: String) -> Self {
        Self {
            content
        }
    }
}

impl Into<String> for Title {
    fn into(self) -> String {
        format!("+ {}", self.content)
    }
}

impl From<String> for Title  {

    fn from(raw: String) -> Self {
        Self{
            content: raw[2..].to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_content() {
        let example = Title::new(
            "example content".to_string()
        );
        let example: String = example.into();
        assert_eq!(
            "+ example content", 
            example
        );

        let example = Title::from(
            "+ example content".to_string()
        );
        let example: String = example.into();
        assert_eq!(
            "+ example content", 
            example
        );
    }
}