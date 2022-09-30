use crate::reporter::Reporter;
use crate::token::{Literal, Token, TokenKind};
use crate::ErrorCode;
use std::collections::LinkedList;

/// A code scanner using lexical grammar to tokens
#[derive(Debug, Default)]
pub struct Scanner {
    source: String,
    tokens: LinkedList<Token>,
    start: usize,
    current: usize,
    line: i32,
}

/// Constructor implementation
impl Scanner {
    /// Creates default scanner with empty string
    pub fn new() -> Self {
        Scanner {
            source: "".to_string(),
            ..Self::default()
        }
    }

    /// Creates a scanner from a string source and empty tokens
    pub fn from_source(source: &str) -> Self {
        Scanner {
            source: source.to_string(),
            line: 1,
            ..Self::default()
        }
    }
}

/// Public method implementation
impl Scanner {
    pub fn scan_tokens(&mut self) -> Result<&LinkedList<Token>, ErrorCode> {
        log::info!("Converting source to token");
        while !self.is_at_end() {
            // Start from the beginning of the next lexeme
            self.start = self.current;
            self.scan_token()?;
        }

        self.tokens
            .push_back(Token::new(TokenKind::Eof, "", None, self.line));
        Ok(&self.tokens)
    }
}

/// Internal method implementation
impl Scanner {
    fn scan_token(&mut self) -> Result<(), ErrorCode> {
        match self.advance() {
            Some(c) => {
                if self.scan_comparator_char_token(c)
                    || self.scan_comment_char_token(c)
                    || self.scan_single_char_token(c)
                    || self.scan_ignored_char(c)
                {
                    // Do nothing if the operation succeeds
                } else {
                    log::warn!("Unable to complete single character scan");
                    Reporter::error(self.line, "Unexpected character: expected single char");
                }
                Ok(())
            }
            None => {
                log::warn!("Unable to process any more token");
                Err(ErrorCode::ProcessError) // TODO: Change the error code
            }
        }
    }

    fn scan_comparator_char_token(&mut self, c: char) -> bool {
        let next_match_equal = self.next_match_char('=');
        match c {
            '!' => self.add_token(
                if next_match_equal {
                    TokenKind::BangEqual
                } else {
                    TokenKind::Bang
                },
                None,
            ),
            '=' => self.add_token(
                if next_match_equal {
                    TokenKind::EqualEqual
                } else {
                    TokenKind::Equal
                },
                None,
            ),
            '<' => self.add_token(
                if next_match_equal {
                    TokenKind::LessEqual
                } else {
                    TokenKind::Less
                },
                None,
            ),
            '>' => self.add_token(
                if next_match_equal {
                    TokenKind::GreaterEqual
                } else {
                    TokenKind::Greater
                },
                None,
            ),
            _ => {
                return false;
            }
        }

        return true;
    }

    fn scan_comment_char_token(&mut self, c: char) -> bool {
        if c != '/' {
            return false;
        }

        match self.next_match_char('/') {
            true => {
                // A comment goes until the end of the line.
                while !self.is_at_end() {
                    match self.peek() {
                        Some(c) => {
                            if c != '\n' {
                                self.advance();
                            }
                        }
                        None => continue,
                    }
                }
                return true;
            }
            false => false,
        }
    }

    fn scan_single_char_token(&mut self, c: char) -> bool {
        match c {
            '(' => self.add_token(TokenKind::LeftParen, None),
            ')' => self.add_token(TokenKind::RightParen, None),
            '{' => self.add_token(TokenKind::LeftBrace, None),
            '}' => self.add_token(TokenKind::RightBrace, None),
            ',' => self.add_token(TokenKind::Comma, None),
            '.' => self.add_token(TokenKind::Dot, None),
            '-' => self.add_token(TokenKind::Minus, None),
            '+' => self.add_token(TokenKind::Plus, None),
            '/' => self.add_token(TokenKind::Slash, None), // TODO: Potentially be removed
            '*' => self.add_token(TokenKind::Star, None),
            ';' => self.add_token(TokenKind::SemiColon, None),
            _ => {
                return false;
            }
        }

        return true;
    }

    fn scan_ignored_char(&mut self, c: char) -> bool {
        match c {
            ' ' | '\r' | '\t' => true,
            '\n' => {
                self.line += 1;
                return true;
            }
            _ => false,
        }
    }

    fn add_token(&mut self, kind: TokenKind, literal: Option<Literal>) {
        // TODO: Try a less error prone approach to select the string slice
        // e.g collection with error validation for range
        let lexeme = &self.source[self.start..self.current];

        let token = Token::new(kind, lexeme, literal, self.line);

        self.tokens.push_back(token);
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> Option<char> {
        self.increment_current();

        // nth is zero-index based
        return self.source.chars().nth(self.current - 1);
    }

    fn peek(&self) -> Option<char> {
        if self.is_at_end() {
            return Some('\0');
        }
        return self.source.chars().nth(self.current);
    }

    fn next_match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        match self.source.chars().nth(self.current) {
            None => false,
            Some(c) => {
                if c != expected {
                    return false;
                }

                self.increment_current();
                return true;
            }
        }
    }

    fn increment_current(&mut self) {
        self.current += 1;
    }
}

#[cfg(test)]
mod scanner_tests {
    use super::*;

    #[test]
    fn test_no_token_with_initial_creation() {
        let scanner = Scanner::from_source(&String::new());
        assert_eq!(scanner.tokens.len(), 0);
    }
    #[test]
    fn test_generates_eof_token_at_default() {
        let mut scanner = Scanner::from_source(&String::new());
        scanner.scan_tokens().unwrap();

        assert_eq!(scanner.tokens.len(), 1);
    }

    #[test]
    fn test_generates_token_for_single_char() {
        let mut scanner = Scanner::from_source("(");
        scanner.scan_tokens().unwrap();
        assert_eq!(scanner.tokens.len(), 2);
    }

    #[test]
    fn test_generates_token_for_multiple_single_char() {
        let mut scanner = Scanner::from_source("(){},.-+/*;");
        scanner.scan_tokens().unwrap();
        assert_eq!(scanner.tokens.len(), 12);
    }

    #[test]
    fn test_generates_token_for_single_comparator() {
        let mut scanner = Scanner::from_source("=<>!");
        scanner.scan_tokens().unwrap();
        assert_eq!(scanner.tokens.len(), 5);
    }

    #[test]

    fn test_generates_token_for_multi_comparator_arms() {
        let mut scanner = Scanner::from_source("<=>=!===");
        scanner.scan_tokens().unwrap();
        assert_eq!(scanner.tokens.len(), 5);
    }
}
