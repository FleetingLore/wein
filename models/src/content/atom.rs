// src/models/web_collections/atom.rs

/// 余项
pub struct Atom {
    pub atom: String,
}

impl Atom {
    pub fn new(content: String) -> Self {
        Self {
            atom: content
        }
    }
}

impl Into<String> for &Atom {
    fn into(self) -> String {
        self.atom.clone()
    }
}

impl From<String> for Atom {

    fn from(raw: String) -> Self {
        Self {
            atom: raw.to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_content() {
        let example = Atom::from(
            "example content".to_string()
        );
        let example: String = (&example).into();
        assert_eq!("example content", example);
        
    }
}