pub trait Line<'a> {
    fn get_indent(&self) -> usize;
    fn get_content(&self) -> String;
}
