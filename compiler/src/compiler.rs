use crate::{error::CompError, lexer::Lexer, parser::Parser, generator::Generator};
use std::fs;
use std::io::Write;

//overarching structure that converts human readable text to machine readable code
pub struct Compiler {
    file_path: String
}

impl Compiler {
    pub fn new (file_path: String ) -> Compiler {
        Compiler { file_path}
    }

    pub fn compile<S: Into<String>>(&mut self, input: S) -> Result<(), CompError> {
        let unparsed_tokens = Lexer::lex(input).unwrap();
        let parsed_tokens = Parser::parse(unparsed_tokens).unwrap();
        let binary = Generator::generate(parsed_tokens).unwrap();

        let mut file = fs::File::create(&self.file_path).unwrap();

        file.write_all(&binary).unwrap();
        Ok(())
    }
}