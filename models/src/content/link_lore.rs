// src/models/web_collections/link_lore.rs

/// Html 链接
pub struct LinkLore {
    pub info: String,
    pub category: String,
}

impl LinkLore {
    pub fn new(info: String, category: String) -> Self {
        Self {
            info,
            category
        }
    }
}

impl Into<String> for &LinkLore {
    fn into(self) -> String {
        format!(
            "{} = {}",
            self.info,
            self.category
        )
    }
}

impl From<String> for LinkLore {

    fn from(raw: String) -> Self {
        let (info, url) = raw
            .split_once(" = ")
            .unwrap();
        let info = info.to_string();
        let url = url.to_string();
        Self{
            info,
            category: url,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_content() {
        let example = LinkLore::from(
            "example info = example category".to_string()
        );
        let example: String = (&example).into();
        assert_eq!(
            "example info = example category",
            example
        );
    }
}