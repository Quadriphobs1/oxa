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
    /// Parses tokens in a top down approach to find the appropriate expression, some expression take
    /// more priority then other and eventually every expression boil down to primitives
    pub fn parse<T: 'static, V: 'static>(&mut self) -> Option<InnerExprType<T, V>>
    where
        V: Visitor<T>,
    {
        match self.expression() {
            Some(e) => Some(e),
            None => {
                Reporter::line_error(self.current, "Parser error");
                None
            }
        }
    }

    /// equality expression parser.
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
            let operator = self.previous();
            let right = self.comparison();
            if right.is_none() || operator.is_none() {
                return None;
            }
            expr = Some(Box::new(Binary::new(
                expr.unwrap(),
                operator.unwrap().clone(),
                right.unwrap(),
            )));
        }
        expr
    }

    /// matches an equality operator or anything of higher precedence.
    ///
    /// # Rule
    /// `comparison → term ((">" | ">=" | "<" | "<=") term)* ;`
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
                operator.unwrap().clone(),
                right.unwrap(),
            )));
        }
        expr
    }

    /// matches addition and subtraction expression.
    ///
    /// # Rule
    /// `term -> primary ("+" | "-") primary;`
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
                operator.unwrap().clone(),
                right.unwrap(),
            )))
        }
        expr
    }

    /// match multiplication and division expression.
    ///
    /// # Rule
    /// `factor -> primary ("*" | "/") primary | primary;`
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
                let operator = self.previous();
                let right = self.unary();

                if right.is_none() || operator.is_none() {
                    return None;
                }
                Some(Box::new(Binary::new(
                    expr.unwrap(),
                    operator.unwrap().clone(),
                    right.unwrap(),
                )))
            }
            false => expr,
        };
    }

    /// matches unary expression.
    ///
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
                operator.unwrap().clone(),
                right.unwrap(),
            )));
        }

        return self.primary();
    }

    /// matches primitive types or parenthesis matching.
    ///
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
                        return Some(Box::new(Literal::new(l.clone())));
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
                        self.error(&token, "Expect ')' after expression.");
                    }
                    None => {}
                }
            }

            let group = Grouping::new(inner_expr.unwrap());

            return Some(Box::new(group));
        }

        if let Some(t) = self.peek() {
            self.error(&t, "Expect expression.");
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
    fn consume(&mut self, kind: &TokenKind) -> Option<Token> {
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
    fn advance(&mut self) -> Option<Token> {
        if !self.is_at_end() {
            self.current += 1
        };
        return self.previous();
    }

    /// return the current token.
    fn peek(&self) -> Option<Token> {
        let token = self.tokens.get(self.current);

        return match token {
            Some(t) => Some(t.clone()),
            None => None,
        };
    }

    /// return the item at index
    fn peek_index(&self, index: usize) -> Option<Token> {
        let token = self.tokens.get(index);

        return match token {
            Some(t) => Some(t.clone()),
            None => None,
        };
    }

    /// returns the most recently consumed token
    fn previous(&mut self) -> Option<Token> {
        self.peek_index(self.current - 1)
    }

    /// returns true if there is still some token to parse
    fn is_at_end(&self) -> bool {
        match self.peek() {
            Some(token) => token.kind == TokenKind::Eof,
            None => true,
        }
    }

    /// discards tokens until we find a statement boundary
    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if let Some(p) = self.previous() {
                if p.kind == TokenKind::SemiColon {
                    return;
                }
            }

            let current = self.peek();

            if current.is_none() {
                return;
            }

            match current.unwrap().kind {
                TokenKind::Class
                | TokenKind::Fun
                | TokenKind::Var
                | TokenKind::For
                | TokenKind::If
                | TokenKind::While
                | TokenKind::Print
                | TokenKind::Return => {
                    return;
                }
                _ => {}
            }

            self.advance();
        }
    }

    fn error(&self, token: &Token, message: &str) -> ParserError {
        Reporter::token_error(token, message);
        return ParserError {};
    }
}

struct ParserError {}

#[cfg(test)]
mod parser_tests {
    use crate::ast::expr::{Binary, Unary};
    use crate::ast::printer::AstPrinter;
    use crate::parser::{Literal, Parser};
    use crate::token;
    use crate::token::{Token, TokenKind};

    #[test]
    fn is_at_end_with_empty_tokens() {
        let parser = Parser::from_tokens(vec![]);

        assert!(parser.is_at_end());
    }

    #[test]
    fn confirms_existence_of_token() {
        let tokens = vec![
            Token::new(TokenKind::Minus, "-", None, 1),
            Token::new(TokenKind::Plus, "+", None, 1),
            Token::new(TokenKind::Slash, "/", None, 1),
            Token::new(TokenKind::Star, "*", None, 1),
        ];

        let mut parser = Parser::from_tokens(tokens);
        assert!(parser.match_token(&vec![
            TokenKind::Minus,
            TokenKind::Plus,
            TokenKind::Slash,
            TokenKind::Plus
        ]))
    }

