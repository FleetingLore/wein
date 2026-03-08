use models::content::*;
use models::domain::line::Line;

fn line_from_text_to_lore(raw: &str) -> Line {
    let indent = raw.trim_start_matches(" ").len() / 2;
    let content = if raw == "" {
        LineContent::Empty(Empty)
    } else if raw.starts_with("---") {
        LineContent::BreakLine(BreakLine)
    } else if raw.starts_with("# ") {
        LineContent::Comment(Comment { content: raw[2..].to_string() })
    } else if raw.starts_with("+ ") {
        LineContent::Title(Title { content: raw[2..].to_string() })
    } else if raw.contains(" = ") {
        let (info, category) = raw.split_once(" = ").unwrap();
        let info = info.to_string();
        let category = category.to_string();
        LineContent::LinkLore(LinkLore { info, category })
    } else if raw.contains(" > ") {
        let (info, md) = raw.split_once(" > ").unwrap();
        let info = info.to_string();
        let md = md.to_string();
        LineContent::LinkMd(LinkMd { info, md })
    } else if raw.contains(" | ") {
        let (info, url) = raw.split_once(" | ").unwrap();
        let info = info.to_string();
        let url = url.to_string();
        LineContent::LinkHtml(LinkHtml { info, url })
    } else {
         LineContent::Default
    };

    Line {
        indent,
        content,
    }
}

fn line_from_lore_to_html(lore: Line) -> String {
    match lore.content {
        LineContent::Empty(Empty) => "".to_string(),
        LineContent::BreakLine(BreakLine) => "<br>".to_string(),
        LineContent::Atom(Atom) => {
            format!(
                r##"<p style="#margin-left: {}rem">"##
                r##"<p>"##
                r##"{}</p></p>"##,
                lore.indent * 2,
                Atom.content
            )
        },
        LineContent::Title(Title) => {
            format!(
                r##"<p style="#margin-left: {}rem">"##
                r##"<strong>"##
                r##"{}</strong></p>"##,
                lore.indent * 2,
                Title.content
            )
        },
        LineContent::Comment(Comment) => {
            format!(
                "<!--"
                "{} -->",
                Comment.content
            )
        },
        LineContent::LinkLore(LinkLore) => {
            format!(
                r##"<p style="margin-left: {}rem">"##
                r##"<a href="#" class="link-lore" data-path="{}">"##
                r##"{}</a></p>"##,
                lore.indent * 2,
                LinkLore.category,
                LinkLore.info
            )
        },
        LineContent::LinkMd(LinkMd) => {
            format!(
                r##"<p style="margin-left: {}rem">"##
                r##"<a href="#" class="link-md" data-path="{}">"##
                r##"{}</a></p>"##,
                lore.indent * 2,
                LinkMd.md,
                LinkMd.info
            )
        },
        LineContent::LinkHtml(LinkHtml) => {
            format!(
                r##"<p style="margin-left: {}rem">"##
                r##"<a href="#" class="link-html" data-path="{}">"##
                r##"{}</a></p>"##,
                lore.indent * 2,
                LinkHtml.url,
                LinkHtml.info
            )
        },
        LineContent::Default => "".to_string()
    }
}