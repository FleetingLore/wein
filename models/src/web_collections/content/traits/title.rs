use crate::core::content::Content;

pub trait Title<'a> : Content<'a> {
    fn get_title(&self) -> String;
}
