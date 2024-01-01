extern crate virtual_machine;

use std::fmt::Error;

use crate::{error::CompError, token::Token};
use virtual_machine::{instruction::OpCode, vm::HEADER};

pub struct Generator {
    pub output: Vec<u8>
}

impl Generator {
    pub fn new() -> Generator {
        Generator { output: HEADER.to_vec() }
    }

    pub fn generate(&mut self, input: &Vec<Token>) -> Result<&Vec<u8>, CompError> {
        if input.is_empty() {
            return Err(CompError::UnexpectedEOF("Input is empty".into()));
        }

        let mut index = 0;

        loop {
            if index >= input.len() {
                break
            }
            match &input[index] {
                Token::Word(word) => {
                    self.output.push(OpCode::from(word).into())
                },
                Token::NumU(num) => {
                    let bytes = num.to_be_bytes();
                    self.output.extend_from_slice(&bytes)
                },
                Token::NumI(num) => {
                    let bytes = num.to_be_bytes();
                    self.output.extend_from_slice(&bytes)
                },
                Token::NumF(num) => {
                    let bytes = num.to_be_bytes();
                    self.output.extend_from_slice(&bytes)
                },
            };

            index += 1
        }

        Ok(&self.output)
    }
}