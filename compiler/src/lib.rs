pub mod error;
pub mod token;
pub mod lexer;

#[cfg(test)]
mod compiler_tests {
    use crate::lexer::*;
    use crate::token::*;

    #[test]
    fn basic_lexing() {
        let program = 
        "
        PUSH 3 -21
        PUSH 2 -2.1
        ADDF
        ";

        let mut lex = Lexer::new(program).unwrap();
        let output = lex.lex().unwrap();

        assert_eq!(&vec![Token::Word("PUSH".to_string()), Token::NumU(3), Token::NumI(-21), Token::Word("PUSH".to_string()), Token::NumU(2), Token::NumF(-2.1), Token::Word("ADDF".to_string())], output)
    }

    fn basic_parsing() {
        let program = 
        "
        PUSH 3 -21
        PUSH 2 -2.1
        ADDF
        ";

        let mut lex = Lexer::new(program).unwrap();
        let output = lex.lex().unwrap();

        assert_eq!(&vec![Token::Word("PUSH".to_string()), Token::NumU(3), Token::NumI(-21), Token::Word("PUSH".to_string()), Token::NumU(2), Token::NumF(-2.1), Token::Word("ADDF".to_string())], output)
    }
}