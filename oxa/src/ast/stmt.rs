use crate::ast::expr::Expr;
use crate::token;
use std::fmt::{Display, Formatter, Result};
use std::marker;

pub trait Stmt<T, U: Visitor<T, V>, V>: Display {
    fn accept(&self, visitor: &mut U) -> T;
}

pub trait Visitor<T, V> {
    fn visit_expression_stmt(&mut self, stmt: &Expression<T, Self, V>) -> T;
    fn visit_print_stmt(&mut self, stmt: &Print<T, Self, V>) -> T;
    fn visit_let_stmt(&mut self, stmt: &Let<T, Self, V>) -> T;
    fn visit_const_stmt(&mut self, stmt: &Const<T, Self, V>) -> T;
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
            _marker_2: marker::PhantomData::default(),
            _marker_3: marker::PhantomData::default(),
        }
    }
}

impl<T, U: Visitor<T, V>, V> Stmt<T, U, V> for Expression<T, U, V> {
    fn accept(&self, visitor: &mut U) -> T {
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
    _marker_2: marker::PhantomData<U>,
    _marker_3: marker::PhantomData<V>,
}

impl<T, U, V> Print<T, U, V> {
    pub fn new(expression: Box<dyn Expr<T, V>>) -> Self {
        Print {
            expression,
            _marker_1: marker::PhantomData::default(),
            _marker_2: marker::PhantomData::default(),
            _marker_3: marker::PhantomData::default(),
        }
    }
}

impl<T, U: Visitor<T, V>, V> Stmt<T, U, V> for Print<T, U, V> {
    fn accept(&self, visitor: &mut U) -> T {
        visitor.visit_print_stmt(self)
    }
}

impl<T, U: Visitor<T, V>, V> Display for Print<T, U, V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.expression)
    }
}

pub struct Let<T, U: ?Sized, V: ?Sized> {
    pub name: token::Token,
    pub initializer: Box<dyn Expr<T, V>>,
    _marker_1: marker::PhantomData<T>,
    _marker_2: marker::PhantomData<U>,
    _marker_3: marker::PhantomData<V>,
}

impl<T, U, V> Let<T, U, V> {
    pub fn new(name: token::Token, initializer: Box<dyn Expr<T, V>>) -> Self {
        Let {
            name,
            initializer,
            _marker_1: marker::PhantomData::default(),
            _marker_2: marker::PhantomData::default(),
            _marker_3: marker::PhantomData::default(),
        }
    }
}

impl<T, U: Visitor<T, V>, V> Stmt<T, U, V> for Let<T, U, V> {
    fn accept(&self, visitor: &mut U) -> T {
        visitor.visit_let_stmt(self)
    }
}

impl<T, U: Visitor<T, V>, V> Display for Let<T, U, V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{} {}", self.name, self.initializer)
    }
}

pub struct Const<T, U: ?Sized, V: ?Sized> {
    pub name: token::Token,
    pub initializer: Box<dyn Expr<T, V>>,
    _marker_1: marker::PhantomData<T>,
    _marker_2: marker::PhantomData<U>,
    _marker_3: marker::PhantomData<V>,
}

impl<T, U, V> Const<T, U, V> {
    pub fn new(name: token::Token, initializer: Box<dyn Expr<T, V>>) -> Self {
        Const {
            name,
            initializer,
            _marker_1: marker::PhantomData::default(),
            _marker_2: marker::PhantomData::default(),
            _marker_3: marker::PhantomData::default(),
        }
    }
}

impl<T, U: Visitor<T, V>, V> Stmt<T, U, V> for Const<T, U, V> {
    fn accept(&self, visitor: &mut U) -> T {
        visitor.visit_const_stmt(self)
    }
}

impl<T, U: Visitor<T, V>, V> Display for Const<T, U, V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{} {}", self.name, self.initializer)
    }
}
