use crate::{expression::Expression, scanner::token::Token};

pub struct VariableDeclaration<'a> {
    pub name: &'a Token,
    pub initializer: Option<Expression<'a>>,
}

pub struct VariableStmt<'a> {
    pub variables: Vec<VariableDeclaration<'a>>,
}
