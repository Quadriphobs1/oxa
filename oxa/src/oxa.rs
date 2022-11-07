use crate::errors::ErrorCode;
use crate::interpreter::Interpreter;
use crate::parser::Parser;
use crate::scanner::Scanner;
use std::fs;

#[derive(Debug, Default)]
pub struct Oxa {
    // TODO: Handle error handle properly with Reporter
    pub error: bool,
    pub runtime_error: bool,
    interpreter: Interpreter,
}

impl Oxa {
    pub fn new() -> Self {
        Oxa::default()
    }
}

/// public methods
impl Oxa {
    pub fn run_file(&mut self, file_path: &str) -> Result<(), ErrorCode> {
        log::info!("Loading file information");
        let file = fs::read_to_string(file_path);
        match file {
            Ok(result) => {
                self.run(&result)?;
                Ok(())
            }
            Err(e) => {
                log::error!("Unable to read file");
                Err(ErrorCode::IO(e))
            }
        }
    }

    pub fn run_prompt(&mut self) -> Result<(), ErrorCode> {
        log::info!("Reading input from prompt");
        let mut input = String::new();
        match std::io::stdin().read_line(&mut input) {
            Ok(_) => {
                self.run(&input)?;
                Ok(())
            }
            Err(e) => {
                log::error!("Unable to get user input from the cli");
                Err(ErrorCode::IO(e))
            }
        }
    }
}

/// private methods
impl Oxa {
    fn run(&self, s: &str) -> Result<(), ErrorCode> {
        let mut scanner = Scanner::from_source(s);

        let tokens = scanner.scan_tokens()?;
        println!("{:?}", tokens);
        let mut parser = Parser::from_tokens(&tokens);
        let expression = parser.parse()?;
        let result = self.interpreter.interpret(expression.as_ref())?;

        println!("{:?}", result);
        Ok(())
    }
}
