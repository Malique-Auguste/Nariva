pub mod error;
pub mod token;
pub mod lexer;
pub mod parser;
pub mod generator;
pub mod compiler;




#[cfg(test)]
mod compiler_tests {
    use crate::generator::Generator;
    use crate::parser::*;
    use crate::lexer::*;
    use crate::token::*;
    use crate::compiler::*;


    #[test]
    fn basic_lexing() {
        let program = 
        "
        PUSH 3 -21
        PUSH 2 -2.1
        ADDF
        ";

        let mut lex = Lexer::new();
        let output = lex.lex(program).unwrap();

        assert_eq!(&vec![Token::Word("PUSH".to_string()), Token::NumU(3), Token::NumI(-21), Token::Word("PUSH".to_string()), Token::NumU(2), Token::NumF(-2.1), Token::Word("ADDF".to_string())], output)
    }

    #[test]
    fn basic_parsing() {
        let program = vec![Token::Word("PUSH".to_string()), Token::NumU(3),  Token::Word("PUSH".to_string()), Token::NumF(-2.1), Token::Word("ADDF".to_string())];
        let program2 = vec![Token::Word("PUSH".to_string()), Token::NumU(3), Token::Word("PUSH".to_string()), Token::NumF(-2.1), Token::Word("ADDF".to_string())];


        let output = Parser::parse(&program).unwrap();

        assert_eq!(&program2, output)
    }

    #[test]
    fn basic_generating() {
        let program = vec![Token::Word("Push".to_string()), Token::NumU(3),  Token::Word("PUSH".to_string()), Token::NumF(-2.1), Token::Word("ADDF".to_string())];
        let binary_code: Vec<u8> = vec![2, 0, 0, 0, 0, 0, 0 ,0 ,3, 2, 192, 0, 204, 204, 204, 204, 204, 205, 12];


        let mut gen = Generator::new();
        let output = gen.generate(&program).unwrap();

        assert_eq!(&binary_code, output)
    }

    #[test]
    fn basic_compiling() {
        let mut comp = Compiler::new("test.nar".to_string());
        let program = "
            PUSH 2.0
            PUSH 2.5
            ADDF
            PUSH 5.0
            DIVF
        ";

        assert_eq!(Ok(()), comp.compile(program))
    }
}