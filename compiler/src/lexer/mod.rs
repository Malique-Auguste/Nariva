pub mod lexer;
pub mod token;


#[cfg(test)]
mod lexer_tests {
    use crate::lexer::lexer::Lexer;
    use crate::lexer::token::{Token, TokenType};

    #[test]
    fn lex_num() {
        let mut lexer = Lexer::new("\t12 + 3 - 15\n\r").unwrap();
        let expected = Ok(vec![
            Token::new(TokenType::Num(12), 1),
            Token::new(TokenType::Plus, 1),
            Token::new(TokenType::Num(3), 1),
            Token::new(TokenType::Minus, 1),
            Token::new(TokenType::Num(15), 1)
        ]);

        assert_eq!(expected, lexer.lex());
    }

    #[test]
    fn lex_string() {
        println!("{}", '\n'.is_alphanumeric());

        let mut lexer = Lexer::new("\nString\n new_name = \"Malique\"").unwrap();
        let expected = Ok(vec![
            Token::new(TokenType::Binding("String".into()), 2),
            Token::new(TokenType::Binding("new_name".into()), 3),
            Token::new(TokenType::Equal, 3),
            Token::new(TokenType::Str("Malique".into()), 3)
        ]);

        assert_eq!(expected, lexer.lex());
    }

    #[test]
    fn lex_keywords_brackets() {
        let mut lexer = Lexer::new("if (pro a) for x\nelse  in pub 2\nwhile priv _1").unwrap();
        let expected = Ok(vec![
            Token::new(TokenType::If, 1),
            Token::new(TokenType::LParenth, 1),
            Token::new(TokenType::Protected, 1),
            Token::new(TokenType::Binding("a".into()), 1),
            Token::new(TokenType::RParenth, 1),
            Token::new(TokenType::For, 1),
            Token::new(TokenType::Binding("x".into()), 1),

            Token::new(TokenType::Else, 2),
            Token::new(TokenType::In, 2),
            Token::new(TokenType::Public, 2),
            Token::new(TokenType::Num(2), 2),

            Token::new(TokenType::While, 3),
            Token::new(TokenType::Private, 3),
            Token::new(TokenType::Binding("_1".into()), 3),
        ]);

        assert_eq!(expected, lexer.lex());
    }
}