use crate::content::LineContent;

pub struct Line {
    pub content: LineContent,
    pub indent: usize,
}

impl std::fmt::Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}",
               "  ".repeat(self.indent),
               {
                   let content = match &self.content {
                       LineContent::Empty(line) => "".to_string(),
                       LineContent::BreakLine(line) => "---".to_string(),
                       LineContent::Atom(line) => line.into(),
                       LineContent::Comment(line) => line.into(),
                       LineContent::Title(line) => line.into(),
                       LineContent::LinkLore(line) => line.into(),
                       LineContent::LinkMd(line) => line.into(),
                       LineContent::LinkHtml(line) => line.into(),
                       LineContent::Default => "".to_string(),
                   };
                   content
               }
        )
    }
}

/*

*/

/*
impl Line {
    fn parse(line: String) -> Line {
        let content = line.chars().take_while(|c| c == &' ');
        Line {
            content: _, // TODO
            indent: 0 // TODO
        }
    }
}
*/
