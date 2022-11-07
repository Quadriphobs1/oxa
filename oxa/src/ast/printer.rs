use crate::ast::expr::{Binary, Expr, Grouping, Literal, Unary};
use crate::ast::stmt::{Expression, Print, Stmt};
use crate::ast::{expr, stmt};

pub struct AstPrinter {}

impl expr::Visitor<String> for AstPrinter {
    fn visit_binary_expr(&self, expr: &Binary<String, Self>) -> String {
        parenthesize(
            self,
            &expr.operator.lexeme,
            &[expr.left.as_ref(), expr.right.as_ref()],
        )
    }

    fn visit_grouping_expr(&self, expr: &Grouping<String, Self>) -> String {
        parenthesize(self, "group", &[expr.expression.as_ref()])
    }

    fn visit_literal_expr(&self, expr: &Literal<String, Self>) -> String {
        expr.value.to_string()
    }

    fn visit_unary_expr(&self, expr: &Unary<String, Self>) -> String {
        parenthesize(self, &expr.operator.lexeme, &[expr.right.as_ref()])
    }
}

impl stmt::Visitor<String, Self> for AstPrinter {
    fn visit_expression_stmt(&self, stmt: &Expression<String, Self, Self>) -> String {
        let value = stmt.expression.accept(self);
        format!("expression {}", value)
    }

    fn visit_print_stmt(&self, stmt: &Print<String, Self, Self>) -> String {
        let value = stmt.expression.accept(self);
        format!("print {}", value)
    }
}

impl AstPrinter {
    pub fn print_expr(&self, expr: &dyn Expr<String, Self>) -> String {
        expr.accept(self)
    }

    pub fn print_stmt(&self, stmt: &dyn Stmt<String, Self, Self>) -> String {
        stmt.accept(self)
    }
}

/// Add parentheses to the expression using Polish Notation..
/// It recursively unfurl the nested expressions arms inside the parentheses.
///
/// see https://en.wikipedia.org/wiki/Polish_notation
///
/// # Example
///
/// ```
/// use oxa::{ast::{expr::{Unary, Literal}, printer::AstPrinter}, token::{Token, TokenKind}, token};
/// use oxa::ast::printer::parenthesize;
///
/// let expr = Unary::new(
///     Token::new(TokenKind::Plus, "+", None, 1),
///     Box::new(Literal::new(token::Literal::from(2)))
/// );
/// let printer = AstPrinter {};
/// let value = parenthesize(&printer, &expr.operator.lexeme, &[expr.right.as_ref()]);
///
/// assert_eq!(&value, "(+ 2)");
/// ```
pub fn parenthesize<V: expr::Visitor<String>>(
    visitor: &V,
    name: &str,
    exprs: &[&dyn Expr<String, V>],
) -> String {
    let mut string = String::new();

    string.push('(');
    string.push_str(name);

    for expr in exprs {
        string.push(' ');
        string.push_str(&expr.accept(visitor));
    }

    string.push(')');

    string
}

#[cfg(test)]
mod parenthesize_tests {
    use crate::ast::expr::{Binary, Grouping, Literal, Unary};
    use crate::ast::printer::{parenthesize, AstPrinter};
    use crate::ast::stmt::Print;
    use crate::token;
    use crate::token::{Token, TokenKind};

    #[test]
    fn parenthesize_binary_expr() {
        let expr = Binary::new(
            Box::new(Literal::new(token::Literal::from(1))),
            Token::new(TokenKind::Plus, "+", None, 1),
            Box::new(Literal::new(token::Literal::from(2))),
        );

        let printer = AstPrinter {};

        let value = parenthesize(
            &printer,
            &expr.operator.lexeme,
            &[expr.left.as_ref(), expr.right.as_ref()],
        );

        assert_eq!(&value, "(+ 1 2)");
    }

    #[test]
    fn print_expr_test() {
        let expr = Binary::new(
            Box::new(Unary::new(
                Token::new(TokenKind::Minus, "-", None, 1),
                Box::new(Literal::new(token::Literal::from(123))),
            )),
            Token::new(TokenKind::Star, "*", None, 1),
            Box::new(Grouping::new(Box::new(Literal::new(token::Literal::from(
                45.67,
            ))))),
        );
        let printer = AstPrinter {};
        let value = printer.print_expr(&expr);
        assert_eq!(&value, "(* (- 123) (group 45.67))");
    }

    #[test]
    fn print_stmt_test() {
        let expr = Binary::new(
            Box::new(Literal::new(token::Literal::from(1))),
            Token::new(TokenKind::Plus, "+", None, 1),
            Box::new(Literal::new(token::Literal::from(2))),
        );

        let print_stmt = Print::new(Box::new(expr));
        let printer = AstPrinter {};
        let value = printer.print_stmt(&print_stmt);
        assert_eq!(&value, "print (+ 1 2)");
    }
}
