use models::domain::line::Line;
use crate::line::{line_from_lore_to_html, line_from_text_to_lore};

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
pub fn file_from_lore_to_html(lines: Vec<Line>) -> String {
    let mut target: Vec<String> = Vec::new();
    for line in lines {
        target.push(line_from_lore_to_html(line));
    }
    target.join("\n")
}
