// src/models/web_collections/link_link.rs

/// Html 链接
pub struct LinkHtml {
    info: String,
    url: String,
}

impl LinkHtml {
    pub fn new(info: String, goto: String) -> Self {
        Self {
            info,
            url: goto
        }
    }
}

impl Into<String> for LinkHtml {
    fn into(self) -> String {
        format!(
            "{} | {}", 
            self.info, 
            self.url
        )
    }
}

impl From<String> for LinkHtml {

    fn from(raw: String) -> Self {
        let (info, url) = raw
            .split_once(" | ")
            .unwrap();
        let info = info.to_string();
        let url = url.to_string();
        Self{
            info,
            url,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_content() {
        let example = LinkHtml::new(
            "example info".to_string(),
            "example url".to_string());
        let example: String = example.into();
        assert_eq!(
            "example info | example url", 
            example
        );

        let example = LinkHtml::from(
            "example info | example url".to_string()
        );
        let example: String = example.into();
        assert_eq!(
            "example info | example url", 
            example
        );
    }
}