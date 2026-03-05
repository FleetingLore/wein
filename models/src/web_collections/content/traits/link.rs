use crate::core::content::Content;

pub trait Link<'a> : Content<'a> {
    fn get_info(&self) -> String;
    fn get_url(&self) -> String;
}