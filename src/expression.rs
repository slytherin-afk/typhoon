pub mod assignment;
pub mod binary;
pub mod comma;
pub mod grouping;
pub mod literal;
pub mod ternary;
pub mod unary;
pub mod variable;

use assignment::Assignment;
use binary::Binary;
use comma::Comma;
use grouping::Grouping;
use literal::Literal;
use ternary::Ternary;
use unary::Unary;
use variable::Variable;

pub enum Expression<'a> {
    Comma(Box<Comma<'a>>),
    Ternary(Box<Ternary<'a>>),
    Binary(Box<Binary<'a>>),
    Unary(Box<Unary<'a>>),
    Grouping(Box<Grouping<'a>>),
    Literal(Box<Literal>),
    Variable(Box<Variable<'a>>),
    Assignment(Box<Assignment<'a>>),
}
