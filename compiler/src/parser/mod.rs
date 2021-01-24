pub mod parser;
pub mod expression;
pub mod operator;
pub mod statement;

#[cfg(test)]
mod parser_tests {
    use crate::lexer::token::{Token, TokenType};
    use crate::parser::parser::Parser;

    #[test]
    fn split_by_line() {
        let tokens: Vec<Token> = vec![
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
        ];

        let expected: Vec<Vec<Token>> = vec![
            vec![
                Token::new(TokenType::If, 1),
                Token::new(TokenType::LParenth, 1),
                Token::new(TokenType::Protected, 1),
                Token::new(TokenType::Binding("a".into()), 1),
                Token::new(TokenType::RParenth, 1),
                Token::new(TokenType::For, 1),
                Token::new(TokenType::Binding("x".into()), 1),
            ],
            vec![
                Token::new(TokenType::Else, 2),
                Token::new(TokenType::In, 2),
                Token::new(TokenType::Public, 2),
                Token::new(TokenType::Num(2), 2),
            ],
            vec![
                Token::new(TokenType::While, 3),
                Token::new(TokenType::Private, 3),
                Token::new(TokenType::Binding("_1".into()), 3),
            ]
        ];

        assert_eq!(expected, Parser::split_by_line(tokens));
    }
}