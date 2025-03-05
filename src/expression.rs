pub mod binary;
pub mod comma;
pub mod grouping;
pub mod literal;
pub mod pretty_tree_printer;
pub mod ternary;
pub mod tree_printer;
pub mod unary;

use binary::Binary;
use comma::Comma;
use grouping::Grouping;
use literal::Literal;
use ternary::Ternary;
use unary::Unary;

pub enum Expression<'a> {
    Comma(Box<Comma<'a>>),
    Ternary(Box<Ternary<'a>>),
    Binary(Box<Binary<'a>>),
    Unary(Box<Unary<'a>>),
    Grouping(Box<Grouping<'a>>),
    Literal(Box<Literal<'a>>),
}

pub trait ExpressionVisitor<T> {
    fn visit_comma(&self, expr: &mut Comma) -> T;
    fn visit_ternary(&self, expr: &mut Ternary) -> T;
    fn visit_binary(&self, expr: &mut Binary) -> T;
    fn visit_unary(&self, expr: &mut Unary) -> T;
    fn visit_grouping(&self, expr: &mut Grouping) -> T;
    fn visit_literal(&self, expr: &mut Literal) -> T;
}

impl<'a> Expression<'a> {
    pub fn accept<T, V: ExpressionVisitor<T>>(&mut self, visitor: &V) -> T {
        match self {
            Expression::Comma(comma) => visitor.visit_comma(comma),
            Expression::Ternary(ternary) => visitor.visit_ternary(ternary),
            Expression::Binary(binary) => visitor.visit_binary(binary),
            Expression::Unary(unary) => visitor.visit_unary(unary),
            Expression::Grouping(grouping) => visitor.visit_grouping(grouping),
            Expression::Literal(literal) => visitor.visit_literal(literal),
        }
    }
}
