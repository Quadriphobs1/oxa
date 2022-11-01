use crate::ast::expr::{Binary, Expr, Grouping, Literal, Unary, Visitor};
use crate::errors::reporter::Reporter;
use crate::errors::ErrorCode;
use crate::object::{Object, ObjectKind, ObjectValue};
use crate::token::{Token, TokenKind};

#[derive(Debug, Default)]
pub struct Interpreter();

type ResultObject = Result<Object, ErrorCode>;

impl Visitor<ResultObject> for Interpreter {
    fn visit_binary_expr(&self, expr: &Binary<ResultObject, Self>) -> ResultObject {
        let right = self.evaluate(expr.right.as_ref())?;
        let left = self.evaluate(expr.left.as_ref())?;

        match expr.operator.kind {
            TokenKind::Plus => {
                check_numeric_or_string_operands(&expr.operator, &left, &right)?;
                Ok(left + right)
            }
            TokenKind::Minus => {
                check_numeric_operands(&expr.operator, &left, &right)?;
                Ok(left - right)
            }
            TokenKind::Slash => {
                check_numeric_operands(&expr.operator, &left, &right)?;
                Ok(left / right)
            }
            TokenKind::Star => {
                check_numeric_operands(&expr.operator, &left, &right)?;
                Ok(left * right)
            }
            TokenKind::Greater => {
                check_numeric_operands(&expr.operator, &left, &right)?;
                Ok(Object::from(left > right))
            }
            TokenKind::GreaterEqual => {
                check_numeric_operands(&expr.operator, &left, &right)?;
                Ok(Object::from(left >= right))
            }
            TokenKind::Less => {
                check_numeric_operands(&expr.operator, &left, &right)?;
                Ok(Object::from(left < right))
            }
            TokenKind::LessEqual => {
                check_numeric_operands(&expr.operator, &left, &right)?;
                Ok(Object::from(left <= right))
            }
            TokenKind::BangEqual => Ok(Object::from(left != right)),
            TokenKind::EqualEqual => Ok(Object::from(left == right)),
            _ => Err(ErrorCode::RuntimeError(
                expr.operator.clone(),
                format!("invalid expression: {} {}", expr.left, expr.right),
            )),
        }
    }

    fn visit_grouping_expr(&self, expr: &Grouping<ResultObject, Self>) -> ResultObject {
        self.evaluate(expr.expression.as_ref())
    }

    fn visit_literal_expr(&self, expr: &Literal<ResultObject, Self>) -> ResultObject {
        Ok(expr.value.clone().into())
    }

    fn visit_unary_expr(&self, expr: &Unary<ResultObject, Self>) -> ResultObject {
        let right = self.evaluate(expr.right.as_ref())?;

        match expr.operator.kind {
            TokenKind::Minus => {
                check_numeric_operand(&expr.operator, &right)?;
                // return -(float)right;
                match right.value {
                    ObjectValue::Number(n) => Ok(Object::from(-n as f32)),
                    ObjectValue::Float(f) => Ok(Object::from(-f)),
                    // TODO: Update error to correct type
                    _ => Err(ErrorCode::ProcessError),
                }
            }

            TokenKind::Bang => Ok(self.is_truthy(&right)),
            // TODO: Update error to correct type
            _ => Err(ErrorCode::ProcessError),
        }
    }
}

/// public method
impl Interpreter {
    pub fn interpret(&self, expr: &dyn Expr<ResultObject, Self>) -> Result<Object, ErrorCode> {
        match self.evaluate(expr) {
            Ok(v) => Ok(v),
            Err(e) => {
                Reporter::runtime_error(&e);
                Err(e)
            }
        }
    }
}

