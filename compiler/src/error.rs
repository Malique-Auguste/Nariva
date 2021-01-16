#[derive(Debug, PartialEq)]
pub enum CompError {
    UnexpectedEOF(String),
    UnexpectedChar(String),
    Overflow(String),
}
