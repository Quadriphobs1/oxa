use crate::token::Token;
use std::marker;

pub trait Expr<T, V: Visitor<T>> {
    fn accept(&self, visitor: &V) -> T;
}

pub trait Visitor<T> {
    fn visit_binary_expr(&self, expr: &Binary<T, Self>) -> T;
    fn visit_grouping_expr(&self, expr: &Grouping<T, Self>) -> T;
    fn visit_literal_expr(&self, expr: &Literal<T, Self>) -> T;
    fn visit_unary_expr(&self, expr: &Unary<T, Self>) -> T;
}

pub struct Binary<T, V: ?Sized> {
    pub left: Box<dyn Expr<T, V>>,
    pub operator: Token,
    pub right: Box<dyn Expr<T, V>>,
    _marker_1: marker::PhantomData<T>,
    _marker_2: marker::PhantomData<V>,
}

impl<T, V> Binary<T, V> {
    pub fn new(left: Box<dyn Expr<T, V>>, operator: Token, right: Box<dyn Expr<T, V>>) -> Self {
        Binary {
            left,
            operator,
            right,
            _marker_1: marker::PhantomData::default(),
            _marker_2: marker::PhantomData::default(),
        }
    }
}

impl<T, V: Visitor<T>> Expr<T, V> for Binary<T, V> {
    fn accept(&self, visitor: &V) -> T {
        return visitor.visit_binary_expr(self);
    }
}

pub struct Grouping<T, V: ?Sized> {
    pub expression: Box<dyn Expr<T, V>>,
    _marker_1: marker::PhantomData<T>,
    _marker_2: marker::PhantomData<V>,
}

impl<T, V> Grouping<T, V> {
    pub fn new(expression: Box<dyn Expr<T, V>>) -> Self {
        Grouping {
            expression,
            _marker_1: marker::PhantomData::default(),
            _marker_2: marker::PhantomData::default(),
        }
    }
}

impl<T, V: Visitor<T>> Expr<T, V> for Grouping<T, V> {
    fn accept(&self, visitor: &V) -> T {
        return visitor.visit_grouping_expr(self);
    }
}

pub struct Literal<T, V: ?Sized> {
    pub value: String,
    _marker_1: marker::PhantomData<T>,
    _marker_2: marker::PhantomData<V>,
}

impl<T, V> Literal<T, V> {
    pub fn new(value: String) -> Self {
        Literal {
            value,
            _marker_1: marker::PhantomData::default(),
            _marker_2: marker::PhantomData::default(),
        }
    }
}

impl<T, V: Visitor<T>> Expr<T, V> for Literal<T, V> {
    fn accept(&self, visitor: &V) -> T {
        return visitor.visit_literal_expr(self);
    }
}

pub struct Unary<T, V: ?Sized> {
    pub operator: Token,
    pub right: Box<dyn Expr<T, V>>,
    _marker_1: marker::PhantomData<T>,
    _marker_2: marker::PhantomData<V>,
}

impl<T, V> Unary<T, V> {
    pub fn new(operator: Token, right: Box<dyn Expr<T, V>>) -> Self {
        Unary {
            operator,
            right,
            _marker_1: marker::PhantomData::default(),
            _marker_2: marker::PhantomData::default(),
        }
    }
}

impl<T, V: Visitor<T>> Expr<T, V> for Unary<T, V> {
    fn accept(&self, visitor: &V) -> T {
        return visitor.visit_unary_expr(self);
    }
}
