use crate::ast::expr::{Assign, Binary, Expr, ExprKind, Grouping, Literal, Unary, Variable};
use crate::ast::stmt::{Const, Expression, Let, Print, Stmt};
use crate::ast::{expr, stmt};
use crate::errors::reporter::Reporter;
use crate::errors::ErrorCode;
use crate::token;
use crate::token::{Token, TokenKind};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

pub type InnerExprType<T, V> = Box<dyn Expr<T, V>>;
pub type InnerStmtType<T, U, V> = Box<dyn Stmt<T, U, V>>;

// Constructor
impl Parser {
    pub fn from_tokens(tokens: &[Token]) -> Self {
        Parser {
            tokens: Vec::from(tokens),
            current: 0,
        }
    }
}

impl Parser {
    /// Parses tokens in a top down approach to find the appropriate expression, some expression take
    /// more priority then other and eventually every expression boil down to primitives
    pub fn parse<T: 'static, U: 'static, V: 'static>(
        &mut self,
    ) -> Result<Vec<InnerStmtType<T, U, V>>, ErrorCode>
    where
        U: stmt::Visitor<T, V>,
        V: expr::Visitor<T>,
    {
        let mut statements: Vec<InnerStmtType<T, U, V>> = Vec::new();

        while !self.is_at_end() {
            if let Some(stmt) = self.declaration::<T, U, V>() {
                statements.push(stmt);
            } else {
                self.synchronize();
                Reporter::line_error(self.current, "Parser error");
            }
        }

        Ok(statements)
    }
}

/// Statement parser methods
impl Parser {
    fn declaration<T: 'static, U: 'static, V: 'static>(&mut self) -> Option<InnerStmtType<T, U, V>>
    where
        U: stmt::Visitor<T, V>,
        V: expr::Visitor<T>,
    {
        if self.match_token(&[TokenKind::Const]) {
            return self.var_declaration(true);
        }

        if self.match_token(&[TokenKind::Let]) {
            return self.var_declaration(false);
        }

        self.statement()
    }

    fn statement<T: 'static, U: 'static, V: 'static>(&mut self) -> Option<InnerStmtType<T, U, V>>
    where
        U: stmt::Visitor<T, V>,
        V: expr::Visitor<T>,
    {
        if self.match_token(&[TokenKind::Print]) {
            return self.print_statement::<T, U, V>();
        }

        self.expression_statement::<T, U, V>()
    }

    /// print statement parser.
    ///
    /// # Rule
    /// `print_stmt      → "print" expression ";" ;`
    fn print_statement<T: 'static, U: 'static, V: 'static>(
        &mut self,
    ) -> Option<InnerStmtType<T, U, V>>
    where
        U: stmt::Visitor<T, V>,
        V: expr::Visitor<T>,
    {
        if let Some(expr) = self.expression::<T, V>() {
            self.check_stmt_terminal();
            let print: Print<T, U, V> = Print::new(expr);

            return Some(Box::new(print));
        }
        None
    }

    /// print statement parser.
    ///
    /// # Rule
    /// `var_decl        → "let" IDENTIFIER ( "=" expression )? ";"
    ///                  | "const" IDENTIFIER ( "=" expression )? ";" ;`
    fn var_declaration<T: 'static, U: 'static, V: 'static>(
        &mut self,
        is_const: bool,
    ) -> Option<InnerStmtType<T, U, V>>
    where
        U: stmt::Visitor<T, V>,
        V: expr::Visitor<T>,
    {
        let name = self.consume(&TokenKind::Identifier)?;

        let initializer = if self.match_token(&[TokenKind::Equal]) {
            self.expression::<T, V>()?
        } else {
            // Make declaration `Nil` by default if there is no initializer
            Box::new(Literal::new(token::Literal::default()))
        };

        self.check_stmt_terminal();

        if is_const {
            return Some(Box::new(Const::new(name, initializer)));
        }

        Some(Box::new(Let::new(name, initializer)))
    }

    /// Expression statement parser
    ///
    /// # Rule
    /// `expr_stmt       → expression ";" ;`
    fn expression_statement<T: 'static, U: 'static, V: 'static>(
        &mut self,
    ) -> Option<InnerStmtType<T, U, V>>
    where
        U: stmt::Visitor<T, V>,
        V: expr::Visitor<T>,
    {
        if let Some(expr) = self.expression::<T, V>() {
            self.check_stmt_terminal();
            let print: Expression<T, U, V> = Expression::new(expr);

            return Some(Box::new(print));
        }
        None
    }
}

