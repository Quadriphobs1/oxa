use crate::ast::expr::Expr;
use std::fmt::{Display, Formatter, Result};
use std::marker;

pub trait Stmt<T, U: Visitor<T, V>, V>: Display {
    fn accept(&self, visitor: &U) -> T;
}

pub trait Visitor<T, V> {
    fn visit_expression_stmt(&self, stmt: &Expression<T, Self, V>) -> T;
    fn visit_print_stmt(&self, stmt: &Print<T, Self, V>) -> T;
}

pub struct Expression<T, U: ?Sized, V: ?Sized> {
    pub expression: Box<dyn Expr<T, V>>,
    _marker_1: marker::PhantomData<T>,
    _marker_2: marker::PhantomData<U>,
    _marker_3: marker::PhantomData<V>,
}

impl<T, U, V> Expression<T, U, V> {
    pub fn new(expression: Box<dyn Expr<T, V>>) -> Self {
        Expression {
            expression,
            _marker_1: marker::PhantomData::default(),
            _marker_3: marker::PhantomData::default(),
            _marker_2: marker::PhantomData::default(),
        }
    }
}

impl<T, U: Visitor<T, V>, V> Stmt<T, U, V> for Expression<T, U, V> {
    fn accept(&self, visitor: &U) -> T {
        visitor.visit_expression_stmt(self)
    }
}

impl<T, U: Visitor<T, V>, V> Display for Expression<T, U, V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.expression)
    }
}

pub struct Print<T, U: ?Sized, V: ?Sized> {
    pub expression: Box<dyn Expr<T, V>>,
    _marker_1: marker::PhantomData<T>,
    _marker_3: marker::PhantomData<U>,
    _marker_2: marker::PhantomData<V>,
}

impl<T, U, V> Print<T, U, V> {
    pub fn new(expression: Box<dyn Expr<T, V>>) -> Self {
        Print {
            expression,
            _marker_1: marker::PhantomData::default(),
            _marker_3: marker::PhantomData::default(),
            _marker_2: marker::PhantomData::default(),
        }
    }
}

impl<T, U: Visitor<T, V>, V> Stmt<T, U, V> for Print<T, U, V> {
    fn accept(&self, visitor: &U) -> T {
        visitor.visit_print_stmt(self)
    }
}

impl<T, U: Visitor<T, V>, V> Display for Print<T, U, V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.expression)
    }
}
