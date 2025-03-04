pub mod binary;
pub mod grouping;
pub mod literal;
pub mod ternary;
pub mod tree_printer;
pub mod unary;

use binary::Binary;
use grouping::Grouping;
use literal::Literal;
use ternary::Ternary;
use unary::Unary;

pub enum Expression<'a> {
    Binary(Box<Binary<'a>>),
    Grouping(Box<Grouping<'a>>),
    Literal(Box<Literal<'a>>),
    Unary(Box<Unary<'a>>),
    Ternary(Box<Ternary<'a>>),
}

pub trait ExpressionVisitor<T> {
    fn visit_binary(&self, expr: &mut Binary) -> T;
    fn visit_grouping(&self, expr: &mut Grouping) -> T;
    fn visit_literal(&self, expr: &mut Literal) -> T;
    fn visit_unary(&self, expr: &mut Unary) -> T;
    fn visit_ternary(&self, expr: &mut Ternary) -> T;
}

impl<'a> Expression<'a> {
    pub fn accept<T, V: ExpressionVisitor<T>>(&mut self, visitor: &V) -> T {
        match self {
            Expression::Binary(binary) => visitor.visit_binary(binary),
            Expression::Grouping(grouping) => visitor.visit_grouping(grouping),
            Expression::Literal(literal) => visitor.visit_literal(literal),
            Expression::Unary(unary) => visitor.visit_unary(unary),
            Expression::Ternary(ternary) => visitor.visit_ternary(ternary),
        }
    }
}
