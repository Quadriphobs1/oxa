use crate::ast::expr::{Assign, Binary, Expr, Grouping, Literal, Unary, Variable};
use crate::ast::stmt::{Const, Expression, Let, Print, Stmt};
use crate::ast::{expr, stmt};
use crate::environment::Environment;
use crate::errors::reporter::Reporter;
use crate::errors::ErrorCode;
use crate::object::{Object, ObjectKind, ObjectValue};
use crate::token::{Token, TokenKind};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Default)]
pub struct InterpreterBuilder {
    environment: Rc<RefCell<Environment>>,
}

impl InterpreterBuilder {
    pub fn new() -> Self {
        InterpreterBuilder {
            environment: Rc::new(RefCell::new(Environment::default())),
        }
    }

    pub fn environment(mut self, environment: Rc<RefCell<Environment>>) -> Self {
        self.environment = environment;
        self
    }

    pub fn build(self) -> Interpreter {
        Interpreter::new(self.environment)
    }
}

pub struct Interpreter {
    environment: Rc<RefCell<Environment>>,
}

/// constructor
impl Interpreter {
    fn new(environment: Rc<RefCell<Environment>>) -> Self {
        Interpreter { environment }
    }

    pub fn builder() -> InterpreterBuilder {
        InterpreterBuilder::default()
    }
}

type ResultObject = Result<Object, ErrorCode>;

impl expr::Visitor<ResultObject> for Interpreter {
    fn visit_assign_expr(&self, expr: &Assign<ResultObject, Self>) -> ResultObject {
        let value = self.evaluate(expr.value.as_ref())?;
        // let obj = self.environment.borrow_mut().assign(&expr.name, value);
        match self.environment.borrow_mut().assign(&expr.name, value) {
            // TODO: Update error to reference error to unknown variable
            // "Undefined variable '" + name.lexeme + "'.");
            None => Err(ErrorCode::ProcessError),
            Some(obj) => {
                let obj_borrow = obj.borrow_mut();
                Ok(obj_borrow.to_owned())
            }
        }
    }

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

    fn visit_variable_expr(&self, expr: &Variable<ResultObject, Self>) -> ResultObject {
        match self.environment.borrow_mut().get(&expr.name) {
            // TODO: Update error to reference error to unknown variable
            // "Undefined variable '" + name.lexeme + "'.");
            None => Err(ErrorCode::ProcessError),
            Some(obj) => Ok(obj.borrow_mut().clone()),
        }
    }
}

impl stmt::Visitor<ResultObject, Self> for Interpreter {
    fn visit_expression_stmt(
        &mut self,
        stmt: &Expression<ResultObject, Self, Self>,
    ) -> ResultObject {
        let value = self.evaluate(stmt.expression.as_ref())?;
        Ok(value)
    }

    fn visit_print_stmt(&mut self, stmt: &Print<ResultObject, Self, Self>) -> ResultObject {
        let value = self.evaluate(stmt.expression.as_ref())?;
        println!("{}", value);
        Ok(value)
    }

    fn visit_let_stmt(&mut self, stmt: &Let<ResultObject, Self, Self>) -> ResultObject {
        let obj = self
            .environment
            .borrow_mut()
            .define(&stmt.name.lexeme, self.evaluate(stmt.initializer.as_ref())?);
        let obj_borrow = obj.borrow_mut();
        Ok(obj_borrow.to_owned())
    }

    fn visit_const_stmt(&mut self, stmt: &Const<ResultObject, Self, Self>) -> ResultObject {
        // TODO: Make const immutable data and can't accept assign after initialisation
        let obj = self
            .environment
            .borrow_mut()
            .define(&stmt.name.lexeme, self.evaluate(stmt.initializer.as_ref())?);
        let obj_borrow = obj.borrow_mut();
        Ok(obj_borrow.to_owned())
    }
}

