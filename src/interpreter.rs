use crate::ast::expr::{Binary, Expr, Grouping, Literal, Unary, Visitor};
use crate::object::{Object, ObjectKind, ObjectValue};
use crate::token::TokenKind;

#[derive(Debug, Default)]
pub struct Interpreter();

impl Visitor<Object> for Interpreter {
    fn visit_binary_expr(&self, expr: &Binary<Object, Self>) -> Object {
        let right = self.evaluate(expr.right.as_ref());
        let left = self.evaluate(expr.left.as_ref());

        match expr.operator.kind {
            TokenKind::Minus => left - right,
            TokenKind::Plus => left + right,
            TokenKind::Slash => left / right,
            TokenKind::Star => left * right,
            TokenKind::Greater => Object::from(left > right),
            TokenKind::GreaterEqual => Object::from(left >= right),
            TokenKind::Less => Object::from(left < right),
            TokenKind::LessEqual => Object::from(left <= right),
            TokenKind::BangEqual => Object::from(left != right),
            TokenKind::EqualEqual => Object::from(left == right),
            _ => Object::default(),
        }
    }

    fn visit_grouping_expr(&self, expr: &Grouping<Object, Self>) -> Object {
        return self.evaluate(expr);
    }

    fn visit_literal_expr(&self, expr: &Literal<Object, Self>) -> Object {
        return expr.value.clone().into();
    }

    fn visit_unary_expr(&self, expr: &Unary<Object, Self>) -> Object {
        let right = self.evaluate(expr.right.as_ref());

        match expr.operator.kind {
            TokenKind::Minus => {
                // return -(float)right;
                match right.value {
                    ObjectValue::Float(f) => Object::from(-f),
                    ObjectValue::Number(n) => Object::from(-n as f32),
                    _ => Object::default(),
                }
            }

            TokenKind::Bang => self.is_truthy(&right),
            _ => Object::default(),
        }
    }
}

impl Interpreter {
    fn evaluate(&self, expr: &dyn Expr<Object, Self>) -> Object {
        expr.accept(self)
    }

    /// checks the boolean equivalent of expression evaluation and returns a boolean object
    fn is_truthy(&self, object: &Object) -> Object {
        match object.kind {
            ObjectKind::Nil => Object::from(false),
            ObjectKind::Bool => object.clone(),
            _ => Object::from(true),
        }
    }
}
