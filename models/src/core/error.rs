#[derive(Debug)]
pub enum LoreError {
    IndentIsNotEven, // indent should be even
    InvalidFormat,   // raw string not in format
}
