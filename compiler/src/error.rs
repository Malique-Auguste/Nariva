#[derive(Debug, PartialEq)]

//compilation errors
pub enum CompError {
    UnexpectedEOF(String),
    UnexpectedChar(String),
    Impossible(String),
    Overflow(String),
}
