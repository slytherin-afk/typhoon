use crate::{expression::Expression, scanner::token::Token};

pub struct VariableStmt<'a> {
    pub names: Vec<&'a Token>,
    pub initializer: Option<Expression<'a>>,
}
