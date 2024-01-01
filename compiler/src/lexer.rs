use std::fmt::Error;

use crate::{error::CompError, token::Token};

pub struct Lexer {
    pub output: Vec<Token>
}

impl Lexer {
    pub fn new() -> Lexer{

        Lexer { output: Vec::new()}
    }

    pub fn lex<S: Into<String>>(&mut self, input: S ) -> Result<&Vec<Token>, CompError> {
        let input: Vec<char> = input.into().chars().collect();
        let mut index = 0;

        if input.is_empty() {
            return Err(CompError::UnexpectedEOF("Input is empty".into()));
        }

        loop {
            if input.len() <= index {
                break;
            }

            self.output.push(  
                match input[index]{
                    'a'..='z' | 'A'..='Z' => self.get_word(&input, &mut index),

                    '0'..='9' | '+' | '-' => self.get_num(&input, &mut index),

                    ' ' | '\t' | '\n' | _ => {
                        index += 1;
                        continue
                    },
                }
            )
        };

        return Ok(&self.output)

    }

    fn get_word(&self, input: &Vec<char>, index: &mut usize) -> Token {
        let mut word = String::from(input[*index]);
        *index += 1;

        loop {
            if *index == input.len() {
                return Token::Word(word)
            }

            match input[*index] {
                ' ' | '\t' | '\n' => {
                    *index += 1;
                    return Token::Word(word)
                },
                _ => {
                    word.push(input[*index]);
                    *index += 1;
                }
           }
        }
    }

    pub fn get_num(&self, input: &Vec<char>, index: &mut usize) -> Token {
        let mut num = String::from(input[*index]);
        *index += 1;
        
        loop {
            if *index == input.len() {
                break
            }

            match input[*index] {
                '0'..='9' | '.' | '_' => {
                    num.push(input[*index]);
                    *index += 1;
                },
                _ => {
                    *index += 1;
                    break
                },
           }
        }

        match num.parse::<u64>() {
            Ok(n) => return Token::NumU(n),
            Err(_) => ()
        };

        match num.parse::<i64>() {
            Ok(n) => return Token::NumI(n),
            Err(_) => ()
        };

        match num.parse::<f64>() {
            Ok(n) => return Token::NumF(n),
            Err(_) => ()
        };

        unreachable!()

    }

}