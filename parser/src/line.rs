use models::content::*;
use models::domain::line::Line;

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

pub fn from_break_line_to_html() -> String {
    "<br>".to_string()
}

pub fn from_atom_to_html(indent: usize, atom: String) -> String {
    format!(
        r#"<p style="margin-left: {}rem" class="atom">{}</p>"#,
        indent,
        atom
    )
}

pub fn from_title_to_html(indent: usize, title: String) -> String {
    format!(
        r#"<p style="margin-left: {}rem" class="title">{}</p>"#,
        indent,
        title
    )
}

pub fn from_comment_to_html(comment: String) -> String {
    format!(
        "<!-- {} -->",
        comment
    )
}

pub fn from_link_lore_to_html(
    indent: usize,
    info: String,
    category: String,
    base: &str
) -> String {
    format!(
        r#"<p style="margin-left: {}rem"><a href="{}/index/{}.html" class="link_lore">{}</a></p>"#,
        indent,
        base,
        category,
        info
    )
}

pub fn from_link_md_to_html(
    indent: usize,
    info: String,
    md: String,
    base: &str
) -> String {
    format!(
        r#"<p style="margin-left: {}rem"><a href="{}/{}" class="link_lore">{}</a></p>"#,
        indent,
        base,
        md,
        info
    )
}

pub fn from_link_html_to_html(
    indent: usize,
    info: String,
    url: String
) -> String {
    format!(
        r##"<p style="margin-left: {}rem"><a href="{}" class="link_html">{}</a></p>"##,
        indent,
        url,
        info
    )
}
