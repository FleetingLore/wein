use crate::core::line::Line;

pub trait Category<'a> {
    fn get_title(&'a self) -> Option<Box<dyn Line<'a>>>;
    fn get_lines(&'a self) -> Option<Vec<Box<dyn Line<'a>>>>;
}
