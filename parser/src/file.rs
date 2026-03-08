use models::domain::file::File;
use models::domain::line::Line;
use crate::line::{line_from_lore_to_html, line_from_text_to_lore};

pub fn file_from_text_to_lore(name: String, raw: String) -> File {
    let mut lines: Vec<Line> = Vec::new();

    for line in raw.lines() {
        lines.push(line_from_text_to_lore(line));
    }

    File {
        name,
        lines
    }
}

pub fn file_from_lore_to_html(file: File) -> String {

    let mut target: Vec<String> = Vec::new();

    let style = r##"<style>
:root {
    --bg: #141516;
    --text: #E6EDF0;
    --text-muted: #c4d9ff;
    --primary: #68c7c7;
    --error: #E67E7E;
}

* {
    margin: 0;
    padding: 0;
    box-sizing: border-box;
}

body {
    background: var(--bg);
    color: var(--text);
    font-family: sans-serif;
    line-height: 1.6;
    padding: 20px;
}

p {
    color: var(--text-muted);
    margin-bottom: 1.2rem;
}

strong {
    color: var(--text);
    font-weight: 600;
    font-size: 1.3rem;
    display: block;
}

a {
    color: var(--primary);
    text-decoration: none;
}

a:hover {
    opacity: 0.8;
}
</style>"##;

    for line in file.lines {
        target.push(line_from_lore_to_html(line));
    }

    let target= target.join("\n");

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
        file.name,
        style,
        target
    )
}