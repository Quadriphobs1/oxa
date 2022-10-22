use crate::error::ErrorCode;
use crate::scanner::Scanner;
use std::fs;

#[derive(Debug, Default)]
pub struct Oxa {
    error: bool,
}

impl Oxa {
    pub fn new() -> Self {
        Oxa { error: false }
    }
}

impl Oxa {
    pub fn run_file(&mut self, file_path: &str) -> Result<(), ErrorCode> {
        log::info!("Loading file information");
        let file = fs::read_to_string(file_path);
        match file {
            Ok(result) => {
                run(&result)?;
                if self.error {
                    log::error!("Error while processing file");
                    return Err(ErrorCode::ProcessError);
                }
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
                run(&input)?;
                self.error = false;
                Ok(())
            }
            Err(e) => {
                log::error!("Unable to get user input from the cli");
                Err(ErrorCode::IO(e))
            }
        }
    }
}

pub fn run(s: &str) -> Result<(), ErrorCode> {
    let mut scanner = Scanner::from_source(s);

    for token in scanner.scan_tokens()? {
        println!("{:?}", token);
    }
    Ok(())
}
