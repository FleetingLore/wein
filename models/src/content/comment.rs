// src/models/web_collections/comment.rs

/// 注释
pub struct Comment {
    pub comment: String,
}

impl Comment {
    pub fn new(content: String) -> Self {
        Self {
            comment: content
        }
    }
}

impl Into<String> for &Comment {
    fn into(self) -> String {
        format!("# {}", self.comment)
    }
}

impl From<String> for Comment {

    fn from(raw: String) -> Self {
        Self{
            comment: raw[2..].to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_content() {
        let example = Comment::from(
            "# example content".to_string()
        );
        let example: String = (&example).into();
        assert_eq!("# example content", example);
    }
}