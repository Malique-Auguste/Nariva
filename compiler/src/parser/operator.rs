use crate::lexer::token::TokenType;
use std::convert::TryFrom;

#[derive(Debug, PartialEq)]
pub enum Operator {
    Plus,
    Dash,
    Star,
    Slash,

    Bang,
    Equal,
    Greater,
    Lesser,

    EqualEqual,
    BangEqual,
    GreaterEqual,
    LesserEqual,
}

impl TryFrom<TokenType> for Operator {
    type Error = String;
    fn try_from(o: TokenType) -> Result<Operator, String> {
        match o {
            TokenType::Dash => Ok(Operator::Dash),
            TokenType::Plus => Ok(Operator::Plus),
            TokenType::Star => Ok(Operator::Star),
            TokenType::Slash => Ok(Operator::Slash),

            TokenType::Bang => Ok(Operator::Bang),
            TokenType::Equal => Ok(Operator::Equal),
            TokenType::Greater => Ok(Operator::Greater),
            TokenType::Lesser => Ok(Operator::Lesser),

            TokenType::EqualEqual => Ok(Operator::EqualEqual),
            TokenType::LesserEqual => Ok(Operator::LesserEqual),
            TokenType::GreaterEqual => Ok(Operator::GreaterEqual),
            TokenType::BangEqual => Ok(Operator::BangEqual),

            _ => Err(format!("{:?}, is not an operator", o))
        }
    }
}