#[derive(Debug, PartialEq)]
pub enum CompError {
    UnexpectedEOF(String),
    Overflow(String),
}
