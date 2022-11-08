use crate::errors::ErrorCode;
use crate::interpreter::{Interpreter, InterpreterBuilder};
use crate::parser::Parser;
use crate::scanner::ScannerBuilder;
use std::cell::RefCell;

use std::fs;
use std::rc::Rc;

pub struct OxaBuilder {
    interpreter: Rc<RefCell<Interpreter>>,
}

impl Default for OxaBuilder {
    fn default() -> Self {
        OxaBuilder {
            interpreter: Rc::new(RefCell::new(InterpreterBuilder::new().build())),
        }
    }
}

impl OxaBuilder {
    pub fn interpreter(mut self, interpreter: Rc<RefCell<Interpreter>>) -> Self {
        self.interpreter = interpreter;
        self
    }

    pub fn build(self) -> Oxa {
        Oxa::new(self.interpreter)
    }
}

pub struct Oxa {
    // TODO: Handle error handle properly with Reporter
    pub error: bool,
    pub runtime_error: bool,
    interpreter: Rc<RefCell<Interpreter>>,
}

impl Oxa {
    fn new(interpreter: Rc<RefCell<Interpreter>>) -> Self {
        Oxa {
            error: false,
            runtime_error: false,
            interpreter,
        }
    }

    pub fn builder() -> OxaBuilder {
        OxaBuilder::default()
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
        let mut scanner = ScannerBuilder::default().source(s).build();

        let tokens = scanner.scan_tokens()?;
        let mut parser = Parser::from_tokens(&tokens);
        let expression = parser.parse()?;
        let result = self
            .interpreter
            .borrow_mut()
            .interpret(expression.as_ref())?;

        println!("{:?}", result);
        Ok(())
    }
}
