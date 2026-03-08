// src/models/web_collections/comment.rs

/// 注释
pub struct Comment {
    content: String,
}

impl Comment {
    pub fn new(content: String) -> Self {
        Self {
            content
        }
    }
}

impl Into<String> for &Comment {
    fn into(self) -> String {
        format!("# {}", self.content)
    }
}

impl From<String> for Comment {

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
        let example = Comment::new(
            "example content".to_string());
        let example: String = (&example).into();
        assert_eq!("# example content", example);

        let example = Comment::from(
            "# example content".to_string());
        let example: String = (&example).into();
        assert_eq!("# example content", example);
    }
}