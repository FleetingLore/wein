use models::content::*;
use models::domain::line::Line;

pub fn line_from_text_to_lore(raw: &str) -> Line {
    let indent = raw.chars().take_while(|&c| c == ' ').count();
    let raw = raw.split_at(indent).1;
    let indent = indent / 2;

    let content = if raw == "" {
        LineContent::Empty(Empty)

    } else if raw.starts_with("---") {
        LineContent::BreakLine(BreakLine)

    } else if raw.starts_with("# ") {
        let comment = raw[2..].to_string();
        LineContent::Comment(
            Comment {
                comment
            }
        )

    } else if raw.starts_with("+ ") {
        let title = raw[2..].to_string();
        LineContent::Title(
            Title {
                title
            }
        )

    } else if raw.contains(" = ") {
        let (info, category) = raw.split_once(" = ").unwrap();
        let info = info.to_string();
        let category = category.to_string();
        LineContent::LinkLore(LinkLore { info, category })

    } else if raw.contains(" > ") {
        let (info, md) = raw.split_once(" > ").unwrap();
        let info = info.to_string();
        let md = md.to_string();
        LineContent::LinkMd(
            LinkMd {
                info,
                md
            }
        )

    } else if raw.contains(" | ") {
        let (info, url) = raw
            .split_once(" | ")
            .unwrap();
        let info = info.to_string();
        let url = url.to_string();
        LineContent::LinkHtml(
            LinkHtml {
                info,
                url
            }
        )
    } else {
        LineContent::Default
    };

    Line {
        indent,
        content,
    }
}

pub fn line_from_lore_to_html(lore: Line) -> String {
    match lore.content {
        LineContent::Empty(Empty) => "".to_string(),
        LineContent::BreakLine(BreakLine) => "<br>".to_string(),
        LineContent::Atom(content) => {
            println!("{}", lore.indent);
            format!(
                r#"<p style="margin-left: {}rem"><p>{}</p></p>"#,
                lore.indent,
                content.atom
            )
        },
        LineContent::Title(content) => {
            println!("{}", lore.indent);
            format!(
                r#"<p style="margin-left: {}rem"><strong>{}</strong></p>"#,
                lore.indent,
                content.title
            )
        },
        LineContent::Comment(content) => {
            println!("{}", lore.indent);

            format!(
                "<!-- {} -->",
                content.comment
            )
        },
        LineContent::LinkLore(content) => {
            println!("{}", lore.indent);

            format!(
                r#"<p style="margin-left: {}rem"><a href="{}">{}</a></p>"#,
                lore.indent,
                content.category,
                content.info
            )
        },
        LineContent::LinkMd(content) => {
            println!("{}", lore.indent);

            format!(
                r#"<p style="margin-left: {}rem"><a href="{}">{}</a></p>"#,
                lore.indent,
                content.md,
                content.info
            )
        },
        LineContent::LinkHtml(content) => {
            println!("{}", lore.indent);

            format!(
                r##"<p style="margin-left: {}rem"><a href="{}">{}</a></p>"##,
                lore.indent,
                content.url,
                content.info
            )
        },
        LineContent::Default => "".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_line_from_text_to_lore() {
        let example = "    a | b.com";
        let target = line_from_text_to_lore(example);
        assert_eq!(target.indent, 2);
    }
}