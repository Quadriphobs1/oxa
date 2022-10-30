use crate::ast::expr::{Binary, Expr, Grouping, Literal, Unary, Visitor};
use crate::reporter::Reporter;
use crate::token;
use crate::token::{Token, TokenKind};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

type InnerExprType<T, V> = Box<dyn Expr<T, V>>;

// Constructor
impl Parser {
    pub fn from_tokens(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }
}

impl Parser {
    /// Equality expression parser
    ///
    /// # Rule
    /// `equality → comparison(("!=" | "==") comparison)*;`
    pub fn expression<T: 'static, V: 'static>(&mut self) -> Option<InnerExprType<T, V>>
    where
        V: Visitor<T>,
    {
        return self.equality();
    }

    fn equality<T: 'static, V: 'static>(&mut self) -> Option<InnerExprType<T, V>>
    where
        V: Visitor<T>,
    {
        let mut expr = self.comparison();
        while self.match_token(&vec![TokenKind::EqualEqual, TokenKind::EqualEqual]) {
            // TODO: Always error check
            let operator = self.previous();

            let right = self.comparison();
            // TODO: Check if break early is the best option
            if right.is_none() || operator.is_none() {
                return None;
            }
            expr = Some(Box::new(Binary::new(
                expr.unwrap(),
                operator.unwrap().to_owned(),
                right.unwrap(),
            )));
        }
        expr
    }

    /// matches an equality operator or anything of higher precedence.
    /// # Rule
    /// `comparison → term((">" | ">=" | "<" | "<=") term)* ;`
    pub fn comparison<T: 'static, V: 'static>(&mut self) -> Option<InnerExprType<T, V>>
    where
        V: Visitor<T>,
    {
        let mut expr = self.term();

        if expr.is_none() {
            return None;
        }

        while self.match_token(&vec![
            TokenKind::Greater,
            TokenKind::GreaterEqual,
            TokenKind::Less,
            TokenKind::LessEqual,
        ]) {
            let operator = self.previous();
            let right = self.term();
            // TODO: Check if break early is the best option
            if right.is_none() || operator.is_none() {
                return None;
            }
            expr = Some(Box::new(Binary::new(
                expr.unwrap(),
                operator.unwrap().to_owned(),
                right.unwrap(),
            )));
        }
        expr
    }

    /// matches addition and subtraction expression
    pub fn term<T: 'static, V: 'static>(&mut self) -> Option<InnerExprType<T, V>>
    where
        V: Visitor<T>,
    {
        let mut expr = self.factor();

        if expr.is_none() {
            return None;
        }

        while self.match_token(&vec![TokenKind::Minus, TokenKind::Plus]) {
            let operator = self.previous();
            let right = self.factor();
            // TODO: Check if break early is the best option
            if right.is_none() || operator.is_none() {
                return None;
            }
            expr = Some(Box::new(Binary::new(
                expr.unwrap(),
                operator.unwrap().to_owned(),
                right.unwrap(),
            )))
        }
        expr
    }

    /// match multiplication and division expression
    pub fn factor<T: 'static, V: 'static>(&mut self) -> Option<InnerExprType<T, V>>
    where
        V: Visitor<T>,
    {
        let expr = self.unary();

        if expr.is_none() {
            return None;
        }

        return match self.match_token(&vec![TokenKind::Slash, TokenKind::Star]) {
            true => {
                let operator = &self.previous();
                let right = self.unary();

                if right.is_none() || operator.is_none() {
                    return None;
                }
                Some(Box::new(Binary::new(
                    expr.unwrap(),
                    operator.unwrap().to_owned(),
                    right.unwrap(),
                )))
            }
            false => None,
        };
    }

    /// matches unary expression
    /// # Rule
    /// `unary → ("!" | "-") unary | primary;`
    pub fn unary<T: 'static, V: 'static>(&mut self) -> Option<InnerExprType<T, V>>
    where
        V: Visitor<T>,
    {
        if self.match_token(&vec![TokenKind::Bang, TokenKind::Minus]) {
            let operator = self.previous();
            let right = self.unary();

            if right.is_none() || operator.is_none() {
                return None;
            }

            return Some(Box::new(Unary::new(
                operator.unwrap().to_owned(),
                right.unwrap(),
            )));
        }

        return self.primary();
    }

    /// matches primitive types or parenthesis matching
    /// # Rule
    /// `primary → NUMBER | STRING | "true" | "false" | "nil" | "("expression")";`

    pub fn primary<T: 'static, V: 'static>(&mut self) -> Option<InnerExprType<T, V>>
    where
        V: Visitor<T>,
    {
        if self.match_token(&vec![TokenKind::False]) {
            return Some(Box::new(Literal::new(token::Literal::from(false))));
        }

        if self.match_token(&vec![TokenKind::True]) {
            return Some(Box::new(Literal::new(token::Literal::from(true))));
        }

        if self.match_token(&vec![TokenKind::Nil]) {
            return Some(Box::new(Literal::new(token::Literal::default())));
        }

        if self.match_token(&vec![TokenKind::Number, TokenKind::String]) {
            // TODO: Differentiate the number types
            return match self.previous() {
                Some(t) => {
                    if let Some(l) = &t.literal {
                        return Some(Box::new(Literal::new(l.to_owned())));
                    }

                    None
                }
                None => None,
            };
        }

        if self.match_token(&vec![TokenKind::LeftParen]) {
            let inner_expr = self.expression();

            if let None = inner_expr {
                return None;
            }

            if self.consume(&TokenKind::RightParen).is_none() {
                match self.peek() {
                    Some(token) => {
                        self.error(token, "Expect ')' after expression.");
                    }
                    None => {}
                }
            }

            let group = Grouping::new(inner_expr.unwrap());

            return Some(Box::new(group));
        }

        return None;
    }
}

// Private Methods
impl Parser {
    /// Checks to see if the current token has any of the given types.
    /// If so, it consumes the token and returns true. Otherwise,
    /// it returns false and leaves the current token alone.
    fn match_token(&mut self, kinds: &[TokenKind]) -> bool {
        for kind in kinds {
            if self.check_token(kind) {
                self.advance();
                return true;
            }
        }

        return false;
    }

    /// Checks if the current token is of the the given kind.
    /// If so, it consumes the token and returns `Some(T)`. Otherwise, it returns `None`.
    ///
    /// NOTE: If it returns None, should be handled as an error by consumer
    fn consume(&mut self, kind: &TokenKind) -> Option<&Token> {
        match self.check_token(kind) {
            true => self.advance(),
            false => None,
        }
    }

    /// returns true if the current token is of the given type.
    fn check_token(&self, kind: &TokenKind) -> bool {
        match self.is_at_end() {
            true => false,
            false => match self.peek() {
                Some(token) => &token.kind == kind,
                None => false,
            },
        }
    }

    /// return the current token and increment the count
    fn advance(&mut self) -> Option<&Token> {
        if !self.is_at_end() {
            self.current += 1
        };
        return self.previous();
    }

    /// return the current token.
    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    /// returns the most recently consumed token
    fn previous(&mut self) -> Option<&Token> {
        self.tokens.get(self.current - 1)
    }

    /// returns true if there is still some token to parse
    fn is_at_end(&self) -> bool {
        match self.peek() {
            Some(token) => token.kind == TokenKind::Eof,
            None => true,
        }
    }

    fn error(&self, token: &Token, message: &str) -> ParserError {
        Reporter::token_error(token, message);
        return ParserError {};
    }
}

struct ParserError {}
