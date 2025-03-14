use crate::{expression::Expression, scanner::token::Token};

#[derive(Clone)]
pub struct VariableDeclaration {
    pub name: Token,
    pub initializer: Option<Expression>,
}

#[derive(Clone)]
pub struct VariableStmt {
    pub variables: Vec<VariableDeclaration>,
}
