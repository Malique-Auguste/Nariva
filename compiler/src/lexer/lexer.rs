use crate::{error::CompError, lexer::token::{Token, TokenType}};

#[derive(Debug, PartialEq)]
pub struct Lexer {
    pub input: Vec<char>,
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

            if current_char == '\n' {
                println!("new line encountered");
            }
            

            current_token_type = Lexer::match_token_type(&current_char);

            match current_token_type {
                TokenType::NewLine => {
                    self.line += 1;
                    println!("line incremented to {}", self.line);
                },

                TokenType::Tab | TokenType::Return | TokenType::Whitespace => continue,

                TokenType::LBracket | TokenType::RBracket | TokenType::LBrace | TokenType::RBrace | TokenType::LParenth | TokenType::RParenth  => output.push(Token::new(current_token_type, self.line)),

                TokenType::Equal | TokenType::Minus | TokenType::Plus  => output.push(Token::new(current_token_type, self.line)),

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
                            self.index += 1;
                            accumulator.push(current_char);
                        } else {
                            output.push(Token::new(TokenType::Num(match accumulator.parse::<i16>() {
                                Ok(x) => x,
                                Err(_) => return Err(CompError::Overflow(accumulator)),
                            }), self.line ));

                            break;
                        }
                    }
                }

                TokenType::Str(s) => {
                    let is_binding: bool = s != "\"";

                    let mut accumulator = match is_binding {
                        true => s,
                        false => String::new()
                    };

                    loop {
                        current_char = match self.peek() {
                            Some(c) => c,
                            None => {
                                if is_binding {
                                    match Lexer::match_keyword(&accumulator) {
                                        Some(tt) => output.push(Token::new(tt, self.line)),
                                        None => output.push(Token::new(TokenType::Binding(accumulator), self.line))
                                    }
                                    break;
                                }
                                else {
                                    return Err(CompError::UnexpectedEOF(format!("Expected \" to close string, but got: {}", current_char)))
                                }
                            }
                        };

                        if !current_char.is_alphanumeric() && current_char != '_' && is_binding {
                            match Lexer::match_keyword(&accumulator) {
                                Some(tt) => output.push(Token::new(tt, self.line)),
                                None => output.push(Token::new(TokenType::Binding(accumulator), self.line))
                            }
                            break
                        }
                        else if current_char == '\"' && !is_binding  {
                            self.index += 1;
                            output.push(Token::new(TokenType::Str(accumulator), self.line));
                            break
                        }
                        else {
                            self.index += 1;
                            accumulator.push(current_char);
                        }
                    }
                },

                TokenType::Binding(_) => unreachable!(),
                _ => unimplemented!()
            }
        }

        Ok(output)
    }

    pub fn match_token_type(c: &char) -> TokenType {
        if *c == '\n' {
            println!("new line encountered");
        }
        match c {
            '0'..='9' => TokenType::Num(c.to_digit(10).unwrap() as i16),
            'a'..='z' | 'A'..='Z' | '\"' | '_' => TokenType::Str(String::from(*c)),

            '(' => TokenType::LParenth,
            ')' => TokenType::RParenth,
            '[' => TokenType::LBracket,
            ']' => TokenType::RBracket,
            '{' => TokenType::LBrace,
            '}' => TokenType::RBrace,

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

    pub fn match_keyword(s: &String) -> Option<TokenType> {
        match s.as_str() {
            "const" => Some(TokenType::Constant),

            "if" => Some(TokenType::If),
            "else" => Some(TokenType::Else),
            "for" => Some(TokenType::For),
            "while" => Some(TokenType::While),
            "in" => Some(TokenType::In),

            "pub" => Some(TokenType::Public),
            "pro" => Some(TokenType::Protected),
            "priv" => Some(TokenType::Private),

            _ => None
        }
    }
}
