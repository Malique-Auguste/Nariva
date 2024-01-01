use std::fmt::Error;

use crate::{error::CompError, token::Token};

pub struct Parser {
    pub program: Vec<Token>,
}

impl Parser {
    pub fn new(input: Vec<Token>) -> Result<Parser, CompError> {
        if input.is_empty() {
            return Err(CompError::UnexpectedEOF("Input is empty".into()));
        }

        Ok(Parser { program })
    }

    pub fn parse(&mut self) -> Result<&Vec<Token>, CompError> {

        loop {
            let mut index = 0;

            if self.program.len() == index {
                break;
            }

            match self.program[index]{
                Token::Word(word) => {
                    match word {
                        "Illegal" | "Halt" | "Pop" | 
                        "AddU" | "SubU" |"MulU" | "DivU" |
                        "AddI" | "SubI" |"MulI" | "DivI" |
                        "AddF" | "SubF" |"MulF" | "DivF" |
                        "Shift" | "BitAnd" | "BitOr" | "BitXor" | "BitNot" => {
                            index += 1
                        }

                        "Push" => {

                            if self.program.len() > index + 2 && self.program[index+1].is_num() && self.program[index+2].is_num() {
                                index += 3
                            }

                            else {
                                return Err(CompError::UnexpectedChar("Two numbers are needed after push command".to_string()))
                            }
                        }
                    }
                }
            }
        }

        return Ok(&self.program)

    }
}