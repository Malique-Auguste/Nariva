pub mod error;
pub mod lexer;
use lexer::{lexer::Lexer, token::{Token, TokenType}};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_num_lex() {
        let mut lexer = Lexer::new("\t12 + 3 - 5\n\r").unwrap();
        let expected = Ok(vec![
            Token::new(TokenType::Num(12), 1),
            Token::new(TokenType::Plus, 1),
            Token::new(TokenType::Num(3), 1),
            Token::new(TokenType::Minus, 1),
            Token::new(TokenType::Num(5), 1)
        ]);

        assert_eq!(expected, lexer.lex());
    }
}
