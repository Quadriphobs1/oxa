use crate::errors::{reporter::Reporter, ErrorCode};
use crate::token::{Literal, Token, TokenKind, KEYWORDS};

use std::str::FromStr;

#[derive(Default)]
pub struct ScannerBuilder {
    source: String,
    line: usize,
    current: usize,
    start: usize,
}

impl ScannerBuilder {
    pub fn source(mut self, source: &str) -> ScannerBuilder {
        self.source = source.to_string();
        self
    }

    pub fn _line(mut self, line: usize) -> ScannerBuilder {
        self.line = line;
        self
    }

    pub fn _start(mut self, start: usize) -> ScannerBuilder {
        self.start = start;
        self
    }

    pub fn _current(mut self, current: usize) -> ScannerBuilder {
        self.current = current;
        self
    }

    pub fn build(self) -> Scanner {
        Scanner::new(&self.source, self.start, self.current, self.line)
    }
}

/// A code scanner using lexical grammar to tokens
#[derive(Default)]
pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

/// Constructor implementation
impl Scanner {
    /// Creates default scanner with empty string
    fn new(source: &str, start: usize, current: usize, line: usize) -> Self {
        Scanner {
            source: source.to_string(),
            start,
            current,
            line,
            ..Self::default()
        }
    }

    fn _builder() -> ScannerBuilder {
        ScannerBuilder::default()
    }
}

/// Public method implementation
impl Scanner {
    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, ErrorCode> {
        log::info!("Converting source to token");
        while !self.is_at_end() {
            // Start from the beginning of the next lexeme
            self.start = self.current;
            self.process_next_token()?;
        }

        self.tokens
            .push(Token::new(TokenKind::Eof, "", None, self.line));
        Ok(self.tokens.clone())
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
                    || self.process_numeric_token(c)
                    || self.process_string_token(c)
                    || self.process_single_char_token(c)
                    || self.process_keyword_token(c)
                    || self.process_ignored_char(c)
                {
                    // Do nothing if the operation succeeds
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

        true
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
                true
            }
            false => false,
        }
    }

    fn process_string_token(&mut self, c: char) -> bool {
        let string: &str = match c {
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
                &self.source[self.start + 1..self.current - 1]
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
                &self.source[self.start + 1..self.current - 1]
            }
            _ => {
                return false;
            }
        };

        match Literal::from_str(string) {
            Ok(l) => {
                self.add_token(TokenKind::String, Some(l));
                true
            }
            Err(_) => {
                log::warn!("Unable to convert string to process string");
                false
            }
        }
    }

    fn process_numeric_token(&mut self, c: char) -> bool {
        match c {
            c if c.is_ascii_digit() => {
                'first_number: loop {
                    let current = self.peek(0);
                    if current.is_some() && current.unwrap().is_numeric() {
                        self.advance();
                    } else {
                        break 'first_number;
                    }
                }

                // Look for a fractional part.
                if let Some(v) = self.peek(0) {
                    if v == '.' {
                        let next = self.peek(1);
                        if next.is_some() && next.unwrap().is_numeric() {
                            // Consume the "."
                            self.advance();

                            'fractional_number: loop {
                                let current = self.peek(0);
                                if current.is_some() && current.unwrap().is_numeric() {
                                    self.advance();
                                } else {
                                    break 'fractional_number;
                                }
                            }
                        }
                    }
                }

                let string = self.get_string();
                match self.get_string().contains('.') {
                    false => match string.parse::<i32>() {
                        Ok(n) => {
                            self.add_token(TokenKind::Number, Some(Literal::from(n)));
                            true
                        }
                        _ => false,
                    },

                    true => match string.parse::<f32>() {
                        Ok(f) => {
                            self.add_token(TokenKind::Number, Some(Literal::from(f)));
                            true
                        }
                        _ => false,
                    },
                }
            }
            _ => false,
        }
    }

    fn process_identifier_token(&mut self, c: char) -> bool {
        match c {
            c if c.is_alphabetic() => {
                // Consume next character until we reach a non alpha-numeric character
                loop {
                    let current = self.peek(0);
                    if current.is_some() && current.unwrap().is_alphanumeric() {
                        self.advance();
                    } else {
                        break;
                    }
                }

                // check if there is a reserved keyword
                let string = self.get_string();
                if KEYWORDS.get(&string).is_some() {
                    return false;
                }
                self.add_token(TokenKind::Identifier, None);

                true
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
            '/' => self.add_token(TokenKind::Slash, None),
            '*' => self.add_token(TokenKind::Star, None),
            ';' => self.add_token(TokenKind::SemiColon, None),
            _ => {
                return false;
            }
        }

        true
    }

    fn process_keyword_token(&mut self, c: char) -> bool {
        match c {
            c if c.is_alphanumeric() => {
                // Consume next character until we reach a non alphabetic character
                loop {
                    let current = self.peek(0);
                    if current.is_some() && current.unwrap().is_alphabetic() {
                        self.advance();
                    } else {
                        break;
                    }
                }

                let string = self.get_string();

                if let Some(keyword) = KEYWORDS.get(&string) {
                    self.add_token(keyword.clone(), None);
                    return true;
                }
                false
            }
            _ => false,
        }
    }

    fn process_ignored_char(&mut self, c: char) -> bool {
        match c {
            // Ignore whitespace.
            ' ' | '\r' | '\t' => true,
            '\n' => {
                self.line += 1;
                true
            }
            _ => false,
        }
    }

    fn add_token(&mut self, kind: TokenKind, literal: Option<Literal>) {
        // TODO: Try a less error prone approach to select the string slice
        // e.g collection with error validation for range
        let lexeme = self.get_string();

        let token = Token::new(kind, &lexeme, literal, self.line);

        self.tokens.push(token);
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
                true
            }
        }
    }

    fn increment_current(&mut self) {
        self.current += 1;
    }

    fn get_string(&self) -> String {
        let lexeme = &self.source[self.start..self.current];
        lexeme.to_string()
    }
}

