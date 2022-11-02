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

#[derive(Clone, Debug, PartialEq, Eq)]
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

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenKind::LeftParen => write!(f, "("),
            TokenKind::RightParen => write!(f, ")"),
            TokenKind::LeftBrace => write!(f, "{{"),
            TokenKind::RightBrace => write!(f, "}}"),
            TokenKind::Comma => write!(f, ","),
            TokenKind::Dot => write!(f, "."),
            TokenKind::SemiColon => write!(f, ";"),
            TokenKind::Minus => write!(f, "-"),
            TokenKind::Plus => write!(f, "+"),
            TokenKind::Slash => write!(f, "/"),
            TokenKind::Star => write!(f, "*"),
            TokenKind::Bang => write!(f, "!"),
            TokenKind::BangEqual => write!(f, "!="),
            TokenKind::Equal => write!(f, "="),
            TokenKind::EqualEqual => write!(f, "=="),
            TokenKind::Greater => write!(f, ">"),
            TokenKind::GreaterEqual => write!(f, ">="),
            TokenKind::Less => write!(f, "<"),
            TokenKind::LessEqual => write!(f, "<="),
            TokenKind::Identifier => write!(f, "identifier"),
            TokenKind::String => write!(f, "string"),
            TokenKind::Number => write!(f, "number"),
            TokenKind::And => write!(f, "and"),
            TokenKind::Class => write!(f, "class"),
            TokenKind::Else => write!(f, "else"),
            TokenKind::False => write!(f, "false"),
            TokenKind::Fun => write!(f, "fun"),
            TokenKind::For => write!(f, "for"),
            TokenKind::If => write!(f, "if"),
            TokenKind::Nil => write!(f, "nil"),
            TokenKind::Or => write!(f, "or"),
            TokenKind::Print => write!(f, "print"),
            TokenKind::Return => write!(f, "return"),
            TokenKind::Super => write!(f, "super"),
            TokenKind::This => write!(f, "this"),
            TokenKind::True => write!(f, "true"),
            TokenKind::Var => write!(f, "var"),
            TokenKind::While => write!(f, "while"),
            TokenKind::Eof => write!(f, "Eof"),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub enum LiteralKind {
    Number(i32),
    Float(f32),
    String(String),
    Bool(bool),
    #[default]
    Nil,
}

impl fmt::Display for LiteralKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LiteralKind::Number(n) => write!(f, "{}", n),
            LiteralKind::String(s) => write!(f, "{}", s),
            LiteralKind::Float(fl) => write!(f, "{}", fl),
            LiteralKind::Bool(b) => write!(f, "{}", b),
            LiteralKind::Nil => write!(f, "Nil"),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Literal {
    pub value: LiteralKind,
}

impl str::FromStr for Literal {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Literal {
            value: LiteralKind::String(s.to_string()),
        })
    }
}

impl From<&str> for Literal {
    fn from(s: &str) -> Self {
        Literal {
            value: LiteralKind::String(s.to_string()),
        }
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

impl PartialEq for Literal {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
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
        match &self.literal {
            Some(literal) => write!(f, "{} {}", self.kind, literal),
            None => write!(f, "{}", self.kind),
        }
    }
}

impl Clone for Token {
    fn clone(&self) -> Self {
        Token {
            kind: self.kind.clone(),
            lexeme: String::from(&self.lexeme),
            literal: self.literal.as_ref().cloned(),
            line: self.line,
        }
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        if self.kind != other.kind
            || self.lexeme != other.lexeme
            || self.literal != other.literal
            || self.line != other.line
        {
            return false;
        }
        true
    }
}
