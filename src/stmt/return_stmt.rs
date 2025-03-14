use crate::{expression::Expression, scanner::token::Token};

#[derive(Clone)]
pub struct ReturnStmt {
    pub keyword: Token,
    pub value: Option<Expression>,
}
