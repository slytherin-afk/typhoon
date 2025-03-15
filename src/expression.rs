pub mod assignment;
pub mod binary;
pub mod call;
pub mod comma;
pub mod grouping;
pub mod lambda;
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
use lambda::Lambda;
use literal::Literal;
use logical::Logical;
use ternary::Ternary;
use unary::Unary;
use variable::Variable;

#[derive(Clone)]
pub enum Expression {
    Comma(Box<Comma>),
    Ternary(Box<Ternary>),
    Binary(Box<Binary>),
    Unary(Box<Unary>),
    Grouping(Box<Grouping>),
    Literal(Box<Literal>),
    Variable(Box<Variable>),
    Assignment(Box<Assignment>),
    Logical(Box<Logical>),
    Call(Box<Call>),
    Lambda(Box<Lambda>),
}
