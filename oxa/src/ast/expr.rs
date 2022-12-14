use crate::token;
use std::fmt::{Display, Formatter, Result};
use std::marker;

pub enum ExprKind<'a, T, V> {
    Assign(&'a Assign<T, V>),
    Binary(&'a Binary<T, V>),
    Grouping(&'a Grouping<T, V>),
    Literal(&'a Literal<T, V>),
    Unary(&'a Unary<T, V>),
    Variable(&'a Variable<T, V>),
}

pub trait Expr<T, V: Visitor<T>>: Display {
    fn accept(&self, visitor: &V) -> T;
    fn kind(&self) -> ExprKind<T, V>;
}

pub trait Visitor<T> {
    fn visit_assign_expr(&self, expr: &Assign<T, Self>) -> T;
    fn visit_binary_expr(&self, expr: &Binary<T, Self>) -> T;
    fn visit_grouping_expr(&self, expr: &Grouping<T, Self>) -> T;
    fn visit_literal_expr(&self, expr: &Literal<T, Self>) -> T;
    fn visit_unary_expr(&self, expr: &Unary<T, Self>) -> T;
    fn visit_variable_expr(&self, expr: &Variable<T, Self>) -> T;
}

pub struct Assign<T, V: ?Sized> {
    pub name: token::Token,
    pub value: Box<dyn Expr<T, V>>,
    _marker_1: marker::PhantomData<T>,
    _marker_2: marker::PhantomData<V>,
}

impl<T, V> Assign<T, V> {
    pub fn new(name: token::Token, value: Box<dyn Expr<T, V>>) -> Self {
        Assign {
            name,
            value,
            _marker_1: marker::PhantomData::default(),
            _marker_2: marker::PhantomData::default(),
        }
    }
}

impl<T, V: Visitor<T>> Expr<T, V> for Assign<T, V> {
    fn accept(&self, visitor: &V) -> T {
        visitor.visit_assign_expr(self)
    }

    fn kind(&self) -> ExprKind<T, V> {
        ExprKind::Assign(self)
    }
}

impl<T, V: Visitor<T>> Display for Assign<T, V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{} {}", self.name, self.value)
    }
}

pub struct Binary<T, V: ?Sized> {
    pub left: Box<dyn Expr<T, V>>,
    pub operator: token::Token,
    pub right: Box<dyn Expr<T, V>>,
    _marker_1: marker::PhantomData<T>,
    _marker_2: marker::PhantomData<V>,
}

impl<T, V> Binary<T, V> {
    pub fn new(
        left: Box<dyn Expr<T, V>>,
        operator: token::Token,
        right: Box<dyn Expr<T, V>>,
    ) -> Self {
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
        visitor.visit_binary_expr(self)
    }

    fn kind(&self) -> ExprKind<T, V> {
        ExprKind::Binary(self)
    }
}

impl<T, V: Visitor<T>> Display for Binary<T, V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{} {} {}", self.left, self.operator, self.right)
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
        visitor.visit_grouping_expr(self)
    }

    fn kind(&self) -> ExprKind<T, V> {
        ExprKind::Grouping(self)
    }
}

impl<T, V: Visitor<T>> Display for Grouping<T, V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.expression)
    }
}

pub struct Literal<T, V: ?Sized> {
    pub value: token::Literal,
    _marker_1: marker::PhantomData<T>,
    _marker_2: marker::PhantomData<V>,
}

impl<T, V> Literal<T, V> {
    pub fn new(value: token::Literal) -> Self {
        Literal {
            value,
            _marker_1: marker::PhantomData::default(),
            _marker_2: marker::PhantomData::default(),
        }
    }
}

impl<T, V: Visitor<T>> Expr<T, V> for Literal<T, V> {
    fn accept(&self, visitor: &V) -> T {
        visitor.visit_literal_expr(self)
    }

    fn kind(&self) -> ExprKind<T, V> {
        ExprKind::Literal(self)
    }
}

impl<T, V: Visitor<T>> Display for Literal<T, V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.value)
    }
}

pub struct Unary<T, V: ?Sized> {
    pub operator: token::Token,
    pub right: Box<dyn Expr<T, V>>,
    _marker_1: marker::PhantomData<T>,
    _marker_2: marker::PhantomData<V>,
}

impl<T, V> Unary<T, V> {
    pub fn new(operator: token::Token, right: Box<dyn Expr<T, V>>) -> Self {
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
        visitor.visit_unary_expr(self)
    }

    fn kind(&self) -> ExprKind<T, V> {
        ExprKind::Unary(self)
    }
}

impl<T, V: Visitor<T>> Display for Unary<T, V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{} {}", self.operator, self.right)
    }
}

pub struct Variable<T, V: ?Sized> {
    pub name: token::Token,
    _marker_1: marker::PhantomData<T>,
    _marker_2: marker::PhantomData<V>,
}

impl<T, V> Variable<T, V> {
    pub fn new(name: token::Token) -> Self {
        Variable {
            name,
            _marker_1: marker::PhantomData::default(),
            _marker_2: marker::PhantomData::default(),
        }
    }
}

impl<T, V: Visitor<T>> Expr<T, V> for Variable<T, V> {
    fn accept(&self, visitor: &V) -> T {
        visitor.visit_variable_expr(self)
    }

    fn kind(&self) -> ExprKind<T, V> {
        ExprKind::Variable(self)
    }
}

impl<T, V: Visitor<T>> Display for Variable<T, V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.name)
    }
}
