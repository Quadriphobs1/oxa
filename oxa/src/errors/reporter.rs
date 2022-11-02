use crate::errors::ErrorCode;
use crate::token::{Token, TokenKind};
use std::fmt::Debug;

// TODO: All reported error should be collected somewhere to log at once
#[derive(Debug, Default)]
pub struct Reporter {}

impl Reporter {
    pub fn line_error(line: usize, message: &str) {
        println!("[line {} Error : {}", line, message);
    }

    pub fn token_error(token: &Token, message: &str) {
        if token.kind == TokenKind::Eof {
            println!("{} at end {}", token.line, message);
        } else {
            println!("{} at '{}' {}", token.line, token.lexeme, message);
        }
    }

    pub fn arithmetic_error(ops: &str) {
        println!("cannot perform arithmetic operation: {}", ops);
    }

    pub fn runtime_error(error: &ErrorCode) {
        println!("Runtime error: {}", error);
    }
}
