
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum TokenType {
    Num(i16),
    Str(String),
    Binding(String),

    Constant,

    If,
    Else,
    For,
    While,
    In,

    Public,
    Protected,
    Private,

    LBracket,
    RBracket,
    LBrace,
    RBrace,
    LParenth,
    RParenth,

    Plus,
    Minus,
    Equal,

    Whitespace,
    Tab,
    Return,
    NewLine,
}

#[derive(PartialEq)]
pub struct Token {
    token_type:TokenType,
    line: usize
}

impl Token {
    pub fn new(token_type: TokenType, line: usize) -> Token {
        Token { token_type, line }
    }
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{:?} : {}", self.token_type, self.line)
    }
}