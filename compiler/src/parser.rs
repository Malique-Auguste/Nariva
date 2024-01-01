use std::fmt::Error;

use crate::{error::CompError, token::Token};

pub struct Parser {
    pub program: Vec<Token>,
}

impl Parser {
    pub fn new(program: Vec<Token>) -> Result<Parser, CompError> {
        if program.is_empty() {
            return Err(CompError::UnexpectedEOF("Input program is empty".into()));
        }

        Ok(Parser { program })
    }

    pub fn parse(&mut self) -> Result<&Vec<Token>, CompError> {
        let mut index = 0;

        loop {

            if self.program.len() == index {
                break;
            }

            match &self.program[index]{
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

                            if self.program.len() > index + 1 && self.program[index+1].is_num()  {
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

        return Ok(&self.program)

    }
}