use crate::lexer::token::{Token, TokenType};
use crate::parser::statement::Statement;
use crate::parser::expression::Expression;
use crate::parser::operator::Operator;
use std::convert::TryFrom;

pub struct Parser {
    input: Vec<Vec<Token>>,
    line: usize,

    symbol_table: Vec<Symbol>
}

impl Parser {
    pub fn new(input: Vec<Token>) -> Parser {
        Parser{ input: Parser::split_by_line(input), line: 0, symbol_table: Vec::new() }
    }

    pub fn split_by_line(mut input: Vec<Token>) -> Vec<Vec<Token>> {
        let mut output: Vec<Vec<Token>> = Vec::new();
        let mut index = 0;
        let mut line = 1;

        loop {
            if index >= input.len() {
                output.push(input.drain(0..index).collect());
                break;
            }
            else if input[index].line == line {
                index += 1;
            }
            else {
                output.push(input.drain(0..index).collect());
                index = 0;
                line += 1;
            }
        }

        output
    }

    pub fn get_expression(mut input: Vec<Token>) -> Expression {
        let index = match input.iter().position(|t| t.token_type == TokenType::EqualEqual || t.token_type == TokenType::LesserEqual || t.token_type == TokenType::GreaterEqual || t.token_type == TokenType::BangEqual) {
            Some(i) => i,
            None => match input.iter().position(|t| t.token_type == TokenType::Plus) {
                Some(i) => i,
                None => match input.iter().position(|t| t.token_type == TokenType::Dash || t.token_type == TokenType::Bang) {
                    Some(i) => {
                        if i == 0 && input.len() == 2 {
                            return Expression::Unary{ operator: Operator::try_from(input.remove(0).token_type).unwrap(), inner: Box::new( match input.remove(0).token_type {
                                TokenType::U64(n) => Expression::U64{inner: n},
                                TokenType::F64(n) => Expression::F64{inner: n},
                                TokenType::Str(n) => Expression::Str{inner: n},
                                _ => unimplemented!()
                            })};
                        }
                        else {
                            let new_input: Vec<Token> = input[1..].to_vec();
                            println!("ap{:?}", new_input);
                            match new_input.iter().position(|t| t.token_type == TokenType::Dash || t.token_type == TokenType::Bang) {
                                Some(j) => {
                                    if input[j].token_type == TokenType::Star || input[j].token_type == TokenType::Slash {
                                        match input.iter().position(|t| t.token_type == TokenType::Star || t.token_type == TokenType::Slash) {
                                            Some(i) => i,
                                        
                                            None => match input.remove(0).token_type {
                                                TokenType::U64(n) => return Expression::U64{inner: n},
                                                TokenType::F64(n) => return Expression::F64{inner: n},
                                                TokenType::Str(n) => return Expression::Str{inner: n},
                                                _ => unimplemented!()
                                            }
                                        }
                                    }
                                    else {
                                        j + 1
                                    }
                                }   
                                None => match input.iter().position(|t| t.token_type == TokenType::Star || t.token_type == TokenType::Slash) {
                                    Some(i) => i,
                                
                                    None => match input.remove(0).token_type {
                                        TokenType::U64(n) => return Expression::U64{inner: n},
                                        TokenType::F64(n) => return Expression::F64{inner: n},
                                        TokenType::Str(n) => return Expression::Str{inner: n},
                                        _ => unimplemented!()
                                    }
                                }
                            }
                        }
                    },

                    None => match input.iter().position(|t| t.token_type == TokenType::Star || t.token_type == TokenType::Slash) {
                        Some(i) => i,
                    
                        None => match input.remove(0).token_type {
                            TokenType::U64(n) => return Expression::U64{inner: n},
                            TokenType::F64(n) => return Expression::F64{inner: n},
                            TokenType::Str(n) => return Expression::Str{inner: n},
                            _ => unimplemented!()
                        }
                    }
                }
            }
        };

        let lhs = input.drain(0..index).collect();
        let rhs = input.drain(1..).collect();

        println!("op {:?}, lhs {:?}, rhs {:?}", input[0], lhs, rhs);
        return Expression::Binary{ lhs: Box::new(Parser::get_expression(lhs)), operator: Operator::try_from(input.remove(0).token_type).unwrap(), rhs: Box::new(Parser::get_expression(rhs))}
    }

    pub fn get_value_init_statement(&mut self, mut line: Vec<Token>, scope: &String) -> Result<Statement, String> {
        let val_type = match line.remove(0).token_type {
            TokenType::Binding(s) => s.clone(),
            _ => return Err("The first token is not a binding and thus no type can be determined for the value.".into())
        };

        let binding = match line.remove(0).token_type {
            TokenType::Binding(s) => s.clone(),
            _ => return Err("The second token is not a binding and thus the binding for the value cannot be determined.".into())
        };

        if self.symbol_table.iter().any(|x| x.binding == binding) {
            return Err("A symbol already exists with that binding".into());
        }

        let expression =  Parser::get_expression(line);

        self.symbol_table.push(Symbol::new(binding, val_type));
        unimplemented!();
    }

    pub fn parse(&mut self) -> Vec<Statement> {
        let mut output: Vec<Statement> = Vec::new();
        let mut line: Vec<Token> = Vec::new();
        let mut index = 0;

        loop {
            
        }

        output
    }
}

struct Symbol {
    binding: String,
    symbol_type: String,
    //scope: String,
}

impl Symbol {
    pub fn new(binding: String,  symbol_type: String) -> Symbol {
        Symbol { binding, symbol_type }
    }
}