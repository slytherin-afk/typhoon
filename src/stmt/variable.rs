use crate::{expr::Expr, token::Token};

#[derive(Clone)]
pub struct VariableDeclaration {
    pub name: Token,
    pub initializer: Option<Expr>,
}
