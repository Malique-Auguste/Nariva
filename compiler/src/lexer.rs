use std::fmt::Error;

use crate::{error::CompError, token::Token};

pub struct Lexer {
    pub input: Vec<char>,
    pub output: Vec<Token>
}

impl Lexer {
    pub fn new<S: Into<String>>(input: S) -> Result<Lexer, CompError> {
        let input: Vec<char> = input.into().chars().collect();

        if input.is_empty() {
            return Err(CompError::UnexpectedEOF("Input is empty".into()));
        }

        Ok(Lexer { input, output: Vec::new() })
    }

    pub fn lex(&mut self) -> Result<Vec<Token>, CompError> {
        let mut current_char: char = self.input[0];

        loop {
            self.output.push(  
                match current_char{
                    'a'..='z' | 'A'..='Z' => {
                        let (word, index) = self.get_word();

                        self.input.drain(0..index);
                        word
                    },

                    '0'..='9' | '+' | '-' => {
                        match self.get_num() {
                            Ok((num, index)) => {
                                self.input.drain(0..index);
                                num
                            },

                            Err(e) => return Err(e)
                        }
                    },

                    ' ' | '\t' | '\n' => {
                        self.input.remove(0);
                        continue
                    },
                }
            )
        }

    }

    pub fn get_word(&self) -> (Token, usize) {
        let mut word = String::from(self.input[0]);
        let mut index = 1;

        loop {
            match self.input[index] {
                ' ' | '\t' | '\n' => {
                    index += 1;
                    return (Token::Word(word), index)
                },
                _ => {
                    word.push(self.input[index]);
                    index += 1;
                }
           }
        }
    }

    pub fn get_num(&self) -> Result<(Token, usize), CompError> {
        let mut num = String::from(self.input[0]);
        let mut index = 1;

        loop {
            match self.input[index] {
                '0'..='9' | '.'  => {
                    num.push(self.input[index]);
                    index += 1;
                },
                _ => {
                    index += 1;
                    break
                },
           }
        }

        match num.parse::<u64>() {
            Ok(n) => return Ok((Token::NumU(n), index)),
            Err(_) => ()
        };

        match num.parse::<i64>() {
            Ok(n) => return Ok((Token::NumI(n), index)),
            Err(_) => ()
        };

        match num.parse::<f64>() {
            Ok(n) => return Ok((Token::NumF(n), index)),
            Err(_) => ()
        };

        Err(CompError::Impossible("Should not be able to reach here".to_string()))

    }

}