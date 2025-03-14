use super::Stmt;

#[derive(Clone)]
pub struct BlockStmt {
    pub stmts: Vec<Stmt>,
}