/// private methods
impl Interpreter {
    fn evaluate(&self, expr: &dyn Expr<ResultObject, Self>) -> ResultObject {
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

fn check_numeric_operand(operator: &Token, right: &Object) -> Result<(), ErrorCode> {
    match right.kind {
        ObjectKind::Float | ObjectKind::Number => Ok(()),
        _ => Err(ErrorCode::RuntimeError(
            operator.clone(),
            format!("Operand must be a number: {}", right),
        )),
    }
}

fn check_numeric_operands(
    operator: &Token,
    left: &Object,
    right: &Object,
) -> Result<(), ErrorCode> {
    match left.kind {
        ObjectKind::Float | ObjectKind::Number => Ok(()),
        _ => Err(ErrorCode::RuntimeError(
            operator.clone(),
            format!("Operand must be a number: {} {}", left, right),
        )),
    }?;

    match right.kind {
        ObjectKind::Float | ObjectKind::Number => Ok(()),
        _ => Err(ErrorCode::RuntimeError(
            operator.clone(),
            format!("Operand must be a number: {} {}", left, right),
        )),
    }?;

    Ok(())
}

fn check_numeric_or_string_operands(
    operator: &Token,
    left: &Object,
    right: &Object,
) -> Result<(), ErrorCode> {
    match left.kind {
        ObjectKind::Float | ObjectKind::Number | ObjectKind::String => Ok(()),
        _ => Err(ErrorCode::RuntimeError(
            operator.clone(),
            format!(
                "Operands must be two numbers or two strings: {} {}",
                left, right
            ),
        )),
    }?;

    match right.kind {
        ObjectKind::Float | ObjectKind::Number | ObjectKind::String => Ok(()),
        _ => Err(ErrorCode::RuntimeError(
            operator.clone(),
            format!(
                "Operands must be two numbers or two strings: {} {}",
                left, right
            ),
        )),
    }?;
    Ok(())
}

#[cfg(test)]
mod interpreter_tests {
    use crate::ast::expr::{Binary, Grouping, Literal, Unary};
    use crate::interpreter::{Interpreter, ResultObject};
    use crate::object::Object;
    use crate::token;
    use crate::token::{Token, TokenKind};

    #[test]
    fn evaluate_unary_expr() {
        let unary: Unary<ResultObject, Interpreter> = Unary::new(
            Token::new(TokenKind::Minus, "-", None, 1),
            Box::new(Literal::new(token::Literal::from(1.0))),
        );

        let interpreter = Interpreter::default();

        let result = interpreter.interpret(&unary).unwrap();

        assert_eq!(result, Object::from(-1.0));
    }

    #[test]
    fn evaluate_binary_expr() {
        let expected: Binary<ResultObject, Interpreter> = Binary::new(
            Box::new(Literal::new(token::Literal::from(10))),
            Token::new(TokenKind::Plus, "+", None, 1),
            Box::new(Literal::new(token::Literal::from(10))),
        );

        let interpreter = Interpreter::default();

        let result = interpreter.interpret(&expected).unwrap();

        assert_eq!(result, Object::from(20));
    }

    #[test]
    fn evaluate_complex_expr() {
        let expected: Binary<ResultObject, Interpreter> = Binary::new(
            Box::new(Literal::new(token::Literal::from(10))),
            Token::new(TokenKind::Plus, "+", None, 1),
            Box::new(Binary::new(
                Box::new(Literal::new(token::Literal::from(10))),
                Token::new(TokenKind::Star, "*", None, 1),
                Box::new(Literal::new(token::Literal::from(10))),
            )),
        );

        let interpreter = Interpreter::default();

        let result = interpreter.interpret(&expected).unwrap();

        assert_eq!(result, Object::from(110));
    }

    #[test]
    fn evaluate_string_and_number_expr() {
        let expected: Binary<ResultObject, Interpreter> = Binary::new(
            Box::new(Literal::new(token::Literal::from("string"))),
            Token::new(TokenKind::Plus, "+", None, 1),
            Box::new(Literal::new(token::Literal::from(10))),
        );

        let interpreter = Interpreter::default();

        let result = interpreter.interpret(&expected).unwrap();

        assert_eq!(result, Object::from("string10"));
    }

    #[test]
    fn error_invalid_expr() {
        let expected: Binary<ResultObject, Interpreter> = Binary::new(
            Box::new(Literal::new(token::Literal::from("string"))),
            Token::new(TokenKind::Minus, "-", None, 1),
            Box::new(Literal::new(token::Literal::from(10))),
        );

        let interpreter = Interpreter::default();

        let result = interpreter.interpret(&expected);

        assert!(result.is_err());
    }

    #[test]
    fn evaluate_grouped_expr() {
        let grouping: Grouping<ResultObject, Interpreter> = Grouping::new(Box::new(Unary::new(
            Token::new(TokenKind::Bang, "!", None, 1),
            Box::new(Literal::new(token::Literal::from(false))),
        )));

        let interpreter = Interpreter::default();

        let result = interpreter.interpret(&grouping).unwrap();

        assert_eq!(result, Object::from(false));
    }
}
