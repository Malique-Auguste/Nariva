use std::{fmt::{Error, format}, collections::HashMap};

use virtual_machine::vm::HEADER;

use crate::{error::CompError, token::Token};

//ensures that tokens are arranged in a certain order / obey certain rules
pub struct Parser;

impl Parser {
    pub fn new() -> Parser {
        Parser
    }

    pub fn parse(mut program: Vec<Token>) -> Result<Vec<Token>, CompError> {
        if program.is_empty() {
            return Err(CompError::UnexpectedEOF("Input program is empty".into()));
        }

        let mut function_record: HashMap<String, usize> = HashMap::new();
        
        let mut program_index = 0;
        let mut binary_index = 0;

        //loop to generate function indices
        loop {
            if program.len() <= program_index {
                break
            }

            match &program[program_index] {
                Token::Func(name) => {
                    function_record.insert(name.clone(), binary_index);
                    program.remove(program_index);
                },

                Token::NumU(_) | Token::NumI(_) | Token::NumF(_) => {
                    if program[program_index - 1].is_push() || program[program_index - 1].is_conditional_jmp() {
                        binary_index += 7
                    }
                }


                _ => ()
            };

            program_index += 1;
            binary_index += 1;
        }

        let mut index = 0;
        //general loop
        loop {

            if program.len() == index {
                break;
            }

            println!("{:?}", program[index]);

            match &program[index]{
                Token::OpCode(word) => {
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
                        "SHIFT" | "BITAND" | "BITOR" | "BITXOR" | "BITNOT" |
                        
                        "CMP" | "JMP" | "Return" | "RETURN" |
                        
                        "ModU" | "ModI" | "ModF" |
                        "MODU" | "MODI" | "MODF" => {
                            index += 1
                        },

                        "Push" | "PUSH" | "JE" | "JNE" | "JG" | "JL" | "PRINT" | "Print"=> {

                            if program.len() > index + 1 && program[index+1].is_num()  {
                                index += 2
                            }

                            else {
                                return Err(CompError::UnexpectedChar(format!("Number needed after a '{:?}' opcode", word)))
                            }
                        },

                        
                        "Call" | "CALL" => {
                            if program.len() > index + 1 {
                                index += 1;
                                match &program[index] {
                                    Token::OpCode(name) => {
                                        match function_record.get(name) {
                                            Some(func_index) => {
                                                program[index] = Token::NumU(*func_index as u64 + HEADER.len() as u64);
                                                index += 1;
                                            },
                                            None => return Err(CompError::UnexpectedChar(format!("Function '{:?}' doesnt exist", name)))
                                        }
                                    },

                                    _ => return Err(CompError::UnexpectedChar(format!("Function needed after a '{:?}' opcode", word)))

                                }
                            }

                            else {
                                return Err(CompError::UnexpectedChar(format!("Function needed after a '{:?}' opcode", word)))
                            }
                        }

                        _ => return Err(CompError::UnexpectedChar(format!("'{:?}' opcode doesn't exist", word)))
                    }
                },

                Token::Func(_) => unreachable!(),

                Token::NumU(_) | Token::NumI(_) | Token::NumF(_) => return Err(CompError::UnexpectedChar("Numbers must only proceed words".to_string()))

            }
        }

        return Ok(program)

    }
}