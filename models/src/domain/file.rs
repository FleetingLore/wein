use std::fmt::{Display, Formatter};
use crate::domain::line::Line;

pub struct File {
    pub name: String,
    pub lines: Vec<Line>
}

impl Display for File {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for line in self.lines.iter() {
            write!(f, "{}", line)?;
        }
        Ok(())
    }
}

/*


impl File {
    fn load(path: String) -> File {
        let path = todo!();
        let file = File {
            name: path,
            lines: vec![]
        };
        file
    }

    fn parse(path: String) {
        let raw = load(path);

    }
}
*/
