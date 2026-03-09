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

pub use br::BreakLine;
pub use atom::Atom;
pub use comment::Comment;
pub use empty::Empty;
pub use link_html::LinkHtml;
pub use link_lore::LinkLore;
pub use link_md::LinkMd;
pub use title::Title;

pub enum LineContent {
    Empty(Empty),
    BreakLine(BreakLine),
    Title(Title),
    Comment(Comment),
    LinkLore(LinkLore),
    LinkMd(LinkMd),
    LinkHtml(LinkHtml),
    Atom(Atom)
}