/// public method
impl Interpreter {
    pub fn interpret(
        &mut self,
        statements: &[Box<dyn Stmt<ResultObject, Self, Self>>],
    ) -> Result<Vec<Object>, ErrorCode> {
        let mut vec = Vec::new();
        for statement in statements {
            match self.execute(statement.as_ref()) {
                Ok(v) => vec.push(v),
                Err(e) => {
                    Reporter::runtime_error(&e);
                }
            }
        }

        Ok(vec)
    }
}

/// private methods
impl Interpreter {
    fn execute(&mut self, stmt: &dyn Stmt<ResultObject, Self, Self>) -> ResultObject {
        stmt.accept(self)
    }

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
    use crate::ast::expr::{Assign, Binary, Grouping, Literal, Unary};
    use crate::ast::stmt::{Expression, Let, Print};
    use crate::interpreter::{Interpreter, InterpreterBuilder, ResultObject};
    use crate::object::Object;
    use crate::token;
    use crate::token::{Token, TokenKind};

    #[test]
    fn evaluate_unary_expr() {
        let unary: Unary<ResultObject, Interpreter> = Unary::new(
            Token::new(TokenKind::Minus, "-", None, 1),
            Box::new(Literal::new(token::Literal::from(1.0))),
        );

        let interpreter = InterpreterBuilder::new().build();

        let result = interpreter.evaluate(&unary).unwrap();

        assert_eq!(result, Object::from(-1.0));
    }

    #[test]
    fn evaluate_binary_expr() {
        let expression: Binary<ResultObject, Interpreter> = Binary::new(
            Box::new(Literal::new(token::Literal::from(10))),
            Token::new(TokenKind::Plus, "+", None, 1),
            Box::new(Literal::new(token::Literal::from(10))),
        );

        let interpreter = InterpreterBuilder::new().build();

        let result = interpreter.evaluate(&expression).unwrap();

        assert_eq!(result, Object::from(20));
    }

    #[test]
    fn evaluate_complex_expr() {
        let expression: Binary<ResultObject, Interpreter> = Binary::new(
            Box::new(Literal::new(token::Literal::from(10))),
            Token::new(TokenKind::Plus, "+", None, 1),
            Box::new(Binary::new(
                Box::new(Literal::new(token::Literal::from(10))),
                Token::new(TokenKind::Star, "*", None, 1),
                Box::new(Literal::new(token::Literal::from(10))),
            )),
        );

        let interpreter = InterpreterBuilder::new().build();

        let result = interpreter.evaluate(&expression).unwrap();

        assert_eq!(result, Object::from(110));
    }

    #[test]
    fn evaluate_string_and_number_expr() {
        let expression: Binary<ResultObject, Interpreter> = Binary::new(
            Box::new(Literal::new(token::Literal::from("string"))),
            Token::new(TokenKind::Plus, "+", None, 1),
            Box::new(Literal::new(token::Literal::from(10))),
        );

        let interpreter = InterpreterBuilder::new().build();

        let result = interpreter.evaluate(&expression).unwrap();

        assert_eq!(result, Object::from("string10"));
    }

    #[test]
    fn error_invalid_expr() {
        let expression: Binary<ResultObject, Interpreter> = Binary::new(
            Box::new(Literal::new(token::Literal::from("string"))),
            Token::new(TokenKind::Minus, "-", None, 1),
            Box::new(Literal::new(token::Literal::from(10))),
        );

        let interpreter = InterpreterBuilder::new().build();

        let result = interpreter.evaluate(&expression);

        assert!(result.is_err());
    }

    #[test]
    fn evaluate_grouped_expr() {
        // TODO: !false should be evaluate to true
        let grouping: Grouping<ResultObject, Interpreter> = Grouping::new(Box::new(Unary::new(
            Token::new(TokenKind::Bang, "!", None, 1),
            Box::new(Literal::new(token::Literal::from(false))),
        )));

        let interpreter = InterpreterBuilder::new().build();

        let result = interpreter.evaluate(&grouping).unwrap();

        assert_eq!(result, Object::from(false));
    }

