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

    //function that compiles a Nariva program into binary.
    pub fn compile<S: Into<String>>(&mut self, input: S, show: bool) -> Result<(), CompError> {
        //These represent separated "chunks" of data from the program.
        let unparsed_tokens = Lexer::lex(input).unwrap();
        if show {
            println!("UT: {:?}\n", unparsed_tokens);
        }

        //checks to make sure that tokens obey certain grammatical rules
        let parsed_tokens = Parser::parse(unparsed_tokens).unwrap();
        if show {
            println!("\nPT: {:?}", parsed_tokens);
        }

        //converts parsed tokens into binary data
        let binary = Generator::generate(parsed_tokens).unwrap();
        if show {
            println!("\nBin: {:?}\n", binary);
        }

        //writes data to file
        let mut file = fs::File::create(&self.file_path).unwrap();

        file.write_all(&binary).unwrap();
        Ok(())
    }
}