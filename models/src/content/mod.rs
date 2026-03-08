/// 余项
pub mod atom;

/// 起域者
pub mod title;

/// 注释
pub mod comment;

/// lore 链接
pub mod link_lore;

/// markdown 链接
pub mod link_md;

/// html 链接
pub mod link_html;

/// 分页符
pub mod br;

/// 空
pub mod empty;

pub enum LineContent {
    Empty(empty::Empty),
    BreakLine(br::BreakLine),
    Atom(atom::Atom),
    Title(title::Title),
    Comment(comment::Comment),
    LinkLore(link_lore::LinkLore),
    LinkMd(link_md::LinkMd),
    LinkHtml(link_html::LinkHtml),
    Default
}