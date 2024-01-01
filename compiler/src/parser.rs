use std::fmt::Error;

use crate::{error::CompError, token::Token};

pub struct Parser;

impl Parser {
    pub fn new() -> Parser {
        Parser
    }

    pub fn parse(program: &Vec<Token>) -> Result<&Vec<Token>, CompError> {
        if program.is_empty() {
            return Err(CompError::UnexpectedEOF("Input program is empty".into()));
        }
        
        let mut index = 0;

        loop {

            if program.len() == index {
                break;
            }

            match &program[index]{
                Token::Word(word) => {
                    match word.as_str() {
                        "Illegal" | "Halt" | "Pop" | 
                        "AddU" | "SubU" |"MulU" | "DivU" |
                        "AddI" | "SubI" |"MulI" | "DivI" |
                        "AddF" | "SubF" |"MulF" | "DivF" |
                        "Shift" | "BitAnd" | "BitOr" | "BitXor" | "BitNot" |
                        
                        "ILLEGAL" | "HALT" | "POP" | 
                        "ADDU" | "SUBU" |"MULU" | "DIVU" |
                        "ADDI" | "SUBI" |"MULI" | "DIVI" |
                        "ADDF" | "SUBF" |"MULF" | "DIVF" |
                        "SHIFT" | "BITAND" | "BITOR" | "BITXOR" | "BITNOT" => {
                            index += 1
                        },

                        "Push" | "PUSH" => {

                            if program.len() > index + 1 && program[index+1].is_num()  {
                                index += 2
                            }

                            else {
                                return Err(CompError::UnexpectedChar("Numbers needed after push command".to_string()))
                            }
                        }

                        _ => return Err(CompError::UnexpectedChar("No other opcodes exist".to_string()))
                    }
                },

                Token::NumU(_) | Token::NumI(_) | Token::NumF(_) => return Err(CompError::UnexpectedChar("Numbers must only proceed words".to_string()))

            }
        }

        return Ok(program)

    }
}