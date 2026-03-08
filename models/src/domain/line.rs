use crate::content::LineContent;

pub struct Line {
    pub content: LineContent,
    pub indent: usize,
}

/*
impl Display for Line {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}",
            "  ".repeat(self.indent),
            "  " // TODO
        )
    }
}
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
