pub mod error;
pub mod token;
pub mod lexer;
pub mod parser;
pub mod generator;



#[cfg(test)]
mod compiler_tests {
    use crate::generator::Generator;
    use crate::parser::*;
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

    #[test]
    fn basic_parsing() {
        let program = vec![Token::Word("PUSH".to_string()), Token::NumU(3),  Token::Word("PUSH".to_string()), Token::NumF(-2.1), Token::Word("ADDF".to_string())];
        let program2 = vec![Token::Word("PUSH".to_string()), Token::NumU(3), Token::Word("PUSH".to_string()), Token::NumF(-2.1), Token::Word("ADDF".to_string())];


        let mut par = Parser::new(program).unwrap();
        let output = par.parse().unwrap();

        assert_eq!(&program2, output)
    }

    #[test]
    fn basic_generating() {
        let program = vec![Token::Word("Push".to_string()), Token::NumU(3),  Token::Word("PUSH".to_string()), Token::NumF(-2.1), Token::Word("ADDF".to_string())];
        let binary_code: Vec<u8> = vec![2, 0, 0, 0, 0, 0, 0 ,0 ,3, 2, 192, 0, 204, 204, 204, 204, 204, 205, 12];


        let mut gen = Generator::new(program).unwrap();
        let output = gen.generate().unwrap();

        assert_eq!(&binary_code, output)
    }
}