    #[test]
    fn execute_print_complex_expr() {
        let expression: Binary<ResultObject, Interpreter> = Binary::new(
            Box::new(Literal::new(token::Literal::from(10))),
            Token::new(TokenKind::Plus, "+", None, 1),
            Box::new(Binary::new(
                Box::new(Literal::new(token::Literal::from(10))),
                Token::new(TokenKind::Star, "*", None, 1),
                Box::new(Literal::new(token::Literal::from(10))),
            )),
        );

        let statement = Print::new(Box::new(expression));

        let mut interpreter = InterpreterBuilder::new().build();

        let result = interpreter.execute(&statement).unwrap();

        assert_eq!(result, Object::from(110));
    }

    #[test]
    fn execute_expression_complex_expr() {
        let expression: Binary<ResultObject, Interpreter> = Binary::new(
            Box::new(Literal::new(token::Literal::from(10))),
            Token::new(TokenKind::Plus, "+", None, 1),
            Box::new(Binary::new(
                Box::new(Literal::new(token::Literal::from(10))),
                Token::new(TokenKind::Star, "*", None, 1),
                Box::new(Literal::new(token::Literal::from(10))),
            )),
        );

        let statement = Expression::new(Box::new(expression));

        let mut interpreter = InterpreterBuilder::new().build();

        let result = interpreter.execute(&statement).unwrap();

        assert_eq!(result, Object::from(110));
    }

    #[test]
    fn interpret_complex_expr() {
        let expression: Binary<ResultObject, Interpreter> = Binary::new(
            Box::new(Literal::new(token::Literal::from(10))),
            Token::new(TokenKind::Plus, "+", None, 1),
            Box::new(Binary::new(
                Box::new(Literal::new(token::Literal::from(10))),
                Token::new(TokenKind::Slash, "/", None, 1),
                Box::new(Literal::new(token::Literal::from(10))),
            )),
        );

        let statement = Expression::new(Box::new(expression));

        let mut interpreter = InterpreterBuilder::new().build();

        let result = interpreter.interpret(&[Box::new(statement)]).unwrap();
        let v = result.get(0).unwrap();

        assert_eq!(v, &Object::from(11));
    }

    #[test]
    fn interpret_variable_expr() {
        let expression: Binary<ResultObject, Interpreter> = Binary::new(
            Box::new(Literal::new(token::Literal::from(10))),
            Token::new(TokenKind::Plus, "+", None, 1),
            Box::new(Binary::new(
                Box::new(Literal::new(token::Literal::from(10))),
                Token::new(TokenKind::Slash, "/", None, 1),
                Box::new(Literal::new(token::Literal::from(10))),
            )),
        );

        let statement = Let::new(
            Token::new(TokenKind::Identifier, "a", None, 1),
            Box::new(expression),
        );

        let mut interpreter = InterpreterBuilder::new().build();

        let result = interpreter.interpret(&[Box::new(statement)]).unwrap();
        let v = result.get(0).unwrap();

        assert_eq!(v, &Object::from(11));
    }

    #[test]
    fn execute_print_on_assign_expr() {
        let mut interpreter = InterpreterBuilder::new().build();
        let literal: Literal<ResultObject, Interpreter> = Literal::new(token::Literal::from(2));
        let statement: Let<ResultObject, Interpreter, Interpreter> = Let::new(
            Token::new(TokenKind::Identifier, "a", None, 1),
            Box::new(literal),
        );

        interpreter.interpret(&[Box::new(statement)]).unwrap();

        let expr = Binary::new(
            Box::new(Literal::new(token::Literal::from(1))),
            Token::new(TokenKind::Plus, "+", None, 1),
            Box::new(Literal::new(token::Literal::from(2))),
        );

        let assign = Assign::new(
            Token::new(TokenKind::Identifier, "a", None, 1),
            Box::new(expr),
        );

        let statement = Print::new(Box::new(assign));

        let result = interpreter.execute(&statement).unwrap();

        assert_eq!(result, Object::from(3));
    }
}
