mod _super;
mod assignment;
mod binary;
mod call;
mod comma;
mod get;
mod lambda;
mod logical;
mod set;
mod ternary;
mod unary;

pub use _super::Super;
pub use assignment::Assignment;
pub use binary::Binary;
pub use call::Call;
pub use comma::Comma;
pub use get::Get;
pub use lambda::Lambda;
pub use logical::Logical;
pub use set::Set;
pub use ternary::Ternary;
pub use unary::Unary;

use crate::{object::Object, token::Token};

#[derive(Clone)]
pub enum Expr {
    Comma(Box<Comma>),
    Lambda(Box<Lambda>),
    Assignment(Box<Assignment>),
    Set(Box<Set>),
    Ternary(Box<Ternary>),
    Logical(Box<Logical>),
    Binary(Box<Binary>),
    Unary(Box<Unary>),
    Call(Box<Call>),
    Get(Box<Get>),
    Grouping(Box<Expr>),
    Variable(Box<Token>),
    This(Box<Token>),
    Super(Box<Super>),
    Literal(Box<Object>),
}

pub trait ExprVisitor {
    type Item;

    fn visit_comma(&mut self, expr: &Comma) -> Self::Item;
    fn visit_lambda(&mut self, expr: &Lambda) -> Self::Item;
    fn visit_assignment(&mut self, expr: &Assignment) -> Self::Item;
    fn visit_set(&mut self, expr: &Set) -> Self::Item;
    fn visit_ternary(&mut self, expr: &Ternary) -> Self::Item;
    fn visit_logical(&mut self, expr: &Logical) -> Self::Item;
    fn visit_binary(&mut self, expr: &Binary) -> Self::Item;
    fn visit_unary(&mut self, expr: &Unary) -> Self::Item;
    fn visit_call(&mut self, expr: &Call) -> Self::Item;
    fn visit_get(&mut self, expr: &Get) -> Self::Item;
    fn visit_grouping(&mut self, expr: &Expr) -> Self::Item;
    fn visit_variable(&mut self, expr: &Token) -> Self::Item;
    fn visit_this(&mut self, expr: &Token) -> Self::Item;
    fn visit_super(&mut self, expr: &Super) -> Self::Item;
    fn visit_literal(&mut self, expr: &Object) -> Self::Item;
}

impl Expr {
    pub fn accept<V: ExprVisitor>(&self, visitor: &mut V) -> V::Item {
        match self {
            Expr::Comma(expr) => visitor.visit_comma(expr),
            Expr::Lambda(expr) => visitor.visit_lambda(expr),
            Expr::Assignment(expr) => visitor.visit_assignment(expr),
            Expr::Set(expr) => visitor.visit_set(expr),
            Expr::Ternary(expr) => visitor.visit_ternary(expr),
            Expr::Logical(expr) => visitor.visit_logical(expr),
            Expr::Binary(expr) => visitor.visit_binary(expr),
            Expr::Unary(expr) => visitor.visit_unary(expr),
            Expr::Call(expr) => visitor.visit_call(expr),
            Expr::Get(expr) => visitor.visit_get(expr),
            Expr::Grouping(expr) => visitor.visit_grouping(expr),
            Expr::Variable(expr) => visitor.visit_variable(expr),
            Expr::This(expr) => visitor.visit_this(expr),
            Expr::Super(expr) => visitor.visit_super(expr),
            Expr::Literal(expr) => visitor.visit_literal(expr),
        }
    }
}
