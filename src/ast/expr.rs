use crate::token::Token;

pub trait Expr {
    fn accept(&self, visitor: Visitor) {}
}

pub struct Visitor {}
impl Visitor {
  pub fn visit_binary_expr(&self, expr: &Binary) {}
  pub fn visit_grouping_expr(&self, expr: &Grouping) {}
  pub fn visit_literal_expr(&self, expr: &Literal) {}
  pub fn visit_unary_expr(&self, expr: &Unary) {}
}

pub struct Binary {
    left: Box<dyn Expr + 'static>,
    operator: Token,
    right: Box<dyn Expr + 'static>,
}

impl Expr for Binary {
    fn accept(&self, visitor: Visitor) {
        return visitor.visit_binary_expr(self);
    }
}

pub struct Grouping {
    expression: Box<dyn Expr + 'static>,
}

impl Expr for Grouping {
    fn accept(&self, visitor: Visitor) {
        return visitor.visit_grouping_expr(self);
    }
}

pub struct Literal {
    value: String,
}

impl Expr for Literal {
    fn accept(&self, visitor: Visitor) {
        return visitor.visit_literal_expr(self);
    }
}

pub struct Unary {
    operator: Token,
    right: Box<dyn Expr + 'static>,
}

impl Expr for Unary {
    fn accept(&self, visitor: Visitor) {
        return visitor.visit_unary_expr(self);
    }
}


