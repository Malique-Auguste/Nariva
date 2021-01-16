use crate::{error::CompError, lexer::token::{Token, TokenType}};

#[derive(Debug, PartialEq)]
pub struct Lexer {
    input: Vec<char>,
    index: usize,
    line: usize
}

impl Lexer {
    pub fn new<S: Into<String>>(input: S) -> Result<Lexer, CompError> {
        let input: Vec<char> = input.into().chars().collect();

        if input.is_empty() {
            return Err(CompError::UnexpectedEOF("Input is empty".into()));
        }

        Ok(Lexer { input, index: 0, line: 1 })
    }

    pub fn advance(&mut self) -> Option<char> {
        if self.index < self.input.len() {
            self.index += 1;
            Some(self.input[self.index - 1])
        } else {
            None
        }
    }

    pub fn peek(&self) -> Option<char> {
        if self.index < self.input.len() {
            Some(self.input[self.index])
        } else {
            None
        }
    }

    pub fn lex(&mut self) -> Result<Vec<Token>, CompError> {
        let mut output: Vec<Token> = Vec::new();
        let mut current_char: char;
        let mut current_token_type: TokenType;

        loop {
            current_char = match self.advance() {
                Some(c) => c,
                None => break,
            };

            current_token_type = Lexer::match_token_type(&current_char);

            match current_token_type {
                TokenType::NewLine => self.line += 1,

                TokenType::Tab | TokenType::Return | TokenType::Whitespace => continue,

                TokenType::Equal | TokenType::Minus | TokenType::Plus => output.push(Token::new(current_token_type, self.line)),

                TokenType::Num(n) => {
                    let mut accumulator = n.to_string();

                    loop {
                        current_char = match self.peek() {
                            Some(c) => c,
                            None => {
                                output.push(Token::new(TokenType::Num(match accumulator.parse::<i16>() {
                                    Ok(x) => x,
                                    Err(_) => return Err(CompError::Overflow(accumulator)),
                                }), self.line ));

                                break;
                            }
                        };

                        if current_char.is_digit(10) {
                            accumulator.push(current_char);
                        } else {
                            output.push(Token::new(TokenType::Num(match accumulator.parse::<i16>() {
                                Ok(x) => x,
                                Err(_) => return Err(CompError::Overflow(accumulator)),
                            }), self.line ));

                            break;
                        }

                        self.index += 1;
                    }
                }
            }
        }

        Ok(output)
    }

    pub fn match_token_type(c: &char) -> TokenType {
        match c {
            '0'..='9' => TokenType::Num(c.to_digit(10).unwrap() as i16),

            '=' => TokenType::Equal,
            '+' => TokenType::Plus,
            '-' => TokenType::Minus,

            ' ' => TokenType::Whitespace,
            '\n' => TokenType::NewLine,
            '\t' => TokenType::Tab,
            '\r' => TokenType::Return,

            _ => unimplemented!(),
        }
    }
}
