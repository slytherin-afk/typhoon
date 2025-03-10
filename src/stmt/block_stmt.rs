use super::Stmt;

pub struct BlockStmt<'a> {
    pub stmts: Vec<Stmt<'a>>,
}
