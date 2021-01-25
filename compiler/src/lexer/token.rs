
use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    U64(u64),
    F64(f64),
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
    Dash,
    Star,
    Slash,
    Dot,

    Equal,
    Bang,
    Greater,
    Lesser,

    EqualEqual,
    BangEqual,
    GreaterEqual,
    LesserEqual,

    Whitespace,
    Tab,
    Return,
    NewLine,
}

#[derive(PartialEq, Clone)]
pub struct Token {
    pub token_type:TokenType,
    pub line: usize
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