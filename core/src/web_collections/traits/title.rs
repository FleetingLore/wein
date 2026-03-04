use crate::core::line::Line;

pub trait Title<'a> : Line<'a> {
    fn get_title(&self) -> String;
}