extern crate virtual_machine;

use std::fmt::Error;

use crate::{error::CompError, token::Token};
use virtual_machine::instruction::OpCode;

pub struct Generator {
    pub input: Vec<Token>,
    pub output: Vec<u8>
}

impl Generator {
    pub fn new(input: Vec<Token>) -> Result<Generator, CompError> {
        if input.is_empty() {
            return Err(CompError::UnexpectedEOF("Input  is empty".into()));
        }

        Ok(Generator { input, output: Vec::new() })
    }

    pub fn generate(&mut self) -> Result<&Vec<u8>, CompError> {
        let mut index = 0;

        loop {
            if index >= self.input.len() {
                break
            }
            match &self.input[index] {
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