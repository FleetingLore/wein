use models::content::LineContent;
use models::domain::line::Line;
use crate::line::*;

/// **raw**: lore 源
pub fn file_from_text_to_lore(raw: String) -> Vec<Line> {
    // 解析结果保存到此处
    let mut lines: Vec<Line> = Vec::new();

    // 一行一行解析
    for line in raw.lines() {
        lines.push(line_from_text_to_lore(line));
    }

    // 返回数据对象
    lines
}

/// 从 Lore 数据对象到 Html
pub fn file_from_lore_to_html(lines: Vec<Line>, base: &str) -> String {
    let mut target: Vec<String> = Vec::new();
    for line in lines {
        let indent = line.indent;
        let line_html: String = match line.content {
            LineContent::Empty(_src) => "".to_string(),
            LineContent::BreakLine(_src) => from_break_line_to_html(),
            LineContent::Comment(src) => from_comment_to_html(src.comment),
            LineContent::Title(src) => from_title_to_html(indent, src.title),
            LineContent::LinkLore(src) => from_link_lore_to_html(indent, src.info, src.category, base),
            LineContent::LinkMd(src) => from_link_md_to_html(indent, src.info, src.md, base),
            LineContent::LinkHtml(src) => from_link_html_to_html(indent, src.info, src.url),
            LineContent::Atom(src) => from_atom_to_html(indent, src.atom),
        };
        target.push(line_html);
    }
    target.join("\n")
}