/// Expression parser methods
impl Parser {
    /// equality expression parser.
    ///
    /// # Rule
    /// `equality → comparison(("!=" | "==") comparison)*;`
    pub fn expression<T: 'static, V: 'static>(&mut self) -> Option<InnerExprType<T, V>>
    where
        V: expr::Visitor<T>,
    {
        self.assignment()
    }

    /// assignment parser method
    ///
    /// # Rule
    /// `expression    → assignment ;`
    /// `assignment    → IDENTIFIER "=" assignment | equality ;`
    pub fn assignment<T: 'static, V: 'static>(&mut self) -> Option<InnerExprType<T, V>>
    where
        V: expr::Visitor<T>,
    {
        let expr = self.equality()?;

        if self.match_token(&[TokenKind::Equal]) {
            let equals = self.previous()?;
            let value = self.assignment()?;
            if let ExprKind::Variable(v) = expr.kind() {
                let name = &v.name;
                return Some(Box::new(Assign::new(name.clone(), value)));
            }
            // The grammar is incorrect
            Reporter::token_error(&equals, "Invalid assignment target.");
        }

        Some(expr)
    }

    fn equality<T: 'static, V: 'static>(&mut self) -> Option<InnerExprType<T, V>>
    where
        V: expr::Visitor<T>,
    {
        let mut expr = self.comparison();
        while self.match_token(&[TokenKind::EqualEqual, TokenKind::EqualEqual]) {
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
    fn comparison<T: 'static, V: 'static>(&mut self) -> Option<InnerExprType<T, V>>
    where
        V: expr::Visitor<T>,
    {
        let mut expr = self.term();

        expr.as_ref()?;

        while self.match_token(&[
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
    fn term<T: 'static, V: 'static>(&mut self) -> Option<InnerExprType<T, V>>
    where
        V: expr::Visitor<T>,
    {
        let mut expr = self.factor();

        expr.as_ref()?;

        while self.match_token(&[TokenKind::Minus, TokenKind::Plus]) {
            let operator = self.previous();
            let right = self.factor();
            // TODO: Check if break early is the best option
            if right.is_none() || operator.is_none() {
                return None;
            }
            expr = Some(Box::new(Binary::new(
                expr.unwrap(),
                operator.unwrap(),
                right.unwrap(),
            )))
        }
        expr
    }

    /// match multiplication and division expression.
    ///
    /// # Rule
    /// `factor -> primary ("*" | "/") primary
    ///            | primary;`
    fn factor<T: 'static, V: 'static>(&mut self) -> Option<InnerExprType<T, V>>
    where
        V: expr::Visitor<T>,
    {
        let expr = self.unary();

        expr.as_ref()?;

        match self.match_token(&[TokenKind::Slash, TokenKind::Star]) {
            true => {
                let operator = self.previous();
                let right = self.unary();

                if right.is_none() || operator.is_none() {
                    return None;
                }
                Some(Box::new(Binary::new(
                    expr.unwrap(),
                    operator.unwrap(),
                    right.unwrap(),
                )))
            }
            false => expr,
        }
    }

    /// matches unary expression.
    ///
    /// # Rule
    /// `unary → ("!" | "-") unary
    ///          | primary;`
    fn unary<T: 'static, V: 'static>(&mut self) -> Option<InnerExprType<T, V>>
    where
        V: expr::Visitor<T>,
    {
        if self.match_token(&[TokenKind::Bang, TokenKind::Minus]) {
            let operator = self.previous();
            let right = self.unary();

            if right.is_none() || operator.is_none() {
                return None;
            }

            return Some(Box::new(Unary::new(operator.unwrap(), right.unwrap())));
        }

        self.primary()
    }

    /// matches primitive types or parenthesis matching.
    ///
    /// # Rule
    /// `primary → NUMBER | STRING
    ///            | "true" | "false" | "nil"
    ///            | "("expression")"
    ///            | IDENTIFIER;`
    fn primary<T: 'static, V: 'static>(&mut self) -> Option<InnerExprType<T, V>>
    where
        V: expr::Visitor<T>,
    {
        if self.match_token(&[TokenKind::Identifier]) {
            return match self.previous() {
                Some(t) => Some(Box::new(Variable::new(t))),
                None => None,
            };
        }

        if self.match_token(&[TokenKind::Number, TokenKind::String]) {
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

        if self.match_token(&[TokenKind::LeftParen]) {
            let inner_expr = self.expression();

            inner_expr.as_ref()?;

            if self.consume(&TokenKind::RightParen).is_none() {
                if let Some(token) = self.peek() {
                    self.error(&token, "Expect ')' after expression.");
                }
            }

            let group = Grouping::new(inner_expr.unwrap());

            return Some(Box::new(group));
        }

        if self.match_token(&[TokenKind::False]) {
            return Some(Box::new(Literal::new(token::Literal::from(false))));
        }

        if self.match_token(&[TokenKind::True]) {
            return Some(Box::new(Literal::new(token::Literal::from(true))));
        }

        if self.match_token(&[TokenKind::Nil]) {
            return Some(Box::new(Literal::new(token::Literal::default())));
        }

        if let Some(t) = self.peek() {
            self.error(&t, "Expect expression.");
        }
        None
    }
}

/// Private Methods
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

        false
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

    fn check_stmt_terminal(&mut self) {
        if self.consume(&TokenKind::SemiColon).is_none() {
            if let Some(token) = self.peek() {
                self.error(&token, "Expect ';' after expression.");
            }
        }
    }

    /// return the current token and increment the count
    fn advance(&mut self) -> Option<Token> {
        if !self.is_at_end() {
            self.current += 1
        };
        self.previous()
    }

    /// return the current token.
    fn peek(&self) -> Option<Token> {
        self.tokens.get(self.current).cloned()
    }

    /// return the item at index
    fn peek_index(&self, index: usize) -> Option<Token> {
        self.tokens.get(index).cloned()
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
                | TokenKind::Let
                | TokenKind::Const
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

    fn error(&self, token: &Token, message: &str) -> ErrorCode {
        Reporter::token_error(token, message);
        ErrorCode::ParserError(token.clone(), message.to_string())
    }
}

#[cfg(test)]
mod parser_tests {
    use crate::ast::expr::{Binary, Unary};
    use crate::ast::printer::AstPrinter;
    use crate::parser::{Literal, Parser};
    use crate::token;
    use crate::token::{Token, TokenKind};

    #[test]
    fn is_at_end_with_empty_tokens() {
        let parser = Parser::from_tokens(&[]);

        assert!(parser.is_at_end());
    }

    #[test]
    fn confirms_existence_of_token() {
        let tokens = [
            Token::new(TokenKind::Minus, "-", None, 1),
            Token::new(TokenKind::Plus, "+", None, 1),
            Token::new(TokenKind::Slash, "/", None, 1),
            Token::new(TokenKind::Star, "*", None, 1),
        ];

        let mut parser = Parser::from_tokens(&tokens);
        assert!(parser.match_token(&[
            TokenKind::Minus,
            TokenKind::Plus,
            TokenKind::Slash,
            TokenKind::Plus
        ]))
    }

    #[test]
    fn parse_simple_expression() {
        // !false
        let tokens = [
            Token::new(TokenKind::Bang, "!", None, 1),
            Token::new(
                TokenKind::False,
                "false",
                Some(token::Literal::from(false)),
                1,
            ),
        ];

        let mut parser = Parser::from_tokens(&tokens);
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
    fn parse_unary_expression() {
        let tokens = [
            Token::new(TokenKind::Minus, "-", None, 1),
            Token::new(TokenKind::Number, "2", Some(token::Literal::from(2)), 1),
        ];

        let mut parser = Parser::from_tokens(&tokens);
        let expr = parser.unary::<String, AstPrinter>();

        assert!(expr.is_some());

        let expected = Unary::new(
            Token::new(TokenKind::Minus, "-", None, 1),
            Box::new(Literal::<String, AstPrinter>::new(token::Literal::from(2))),
        );

        assert_eq!(expr.unwrap().to_string(), expected.to_string());
    }

    #[test]
    fn parse_complex_expression() {
        // 10 == 10
        let tokens = [
            Token::new(TokenKind::Number, "10", Some(token::Literal::from(10)), 1),
            Token::new(TokenKind::EqualEqual, "==", None, 1),
            Token::new(TokenKind::Number, "10", Some(token::Literal::from(10)), 1),
        ];

        let mut parser = Parser::from_tokens(&tokens);
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
        let tokens = [
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

        let mut parser = Parser::from_tokens(&tokens);
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
        let tokens = [
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

        let mut parser = Parser::from_tokens(&tokens);
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

    #[test]
    fn error_parsing_incomplete_expression() {
        // 1 +
        let tokens = [
            Token::new(TokenKind::Number, "1", Some(token::Literal::from(1)), 1),
            Token::new(TokenKind::Plus, "+", None, 1),
        ];

        let mut parser = Parser::from_tokens(&tokens);
        let expr = parser.expression::<String, AstPrinter>();

        assert!(expr.is_none());
    }

    #[test]
    fn parse_print_unary_statement() {
        // print "one";
        let tokens = [
            Token::new(TokenKind::Print, "print", None, 1),
            Token::new(
                TokenKind::String,
                "one",
                Some(token::Literal::from("one")),
                1,
            ),
            Token::new(TokenKind::SemiColon, ";", None, 1),
            Token::new(TokenKind::Eof, "", None, 1),
        ];

        let mut parser = Parser::from_tokens(&tokens);
        let statement = parser.statement::<String, AstPrinter, AstPrinter>();
        assert!(statement.is_some());
    }

    #[test]
    fn parse_expr_statement() {
        // print (a + b) * (10 / 2);
        let tokens = [
            Token::new(TokenKind::Print, "print", None, 1),
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
            Token::new(TokenKind::SemiColon, ";", None, 1),
            Token::new(TokenKind::Eof, "", None, 1),
        ];

        let mut parser = Parser::from_tokens(&tokens);
        let statement = parser.statement::<String, AstPrinter, AstPrinter>();
        assert!(statement.is_some());
        assert_eq!(statement.unwrap().to_string(), format!("a + b * 10 / 2"));
    }

    #[test]
    fn parse_expr_statements() {
        // print 1 + 2;
        let tokens = [
            Token::new(TokenKind::Print, "print", None, 1),
            Token::new(TokenKind::Number, "1", Some(token::Literal::from(1)), 1),
            Token::new(TokenKind::Plus, "+", None, 1),
            Token::new(TokenKind::Number, "2", Some(token::Literal::from(2)), 1),
            Token::new(TokenKind::SemiColon, ";", None, 1),
            Token::new(TokenKind::Eof, "", None, 1),
        ];

        let mut parser = Parser::from_tokens(&tokens);
        let statements = parser.parse::<String, AstPrinter, AstPrinter>().unwrap();
        let print = statements.get(0);
        assert_eq!(print.unwrap().to_string(), format!("1 + 2"))
    }
}
