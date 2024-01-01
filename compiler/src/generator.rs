extern crate virtual_machine;

use std::{fmt::Error, process::Output};

use crate::{error::CompError, token::Token};
use virtual_machine::{instruction::OpCode, vm::HEADER};

//converts tokens into code
pub struct Generator;

impl Generator {
    pub fn new() -> Generator {
        Generator 
    }

    pub fn generate(input: Vec<Token>) -> Result<Vec<u8>, CompError> {
        if input.is_empty() {
            return Err(CompError::UnexpectedEOF("Input is empty".into()));
        }

        let mut output = HEADER.to_vec();

        let mut index = 0;

        loop {
            if index >= input.len() {
                break
            }
            match &input[index] {
                Token::OpCode(word) => {
                    output.push(OpCode::from(word).into())
                },
                Token::Func(f) => unimplemented!(),
                Token::NumU(num) => {
                    let bytes = num.to_be_bytes();
                    output.extend_from_slice(&bytes)
                },
                Token::NumI(num) => {
                    let bytes = num.to_be_bytes();
                    output.extend_from_slice(&bytes)
                },
                Token::NumF(num) => {
                    let bytes = num.to_be_bytes();
                    output.extend_from_slice(&bytes)
                },
            };

            index += 1
        }

        Ok(output)
    }
}