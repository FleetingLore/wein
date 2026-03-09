pub struct LoreHtml {
    pub title: String,
    pub style: String,
    pub content: String,
}

impl LoreHtml {
    pub fn new(title: String, style: String, content: String) -> LoreHtml {
        LoreHtml {
            title,
            style,
            content,
        }
    }
}

impl Into<String> for LoreHtml {
    fn into(self) -> String {
        format!(
            r#"<html lang="zh-CN">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>{}</title>
{}
</head>
<body><main>
{}
</main></body>
</html>"#,
            self.title,
            self.style,
            self.content,
        )
    }
}