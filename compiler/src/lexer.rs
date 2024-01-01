use std::{fmt::Error, process::Output};

use crate::{error::CompError, token::Token};

//converts human readable text into tokens
pub struct Lexer;

impl Lexer {
    pub fn new() -> Lexer{

        Lexer
    }

    pub fn lex<S: Into<String>>( input: S ) -> Result<Vec<Token>, CompError> {
        let input: Vec<char> = input.into().chars().collect();
        let mut output = Vec::new();
        let mut index = 0;

        if input.is_empty() {
            return Err(CompError::UnexpectedEOF("Input is empty".into()));
        }

        loop {
            if input.len() <= index {
                break;
            }

            output.push(  
                match input[index]{
                    'a'..='z' | 'A'..='Z' => Lexer::get_word(&input, &mut index),

                    '0'..='9' | '+' | '-' => Lexer::get_num(&input, &mut index),

                    ' ' | '\t' | '\n' | _ => {
                        index += 1;
                        continue
                    },
                }
            )
        };

        return Ok(output)

    }

    fn get_word(input: &Vec<char>, index: &mut usize) -> Token {
        let mut word = String::from(input[*index]);
        *index += 1;

        loop {
            if *index == input.len() {
                return Token::OpCode(word)
            }

            match input[*index] {
                ' ' | '\t' | '\n' => {
                    *index += 1;
                    return Token::OpCode(word)
                },
                ':' => {
                    *index += 1;
                    return Token::Func(word)
                }
                _ => {
                    word.push(input[*index]);
                    *index += 1;
                }
           }
        }
    }

    pub fn get_num(input: &Vec<char>, index: &mut usize) -> Token {
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