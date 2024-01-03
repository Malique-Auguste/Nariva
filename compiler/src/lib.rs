pub mod error;
pub mod token;
pub mod lexer;
pub mod parser;
pub mod generator;
pub mod compiler;
extern crate virtual_machine;




#[cfg(test)]
mod compiler_tests {
    use crate::generator::Generator;
    use crate::parser::*;
    use crate::lexer::*;
    use crate::token::*;
    use crate::compiler::*;
    use virtual_machine::vm::HEADER;



    #[test]
    fn basic_lexing() {
        let program = 
        "
        PUSH 3 -21
        PUSH 2 -2.1
        ADDF
        ";

        let output = Lexer::lex(program).unwrap();

        assert_eq!(vec![Token::OpCode("PUSH".to_string()), Token::NumU(3), Token::NumI(-21), Token::OpCode("PUSH".to_string()), Token::NumU(2), Token::NumF(-2.1), Token::OpCode("ADDF".to_string())], output)
    }

    #[test]
    fn basic_parsing() {
        let program = vec![Token::OpCode("PUSH".to_string()), Token::NumU(3),  Token::OpCode("PUSH".to_string()), Token::NumF(-2.1), Token::OpCode("ADDF".to_string())];
        let program2 = vec![Token::OpCode("PUSH".to_string()), Token::NumU(3), Token::OpCode("PUSH".to_string()), Token::NumF(-2.1), Token::OpCode("ADDF".to_string())];


        let output = Parser::parse(program).unwrap();

        assert_eq!(program2, output)
    }

    #[test]
    fn basic_generating() {
        let program = vec![Token::OpCode("Push".to_string()), Token::NumU(3),  Token::OpCode("PUSH".to_string()), Token::NumF(-2.1), Token::OpCode("ADDF".to_string())];
        let mut binary_code: Vec<u8> = [HEADER.to_vec(), [2, 0, 0, 0, 0, 0, 0 ,0 ,3, 2, 192, 0, 204, 204, 204, 204, 204, 205, 12].to_vec()].concat();



        let output = Generator::generate(program).unwrap();

        assert_eq!(binary_code, output)
    }

    #[test]
    fn basic_compiling() {
        let mut comp = Compiler::new("../nar files/test.nar".to_string());
        let program = "
            PUSH 2.0
            PUSH 2.5
            ADDF
            PUSH 5.0
            DIVF
        ";

        assert_eq!(Ok(()), comp.compile(program, true))
    }

    #[test]
    fn lex_func() {
        let program = 
        "
        PUSH -21
        ADDF

        my_func:
        PUSH 2
        RETURN
        ";

        let output = Lexer::lex(program).unwrap();

        assert_eq!(vec![Token::OpCode("PUSH".to_string()), Token::NumI(-21), Token::OpCode("ADDF".to_string()), Token::Func("my_func".to_string()), Token::OpCode("PUSH".to_string()), Token::NumU(2), Token::OpCode("RETURN".to_string()), ], output)
    }

    #[test]
    fn parse_func() {
        let program = vec![Token::OpCode("PUSH".to_string()), Token::NumI(-21), Token::OpCode("ADDF".to_string()), Token::Func("my_func".to_string()), Token::OpCode("PUSH".to_string()), Token::NumU(2), Token::OpCode("RETURN".to_string()) ];
        let program2 = vec![Token::OpCode("PUSH".to_string()), Token::NumI(-21), Token::OpCode("ADDF".to_string()), Token::Func("my_func".to_string()), Token::OpCode("PUSH".to_string()), Token::NumU(2), Token::OpCode("RETURN".to_string()) ];
        
        let output = Parser::parse(program).unwrap();

        assert_eq!(program2, output)
    }

    #[test]
    fn compile_func1() {
        let mut comp = Compiler::new("../nar files/fizzbuzz.nar".to_string());

        let program = "
            PUSH 30
            DUPLI
            CALL fizz
            CALL buzz
            HALT

            fizz:
            PUSH 5
            MODU
            PUSH 0
            CMP 0
            JNE 3
            PUSH 70
            PRINT 3
            RETURN

            buzz:
            PUSH 3
            MODU
            PUSH 0
            CMP 0
            JNE 3
            PUSH 66
            PRINT 3
            RETURN
        ";

        assert_eq!(Ok(()), comp.compile(program, true))
    }

    #[test]
    fn compile_func2() {
        let mut comp = Compiler::new("../nar files/fizzbuzz.nar".to_string());

        let program = "
            PUSH 6
            STORE 0
            CALL fizz
            CALL buzz
            CALL reduce
            DUPLI
            PUSH 0
            CMP 0
            JNE -6
            HALT

            reduce:
            PUSH 1
            SUBU
            RETURN

            fizz:
            LOAD 0
            PUSH 3
            MODU
            PUSH 0
            STORE 1
            LOAD 1
            CMP 0
            

            buzz:
            DUPLI
            PUSH 5
            MODU
            PUSH 0
            CMP 0
            JNE 5
            PUSH 66
            PRINT 3
            PUSH 2000
            JNE 2
            PUSH 1000
            RETURN

            self:
            DUPLI
            PUSH 5
            MODU
            PUSH 0
            CMP 0
            JNE 3
            PUSH 66
            PRINT 3
            RETURN
        ";

        assert_eq!(Ok(()), comp.compile(program, true))
    }
}