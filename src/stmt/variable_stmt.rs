use crate::{expression::Expression, scanner::token::Token};

pub struct VariableStmt<'a> {
    pub name: &'a Token,
    pub initializer: Option<Expression<'a>>,
}
