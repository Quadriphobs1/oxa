use phf::phf_map;

use std::{fmt, str};

pub static KEYWORDS: phf::Map<&'static str, TokenKind> = phf_map! {
    "and" => TokenKind::And,
    "class" => TokenKind::Class,
    "else" => TokenKind::Else,
    "false" => TokenKind::False,
    "for" => TokenKind::For,
    "fun" => TokenKind::Fun,
    "if" => TokenKind::If,
    "nil" => TokenKind::Nil,
    "or" => TokenKind::Or,
    "print" => TokenKind::Print,
    "return" => TokenKind::Return,
    "super" => TokenKind::Super,
    "this" => TokenKind::This,
    "true" => TokenKind::True,
    "var" => TokenKind::Var,
    "while" => TokenKind::While,
};

#[derive(Debug, PartialEq, Clone)]
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

#[derive(Debug, Default, Clone)]
pub enum LiteralKind {
    Number(i32),
    Float(f32),
    String(String),
    Bool(bool),
    #[default]
    Nil,
}

#[derive(Debug, Default, Clone)]
pub struct Literal {
    value: LiteralKind,
}

impl ToString for Literal {
    fn to_string(&self) -> String {
        match &self.value {
            LiteralKind::Number(n) => n.to_string(),
            LiteralKind::Float(f) => f.to_string(),
            LiteralKind::Bool(b) => b.to_string(),
            LiteralKind::String(s) => s.to_string(),
            LiteralKind::Nil => "Nil".to_string(),
        }
    }
}

impl str::FromStr for Literal {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Literal {
            value: LiteralKind::String(s.to_string()),
        })
    }
}

impl From<f32> for Literal {
    fn from(f: f32) -> Self {
        Literal {
            value: LiteralKind::Float(f),
        }
    }
}

impl From<bool> for Literal {
    fn from(b: bool) -> Self {
        Literal {
            value: LiteralKind::Bool(b),
        }
    }
}

impl From<i32> for Literal {
    fn from(i: i32) -> Self {
        Literal {
            value: LiteralKind::Number(i),
        }
    }
}

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
    pub literal: Option<Literal>,
    pub line: usize,
}

impl Token {
    pub fn new(kind: TokenKind, lexeme: &str, literal: Option<Literal>, line: usize) -> Self {
        Token {
            kind,
            lexeme: lexeme.to_string(),
            literal,
            line,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} {:?} {:?}", self.kind, self.lexeme, self.literal)
    }
}

impl ToOwned for Token {
    type Owned = Self;

    fn to_owned(&self) -> Self::Owned {
        Token {
            kind: self.kind.clone(),
            lexeme: String::from(&self.lexeme),
            literal: match &self.literal {
                Some(l) => Some(l.clone()),
                None => None,
            },
            line: self.line,
        }
    }
}
