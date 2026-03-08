// src/models/web_collections/link_md.rs

/// Html 链接
pub struct LinkMd {
    pub info: String,
    pub md: String,
}

impl LinkMd {
    pub fn new(info: String, md: String) -> Self {
        Self {
            info,
            md
        }
    }
}

impl Into<String> for &LinkMd {
    fn into(self) -> String {
        format!(
            "{} > {}",
            self.info,
            self.md
        )
    }
}

impl From<String> for LinkMd {

    fn from(raw: String) -> Self {
        let (info, md) = raw
            .split_once(" > ")
            .unwrap();
        let info = info.to_string();
        let md = md.to_string();
        Self{
            info,
            md,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_content() {
        let example = LinkMd::from(
            "example info > example md".to_string()
        );
        let example: String = (&example).into();
        assert_eq!(
            "example info > example md",
            example
        );
    }
}