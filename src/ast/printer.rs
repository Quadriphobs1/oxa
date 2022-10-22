use crate::ast::expr::{Binary, Expr, Grouping, Literal, Unary, Visitor};

pub struct AstPrinter {}

impl Visitor<String> for AstPrinter {
    fn visit_binary_expr(&self, expr: &Binary<String, Self>) -> String {
        return parenthesize(self, &expr.operator.lexeme, &vec![&expr.left, &expr.right]);
    }

    fn visit_grouping_expr(&self, expr: &Grouping<String, Self>) -> String {
        return parenthesize(self, "group", &vec![&expr.expression]);
    }

    fn visit_literal_expr(&self, expr: &Literal<String, Self>) -> String {
        return expr.value.to_string();
    }

    fn visit_unary_expr(&self, expr: &Unary<String, Self>) -> String {
        return parenthesize(self, &expr.operator.lexeme, &vec![&expr.right]);
    }
}

impl AstPrinter {
    pub fn print(&self, expr: Box<dyn Expr<String, Self>>) -> String {
        return expr.accept(self);
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
/// use oxa::{ast::{expr::{Unary, Literal}, printer::AstPrinter}, token::{Token, TokenKind}};
/// use oxa::ast::printer::parenthesize;
///
/// let expr = Unary::new(Token::new(TokenKind::Plus, "+", None, 1), Box::new(Literal::new("2".to_string())));
/// let printer = AstPrinter {};
/// let value = parenthesize(&printer, &expr.operator.lexeme, &vec![&expr.right]);
///
/// assert_eq!(&value, "(+ 2)");
/// ```
pub fn parenthesize<V: Visitor<String>>(
    visitor: &V,
    name: &str,
    exprs: &Vec<&Box<dyn Expr<String, V>>>,
) -> String {
    let mut string = String::new();

    string.push('(');
    string.push_str(name);

    for expr in exprs {
        string.push(' ');
        string.push_str(&*expr.accept(visitor));
    }

    string.push(')');

    string
}

#[cfg(test)]
mod parenthesize_tests {
    use crate::ast::expr::{Binary, Grouping, Literal, Unary};
    use crate::ast::printer::{parenthesize, AstPrinter};
    use crate::token::{Token, TokenKind};

    #[test]
    fn parenthesize_binary_expr() {
        let expr = Binary::new(
            Box::new(Literal::new("1".to_string())),
            Token::new(TokenKind::Plus, "+", None, 1),
            Box::new(Literal::new("2".to_string())),
        );

        let printer = AstPrinter {};

        let value = parenthesize(
            &printer,
            &expr.operator.lexeme,
            &vec![&expr.left, &expr.right],
        );

        assert_eq!(&value, "(+ 1 2)");
    }

    #[test]
    fn printer_test() {
        let expr = Binary::new(
            Box::new(Unary::new(
                Token::new(TokenKind::Minus, "-", None, 1),
                Box::new(Literal::new("123".to_string())),
            )),
            Token::new(TokenKind::Star, "*", None, 1),
            Box::new(Grouping::new(Box::new(Literal::new("45.67".to_string())))),
        );
        let printer = AstPrinter {};
        let value = printer.print(Box::new(expr));
        assert_eq!(&value, "(* (- 123) (group 45.67))")
    }
}
