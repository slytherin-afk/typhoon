pub mod binary;
pub mod comma;
pub mod grouping;
pub mod literal;
pub mod ternary;
pub mod unary;
pub mod visitor_ast_printer;
pub mod visitor_interpreter;

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
    Literal(Box<Literal>),
}

trait ExpressionVisitor {
    type Item;

    fn visit_comma(&self, expr: &mut Comma) -> Self::Item;
    fn visit_ternary(&self, expr: &mut Ternary) -> Self::Item;
    fn visit_binary(&self, expr: &mut Binary) -> Self::Item;
    fn visit_unary(&self, expr: &mut Unary) -> Self::Item;
    fn visit_grouping(&self, expr: &mut Grouping) -> Self::Item;
    fn visit_literal(&self, expr: &mut Literal) -> Self::Item;
}

impl<'a> Expression<'a> {
    fn accept<V: ExpressionVisitor>(&mut self, visitor: &V) -> V::Item {
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
