// src/models/web_collections/atom.rs

/// 余项
pub struct Atom {
    content: String,
}

impl Atom {
    pub fn new(content: String) -> Self {
        Self {
            content
        }
    }
}

impl Into<String> for &Atom {
    fn into(self) -> String {
        self.content.clone()
    }
}

impl From<String> for Atom {

    fn from(raw: String) -> Self {
        Self {
            content: raw.to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_content() {
        let example = Atom::new(
            "example content".to_string());
        let example: String = (&example).into();
        assert_eq!("example content", example);

        let example = Atom::from(
            "example content".to_string());
        let example: String = (&example).into();
        assert_eq!("example content", example);
    }
}