    #[test]
    fn parse_simple_expression() {
        // !false
        let tokens = vec![
            Token::new(TokenKind::Bang, "!", None, 1),
            Token::new(
                TokenKind::False,
                "false",
                Some(token::Literal::from(false)),
                1,
            ),
        ];

        let mut parser = Parser::from_tokens(tokens);
        let expr = parser.unary::<String, AstPrinter>();

        assert!(expr.is_some());

        let expected = Unary::new(
            Token::new(TokenKind::Bang, "!", None, 1),
            Box::new(Literal::<String, AstPrinter>::new(token::Literal::from(
                false,
            ))),
        );

        assert_eq!(expr.unwrap().to_string(), expected.to_string());
    }

    #[test]
    fn parse_complex_expression() {
        // 10 == 10
        let tokens = vec![
            Token::new(TokenKind::Number, "10", Some(token::Literal::from(10)), 1),
            Token::new(TokenKind::EqualEqual, "==", None, 1),
            Token::new(TokenKind::Number, "10", Some(token::Literal::from(10)), 1),
        ];

        let mut parser = Parser::from_tokens(tokens);
        let expr = parser.expression::<String, AstPrinter>();

        assert!(expr.is_some());

        let expected = Binary::new(
            Box::new(Literal::<String, AstPrinter>::new(token::Literal::from(10))),
            Token::new(TokenKind::EqualEqual, "==", None, 1),
            Box::new(Literal::<String, AstPrinter>::new(token::Literal::from(10))),
        );

        assert_eq!(expr.unwrap().to_string(), expected.to_string());
    }

    #[test]
    fn parse_advance_expression() {
        // a == b == c == d == e
        let tokens = vec![
            Token::new(TokenKind::String, "a", Some(token::Literal::from("a")), 1),
            Token::new(TokenKind::EqualEqual, "==", None, 1),
            Token::new(TokenKind::String, "b", Some(token::Literal::from("b")), 1),
            Token::new(TokenKind::EqualEqual, "==", None, 1),
            Token::new(TokenKind::String, "c", Some(token::Literal::from("c")), 1),
            Token::new(TokenKind::EqualEqual, "==", None, 1),
            Token::new(TokenKind::String, "d", Some(token::Literal::from("d")), 1),
            Token::new(TokenKind::EqualEqual, "==", None, 1),
            Token::new(TokenKind::String, "e", Some(token::Literal::from("e")), 1),
        ];

        let mut parser = Parser::from_tokens(tokens);
        let expr = parser.expression::<String, AstPrinter>();

        assert!(expr.is_some());

        let expected = Binary::new(
            Box::new(Binary::new(
                Box::new(Binary::new(
                    Box::new(Binary::new(
                        Box::new(Literal::<String, AstPrinter>::new(token::Literal::from(
                            "a",
                        ))),
                        Token::new(TokenKind::EqualEqual, "==", None, 1),
                        Box::new(Literal::<String, AstPrinter>::new(token::Literal::from(
                            "b",
                        ))),
                    )),
                    Token::new(TokenKind::EqualEqual, "==", None, 1),
                    Box::new(Literal::<String, AstPrinter>::new(token::Literal::from(
                        "c",
                    ))),
                )),
                Token::new(TokenKind::EqualEqual, "==", None, 1),
                Box::new(Literal::<String, AstPrinter>::new(token::Literal::from(
                    "d",
                ))),
            )),
            Token::new(TokenKind::EqualEqual, "==", None, 1),
            Box::new(Literal::<String, AstPrinter>::new(token::Literal::from(
                "e",
            ))),
        );

        assert_eq!(expr.unwrap().to_string(), expected.to_string());
    }

    #[test]
    fn parse_extreme_expression() {
        // (a + b) * (10 / 2)
        let tokens = vec![
            Token::new(TokenKind::LeftParen, "(", None, 1),
            Token::new(TokenKind::String, "a", Some(token::Literal::from("a")), 1),
            Token::new(TokenKind::Plus, "+", None, 1),
            Token::new(TokenKind::String, "b", Some(token::Literal::from("b")), 1),
            Token::new(TokenKind::RightParen, ")", None, 1),
            Token::new(TokenKind::Star, "*", None, 1),
            Token::new(TokenKind::LeftParen, "(", None, 1),
            Token::new(TokenKind::Number, "10", Some(token::Literal::from(10)), 1),
            Token::new(TokenKind::Slash, "/", None, 1),
            Token::new(TokenKind::Number, "2", Some(token::Literal::from(2)), 1),
            Token::new(TokenKind::RightParen, ")", None, 1),
        ];

        let mut parser = Parser::from_tokens(tokens);
        let expr = parser.expression::<String, AstPrinter>();

        assert!(expr.is_some());

        let expected = Binary::new(
            Box::new(Binary::<String, AstPrinter>::new(
                Box::new(Literal::<String, AstPrinter>::new(token::Literal::from(
                    "a",
                ))),
                Token::new(TokenKind::Plus, "+", None, 1),
                Box::new(Literal::<String, AstPrinter>::new(token::Literal::from(
                    "b",
                ))),
            )),
            Token::new(TokenKind::Star, "*", None, 1),
            Box::new(Binary::<String, AstPrinter>::new(
                Box::new(Literal::<String, AstPrinter>::new(token::Literal::from(10))),
                Token::new(TokenKind::Slash, "/", None, 1),
                Box::new(Literal::<String, AstPrinter>::new(token::Literal::from(2))),
            )),
        );

        assert_eq!(expr.unwrap().to_string(), expected.to_string());
    }
}
