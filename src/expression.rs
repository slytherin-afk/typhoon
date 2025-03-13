pub mod assignment;
pub mod binary;
pub mod call;
pub mod comma;
pub mod grouping;
pub mod literal;
pub mod logical;
pub mod ternary;
pub mod unary;
pub mod variable;

use assignment::Assignment;
use binary::Binary;
use call::Call;
use comma::Comma;
use grouping::Grouping;
use literal::Literal;
use logical::Logical;
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
    Logical(Box<Logical<'a>>),
    Call(Box<Call<'a>>),
}
