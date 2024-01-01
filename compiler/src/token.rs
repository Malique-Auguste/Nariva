#[derive(Debug, PartialEq)]
pub enum Token {
    Word (String),
    NumU (u64),
    NumI (i64),
    NumF (f64)
}

impl Token {
    pub fn is_num(&self) -> bool {
        match self {
            Token::NumU(_) | Token::NumI(_) | Token::NumF(_) => true,
            _ => false
        }
    }
}