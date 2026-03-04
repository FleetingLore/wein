use crate::core::line::Line;

pub trait Link<'a> : Line<'a> {
    fn get_info(&self) -> String;
    fn get_url(&self) -> String;
}