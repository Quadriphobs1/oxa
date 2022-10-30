use crate::token::{Token, TokenKind};
use std::fmt::Debug;

#[derive(Debug, Default)]
pub struct Reporter {}

impl Reporter {
    pub fn line_error(line: usize, message: &str) {
        report_error(line, "", message)
    }

    pub fn token_error(token: &Token, message: &str) {
        if token.kind == TokenKind::Eof {
            println!("{} at end {}", token.line, message);
        } else {
            println!("{} at '{}' {}", token.line, token.lexeme, message);
        }
    }
}

fn report_error(line: usize, location: &str, message: &str) {
    println!("[line {} Error {}: {}", line, location, message);
}
