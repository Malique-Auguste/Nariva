#[derive(Debug, PartialEq)]
pub enum Token {
    Word (String),
    NumU (u64),
    NumI (i64),
    NumF (f64)
}