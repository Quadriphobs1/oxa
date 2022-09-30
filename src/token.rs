use std::fmt;
use std::fmt::Formatter;

#[derive(Debug)]
pub enum TokenKind {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    SemiColon,
    Minus,
    Plus,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // literals.
    Identifier,
    String,
    Number,

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}

#[derive(Debug, Default)]
pub struct Literal {}

#[derive(Debug)]
pub struct Token {
    kind: TokenKind,
    lexeme: String,
    literal: Option<Literal>,
    line: i32,
}

impl Token {
    pub fn new(kind: TokenKind, lexeme: &str, literal: Option<Literal>, line: i32) -> Self {
        Token {
            kind,
            lexeme: lexeme.to_string(),
            literal,
            line,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} {:?} {:?}", self.kind, self.lexeme, self.literal)
    }
}
