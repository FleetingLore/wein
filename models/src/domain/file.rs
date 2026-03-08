use crate::domain::line::Line;

pub struct File {
    pub name: String,
    pub lines: Vec<Line>
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
