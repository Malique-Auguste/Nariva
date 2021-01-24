use crate::lexer::token::Token;

pub struct Parser {
    input: Vec<Vec<Token>>,
    line: usize
}

impl Parser {
    pub fn new(input: Vec<Token>) -> Parser {
        Parser{ input: Parser::split_by_line(input), line: 0 }
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

    pub fn parse() -> Vec<u8> {
        unimplemented!()
    }
}