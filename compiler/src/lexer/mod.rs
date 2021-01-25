pub mod lexer;
pub mod token;


#[cfg(test)]
mod lexer_tests {
    use crate::lexer::lexer::Lexer;
    use crate::lexer::token::{Token, TokenType};

    #[test]
    fn lex_num() {
        let mut lexer = Lexer::new("\t12 + (3.5 - 15.255) * 2 / 3\n\r").unwrap();
        let expected = Ok(vec![
            Token::new(TokenType::U64(12), 1),
            Token::new(TokenType::Plus, 1),
            Token::new(TokenType::LParenth, 1),
            Token::new(TokenType::F64(3.5), 1),
            Token::new(TokenType::Dash, 1),
            Token::new(TokenType::F64(15.255), 1),
            Token::new(TokenType::RParenth, 1),
            Token::new(TokenType::Star, 1),
            Token::new(TokenType::U64(2), 1),
            Token::new(TokenType::Slash, 1),
            Token::new(TokenType::U64(3), 1),
        ]);

        assert_eq!(expected, lexer.lex());
    }

    #[test]
    fn lex_string() {
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
    fn lex_equalitites() {
        let mut lexer = Lexer::new("= >= ! < > <= ==").unwrap();
        let expected = Ok(vec![
            Token::new(TokenType::Equal, 1),
            Token::new(TokenType::GreaterEqual, 1),
            Token::new(TokenType::Bang, 1),
            Token::new(TokenType::Lesser, 1),
            Token::new(TokenType::Greater, 1),
            Token::new(TokenType::LesserEqual, 1),
            Token::new(TokenType::EqualEqual, 1),
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
            Token::new(TokenType::U64(2), 2),

            Token::new(TokenType::While, 3),
            Token::new(TokenType::Private, 3),
            Token::new(TokenType::Binding("_1".into()), 3),
        ]);

        assert_eq!(expected, lexer.lex());
    }
}