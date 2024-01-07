use core::num;
use std::{fmt::{Error, format}, collections::HashMap, convert::TryInto, thread::current};

use virtual_machine::{vm::HEADER, instruction::OpCode};

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
                    binary_index += 7
                },

                Token::OpCode(s) => {
                    if s == "Call" || s == "CALL" {
                        binary_index += 7
                    }
                },


                _ => ()
            };

            program_index += 1;
            binary_index += 1;
        }

        //panic!(format!("{:?}", function_record));

        let mut index = 0;
        //general loop
        loop {

            if program.len() == index {
                break;
            }

            match &program[index]{
                Token::OpCode(word) => {
                    match word.as_str() {
                        "Push" | "PUSH" | "CMP" | "PRINT" | "Print" | "Load" | "LOAD" | "Store" | "STORE" | "PrintSTR" | "PRINTSTR"=> {

                            if program.len() > index + 1 && program[index+1].is_num()  {
                                index += 2
                            }

                            else {
                                return Err(CompError::UnexpectedChar(format!("Number needed after a '{}' opcode", word)))
                            }
                        },

                        "JE" | "JNE" | "JG" | "JL" | "JMP"=> {

                            if program.len() > index + 1 && program[index+1].is_num()  {
                                match program[index + 1] {
                                    Token::NumU(num) => {
                                        let num_clone = i64::from_be_bytes(num.to_be_bytes());
                                        program[index + 1] = Token::NumU(Parser::get_jump_index(num_clone, &program, index).unwrap())
                                    },
                                    Token::NumI(num) => {
                                        let num_clone = i64::from_be_bytes(num.to_be_bytes());
                                        program[index + 1] = Token::NumU(Parser::get_jump_index(num_clone, &program, index).unwrap())
                                    }
                                    _ => return Err(CompError::UnexpectedChar(format!("unsigned Number needed after a '{}' opcode", word)))

                                }
                                index += 2

                            }

                            else {
                                return Err(CompError::UnexpectedChar(format!("unsigned Number needed after a '{}' opcode", word)))
                            }
                        },

                        
                        "Call" | "CALL" => {
                            if program.len() > index + 1 {
                                index += 1;
                                match &program[index] {
                                    Token::OpCode(name) => {
                                        match function_record.get(name) {
                                            Some(func_index) => {
                                                //This gives us the location right before the first line of the function
                                                //When vm runs it executes the instruction after this and ths executes the first part of the function
                                                program[index] = Token::NumU(*func_index as u64 + HEADER.len() as u64 - 1);
                                                index += 1;
                                            },
                                            None => return Err(CompError::UnexpectedChar(format!("Function '{}' doesnt exist", name)))
                                        }
                                    },

                                    _ => return Err(CompError::UnexpectedChar(format!("Function needed after a '{}' opcode", word)))

                                }
                            }

                            else {
                                return Err(CompError::UnexpectedChar(format!("Function needed after a '{}' opcode", word)))
                            }
                        }

                        _ => {
                            match word.into() {
                                OpCode::Illegal => return Err(CompError::UnexpectedChar(format!("'{}' opcode doesn't exist", word))),
                                _ => index += 1
                            }
                        }
                    }
                },

                Token::Func(_) => unreachable!(),

                Token::NumU(_) | Token::NumI(_) | Token::NumF(_) => return Err(CompError::UnexpectedChar("Numbers must only proceed words".to_string()))

            }
        }

        return Ok(program)

    }

    fn get_jump_index(desired_num_opcodes: i64, program: &Vec<Token>, mut current_index: usize) -> Result<u64, CompError> {
        let forward_jump = desired_num_opcodes > 0;
        
        let desired_num_opcodes = desired_num_opcodes.abs();
        let mut current_num_opcodes = 0;
        let mut binary_index:i64 = 0;
        //println!("\nDesires opcodes: {}", desired_num_opcodes);
        //println!("Current I: {:?}, Bin: {}, PrADD: {}", program[current_index], binary_index, current_index);

        if desired_num_opcodes == 0 {
            return  Err(CompError::UnexpectedChar("Desried opcodes cannot be zero".into()));
        }
        
        loop {
            if current_num_opcodes >= desired_num_opcodes {
                break;
            }

            if forward_jump {
                current_index += 1;
                binary_index += 1;
            }
            else {
                
                current_index -= 1;
                binary_index -= 1;
            }

            if current_index >= program.len() {
                return Err(CompError::UnexpectedEOF("Jump to distance greater than file".to_string()))
            }
            /*
            Not possible
            else if current_index < 0 {
                return Err(CompError::UnexpectedEOF("Jump to distance too far back".to_string()))
            }
            */

            match program[current_index] {
                Token::OpCode(_) => current_num_opcodes += 1,
                Token::NumF(_) | Token::NumI(_) | Token::NumU(_) => {
                    if forward_jump {
                        binary_index += 7;
                    }
                    else {
                        binary_index -= 7;
                    }
                },
                _ => continue
            }

            //println!("Current I: {:?}, Bin: {}, PrADD: {}", program[current_index], binary_index, current_index);

        }

        Ok(u64::from_be_bytes(binary_index.to_be_bytes()))
    }
}