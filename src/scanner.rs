use crate::error::ErrorCode;
use crate::reporter::Reporter;
use crate::token::{Literal, Token, TokenKind, KEYWORDS};

use std::{collections::LinkedList, str::FromStr};

/// A code scanner using lexical grammar to tokens
#[derive(Debug, Default)]
pub struct Scanner {
    source: String,
    tokens: LinkedList<Token>,
    start: usize,
    current: usize,
    line: usize,
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
            self.process_next_token()?;
        }

        self.tokens
            .push_back(Token::new(TokenKind::Eof, "", None, self.line));
        Ok(&self.tokens)
    }
}

/// Internal method implementation
impl Scanner {
    fn process_next_token(&mut self) -> Result<(), ErrorCode> {
        match self.advance() {
            Some(c) => {
                // Note: The match order is done with priority to avoid matching to the wrong token
                if self.process_comparator_char_token(c)
                    || self.process_comment_char_token(c)
                    || self.process_identifier_token(c)
                    || self.process_number_token(c)
                    || self.process_string_token(c)
                    || self.process_single_char_token(c)
                    || self.process_ignored_char(c)
                {
                    // Do nothing if the operation succeeds
                } else {
                    log::warn!("Unable to complete for character");
                    Reporter::line_error(self.line, &format!("Unexpected character: {}", c));
                }
                Ok(())
            }
            None => {
                log::warn!("Unable to process any more token");
                Err(ErrorCode::ProcessError) // TODO: Change the error code
            }
        }
    }

    fn process_comparator_char_token(&mut self, c: char) -> bool {
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

    fn process_comment_char_token(&mut self, c: char) -> bool {
        // TODO: Provide support for multi line comment /* .... */
        if c != '/' {
            return false;
        }

        match self.next_match_char('/') {
            true => {
                // A comment goes until the end of the line.
                while !self.is_at_end() && self.peek(0).is_some() {
                    if let Some(v) = self.peek(0) {
                        if v != '\n' {
                            self.advance();
                        }
                    }
                }
                return true;
            }
            false => false,
        }
    }

    fn process_string_token(&mut self, c: char) -> bool {
        let string: &str;
        match c {
            '"' => {
                while !self.is_at_end() {
                    if let Some(p) = self.peek(0) {
                        match p {
                            '"' => break,
                            '\n' => self.line += 1,
                            _ => {}
                        }
                    }
                    self.advance();
                }
                if self.is_at_end() {
                    log::warn!("Unexpected character: unterminated string.");
                    Reporter::line_error(self.line, "Unexpected character: unterminated string.");
                    return false;
                }

                // The closing ".
                self.advance();

                // Trim the surrounding quotes.
                string = &self.source[self.start + 1..self.current - 1];
            }
            '\'' => {
                while !self.is_at_end() {
                    if let Some(p) = self.peek(0) {
                        match p {
                            '\'' => break,
                            '\n' => self.line += 1,
                            _ => {}
                        }
                    }
                    self.advance();
                }
                if self.is_at_end() {
                    log::warn!("Unexpected character: unterminated string.");
                    Reporter::line_error(self.line, "Unexpected character: unterminated string.");
                    return false;
                }

                // The closing ".
                self.advance();

                // Trim the surrounding quotes.
                string = &self.source[self.start + 1..self.current - 1];
            }
            _ => {
                return false;
            }
        };

        return match Literal::from_str(string) {
            Ok(l) => {
                self.add_token(TokenKind::String, Some(l));
                true
            }
            Err(_) => {
                log::warn!("Unable to convert string to process string");
                false
            }
        };
    }

    fn process_number_token(&mut self, c: char) -> bool {
        match c {
            c if is_digit(c) => {
                'first_number: loop {
                    let current = self.peek(0);
                    if current.is_some() && is_digit(current.unwrap()) {
                        self.advance();
                    } else {
                        break 'first_number;
                    }
                }

                // Look for a fractional part.
                if let Some(v) = self.peek(0) {
                    match v {
                        '.' => {
                            let next = self.peek(1);
                            if next.is_some() && is_digit(next.unwrap()) {
                                // Consume the "."
                                self.advance();

                                'fractional_number: loop {
                                    let current = self.peek(0);
                                    if current.is_some() && is_digit(current.unwrap()) {
                                        self.advance();
                                    } else {
                                        break 'fractional_number;
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }

                // TODO: Parse string to double or number
                let number = &self.source[self.start..self.current];
                return match Literal::from_str(number) {
                    Ok(l) => {
                        self.add_token(TokenKind::Number, Some(l));
                        true
                    }
                    Err(_) => {
                        log::warn!("Unable to convert string to number");
                        false
                    }
                };
            }
            _ => false,
        }
    }

    fn process_identifier_token(&mut self, c: char) -> bool {
        match c {
            c if is_alpha(c) => {
                loop {
                    let current = self.peek(0);
                    if current.is_some() && is_alpha_numeric(current.unwrap()) {
                        self.advance();
                    } else {
                        break;
                    }
                }

                // check if there is a reserved keyword
                let string = &self.source[self.start..self.current];
                if KEYWORDS.get(string).is_some() {
                    return false;
                }
                self.add_token(TokenKind::Identifier, None);

                return true;
            }
            _ => false,
        }
    }

    fn process_single_char_token(&mut self, c: char) -> bool {
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

    fn process_ignored_char(&mut self, c: char) -> bool {
        match c {
            // Ignore whitespace.
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

    fn peek(&self, to: usize) -> Option<char> {
        let to_index = self.current + to;

        if self.is_at_end() || to_index >= self.source.len() {
            return Some('\0');
        }
        return self.source.chars().nth(to_index);
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

fn is_alpha_numeric(c: char) -> bool {
    return is_alpha(c) || is_digit(c);
}
fn is_digit(c: char) -> bool {
    return c >= '0' && c <= '9';
}

fn is_alpha(c: char) -> bool {
    return c >= 'a' && c <= 'z' || c >= 'A' && c <= 'Z' || c == '_';
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

    #[test]
    fn test_ignore_comment_characters() {
        let mut scanner = Scanner::from_source("// ignored comment character");
        scanner.scan_tokens().unwrap();

        assert_eq!(scanner.tokens.len(), 1);
    }

    #[test]
    fn test_ignore_comment_line() {
        let mut scanner = Scanner::from_source("!*+-/=<> <= == // operators");
        scanner.scan_tokens().unwrap();
        assert_eq!(scanner.tokens.len(), 10);
    }

    #[test]
    fn test_generates_token_for_strings() {
        let mut scanner = Scanner::from_source("\"String character escapes\"");
        scanner.scan_tokens().unwrap();
        assert_eq!(scanner.tokens.len(), 2);
    }

    #[test]
    fn test_generates_token_for_light_string() {
        let mut scanner = Scanner::from_source("\'String character escapes\'");
        scanner.scan_tokens().unwrap();
        assert_eq!(scanner.tokens.len(), 2);
    }

    #[test]
    fn test_generates_token_for_numbers() {
        let mut scanner = Scanner::from_source("1234.567");
        scanner.scan_tokens().unwrap();
        assert_eq!(scanner.tokens.len(), 2);
    }

    #[test]
    fn test_generates_token_for_identifiers() {
        let mut scanner = Scanner::from_source("idFor1234");
        scanner.scan_tokens().unwrap();
        assert_eq!(scanner.tokens.len(), 2);
    }

    #[test]
    fn test_ignore_keywords_token() {
        let mut scanner = Scanner::from_source("and or print 1234 return nil idFor1234");
        scanner.scan_tokens().unwrap();
        assert_eq!(scanner.tokens.len(), 3);
    }
}
