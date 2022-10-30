pub mod ast;
pub mod error;
pub mod oxa;
pub mod scanner;
pub mod token;

pub mod parser;
mod reporter;

// pub trait Expr<T, V: Visitor<T>>: PartialEq {
//     fn accept(&self, visitor: &V) -> T;
// }
//
// pub struct Literal<T, V: ?Sized> {
//     pub value: token::Literal,
//     _marker_1: marker::PhantomData<T>,
//     _marker_2: marker::PhantomData<V>,
// }
//
// impl<T, V> Literal<T, V> {
//     pub fn new(value: token::Literal) -> Self {
//         Literal {
//             value,
//             _marker_1: marker::PhantomData::default(),
//             _marker_2: marker::PhantomData::default(),
//         }
//     }
// }
//
// impl<T, V> PartialEq<Literal<T, V>> for Literal<T, V> {
//     fn eq(&self, other: &Literal<T, V>) -> bool {
//         self.value == other.value
//     }
// }
//
// impl<T, V: Visitor<T>> Expr<T, V> for Literal<T, V> {
//     fn accept(&self, visitor: &V) -> T {
//         return visitor.visit_literal_expr(self);
//     }
// }
//
// fn text_lit() {
//     let literalA = Literal::new(token::Literal::from(1));
//     let literalB = Literal::new(token::Literal::from(1));
//
//     let c = literalA == literalB;
// }
