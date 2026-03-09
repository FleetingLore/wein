use models::content::*;
use models::domain::line::Line;

/// 从原始文本到 lore
pub fn line_from_text_to_lore(raw: &str) -> Line {
    // 获取缩进
    let indent = raw.chars().take_while(|&c| c == ' ').count();
    let raw = raw.split_at(indent).1;
    let indent = indent / 2;

    // 返回中间表示
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
        LineContent::Atom(
            Atom {
                atom: raw.to_string(),
            }
        )
    };

    // 返回数据对象
    Line {
        indent,
        content,
    }
}

/// 从 lore 到 html
pub fn line_from_lore_to_html(lore: Line) -> String {
    // 一一对应
    match lore.content {
        LineContent::Empty(Empty) => "".to_string(),
        LineContent::BreakLine(BreakLine) => "<br>".to_string(),

        LineContent::Title(content) => {
            format!(
                r#"<p style="margin-left: {}rem"><strong>{}</strong></p>"#,
                lore.indent,
                content.title
            )
        },

        LineContent::Comment(content) => {
            format!(
                "<!-- {} -->",
                content.comment
            )
        },

        LineContent::LinkLore(content) => {
            format!(
                r#"<p style="margin-left: {}rem"><a href="https://fleetinglore.github.io/{}.html">{}</a></p>"#,
                lore.indent,
                content.category,
                content.info
            )
        },

        LineContent::LinkMd(content) => {
            format!(
                r#"<p style="margin-left: {}rem"><a href="https://fleetinglore.github.io/{}">{}</a></p>"#,
                lore.indent,
                content.md,
                content.info
            )
        },

        LineContent::LinkHtml(content) => {
            format!(
                r##"<p style="margin-left: {}rem"><a href="{}">{}</a></p>"##,
                lore.indent,
                content.url,
                content.info
            )
        },
        LineContent::Atom(content) => {
            format!(
                r#"<p style="margin-left: {}rem">{}</p>"#,
                lore.indent,
                content.atom
            )
        }
    }
}
