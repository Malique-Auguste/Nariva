use crate::{error::CompError, lexer::Lexer, parser::Parser, generator::Generator};
use std::fs;
use std::io::Write;

pub struct Compiler {
    lexer: Lexer,
    parser: Parser,
    generator: Generator,
    file_path: String
}

impl Compiler {
    pub fn new (file_path: String ) -> Compiler {
        Compiler { lexer: Lexer::new(), parser: Parser::new(), generator: Generator::new(), file_path}
    }

    pub fn compile<S: Into<String>>(&mut self, input: S) -> Result<(), CompError> {

        Parser::parse(&self.lexer.output).unwrap();

        let mut file = fs::File::create(&self.file_path).unwrap();

        file.write_all(&self.generator.output).unwrap();
        Ok(())
    }
}