#[cfg(test)]
mod scanner_tests {
    use super::*;

    #[test]
    fn test_no_token_with_initial_creation() {
        let scanner = ScannerBuilder::default().source("").build();
        assert_eq!(scanner.tokens.len(), 0);
    }

    #[test]
    fn test_generates_eof_token_at_default() {
        let mut scanner = ScannerBuilder::default().source("").build();
        scanner.scan_tokens().unwrap();

        assert_eq!(scanner.tokens.len(), 1);
    }

    #[test]
    fn test_generates_token_for_single_char() {
        let mut scanner = ScannerBuilder::default().source("(").build();
        scanner.scan_tokens().unwrap();
        assert_eq!(scanner.tokens.len(), 2);
    }

    #[test]
    fn test_generates_token_for_number() {
        let mut scanner = ScannerBuilder::default().source("1").build();
        scanner.scan_tokens().unwrap();
        assert_eq!(scanner.tokens.len(), 2);
        assert_eq!(scanner.tokens.get(0).unwrap().kind, TokenKind::Number);
    }

    #[test]
    fn test_generates_token_for_unary_number() {
        let mut scanner = ScannerBuilder::default().source("-1").build();
        scanner.scan_tokens().unwrap();
        assert_eq!(scanner.tokens.len(), 3);
        assert_eq!(scanner.tokens.get(0).unwrap().kind, TokenKind::Minus);
        assert_eq!(scanner.tokens.get(1).unwrap().kind, TokenKind::Number);
    }

    #[test]
    fn test_generates_token_for_expression() {
        let mut scanner = ScannerBuilder::default().source("1 + 2").build();
        scanner.scan_tokens().unwrap();
        assert_eq!(scanner.tokens.len(), 4);
        assert_eq!(scanner.tokens.get(0).unwrap().kind, TokenKind::Number);
        assert_eq!(scanner.tokens.get(1).unwrap().kind, TokenKind::Plus);
        assert_eq!(scanner.tokens.get(2).unwrap().kind, TokenKind::Number);
    }

    #[test]
    fn test_generates_token_for_multiple_single_char() {
        let mut scanner = ScannerBuilder::default().source("(){},.-+/*;").build();
        scanner.scan_tokens().unwrap();
        assert_eq!(scanner.tokens.len(), 12);
    }

    #[test]
    fn test_generates_token_for_single_comparator() {
        let mut scanner = ScannerBuilder::default().source("=<>!").build();
        scanner.scan_tokens().unwrap();
        assert_eq!(scanner.tokens.len(), 5);
    }

    #[test]
    fn test_generates_token_for_multi_comparator_arms() {
        let mut scanner = ScannerBuilder::default().source("<=>=!===").build();
        scanner.scan_tokens().unwrap();
        assert_eq!(scanner.tokens.len(), 5);
    }

    #[test]
    fn test_ignore_comment_characters() {
        let mut scanner = ScannerBuilder::default()
            .source("// ignored comment character")
            .build();
        scanner.scan_tokens().unwrap();

        assert_eq!(scanner.tokens.len(), 1);
    }

    #[test]
    fn test_ignore_comment_line() {
        let mut scanner = ScannerBuilder::default()
            .source("!*+-/=<> <= == // operators")
            .build();
        scanner.scan_tokens().unwrap();
        assert_eq!(scanner.tokens.len(), 10);
    }

    #[test]
    fn test_generates_token_for_strings() {
        let mut scanner = ScannerBuilder::default()
            .source("\"String character escapes\"")
            .build();
        scanner.scan_tokens().unwrap();
        assert_eq!(scanner.tokens.len(), 2);
    }

    #[test]
    fn test_generates_token_for_light_string() {
        let mut scanner = ScannerBuilder::default()
            .source("\'String character escapes\'")
            .build();
        scanner.scan_tokens().unwrap();
        assert_eq!(scanner.tokens.len(), 2);
    }

    #[test]
    fn test_generates_token_for_numbers() {
        let mut scanner = ScannerBuilder::default().source("1234.567").build();
        scanner.scan_tokens().unwrap();
        assert_eq!(scanner.tokens.len(), 2);
    }

    #[test]
    fn test_generates_token_for_identifiers() {
        let mut scanner = ScannerBuilder::default().source("idFor1234").build();
        scanner.scan_tokens().unwrap();
        assert_eq!(scanner.tokens.len(), 2);
    }

    #[test]
    fn test_ignore_keywords_token() {
        let mut scanner = ScannerBuilder::default()
            .source("and or print 1234 return nil idFor1234")
            .build();
        scanner.scan_tokens().unwrap();
        assert_eq!(scanner.tokens.len(), 8);
    }

    #[test]
    fn test_print_statement() {
        let mut scanner = ScannerBuilder::default().source("print 1 + 2;").build();
        scanner.scan_tokens().unwrap();
        assert_eq!(scanner.tokens.len(), 6);
    }
}
