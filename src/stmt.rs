use crate::{expression::Expression, ResolvableFunction, Token};

#[derive(Clone)]
pub enum Stmt {
    EmptyStmt,
    ExpressionStmt(Box<ExpressionStmt>),
    PrintStmt(Box<PrintStmt>),
    VariableStmt(Box<VariableStmt>),
    BlockStmt(Box<BlockStmt>),
    IfStmt(Box<IfStmt>),
    WhileStmt(Box<WhileStmt>),
    BreakStmt(Token),
    ContinueStmt(Token),
    FunctionStmt(Box<FunctionStmt>),
    ReturnStmt(Box<ReturnStmt>),
}

#[derive(Clone)]
pub struct ExpressionStmt {
    pub value: Expression,
}

#[derive(Clone)]
pub struct PrintStmt {
    pub value: Expression,
}

#[derive(Clone)]
pub struct VariableDeclaration {
    pub name: Token,
    pub initializer: Option<Expression>,
}

#[derive(Clone)]
pub struct VariableStmt {
    pub stmts: Vec<VariableDeclaration>,
}

#[derive(Clone)]
pub struct BlockStmt {
    pub stmts: Vec<Stmt>,
}

#[derive(Clone)]
pub struct IfStmt {
    pub condition: Expression,
    pub truth: Stmt,
    pub falsy: Option<Stmt>,
}

#[derive(Clone)]
pub struct WhileStmt {
    pub condition: Expression,
    pub body: Stmt,
}

#[derive(Clone)]
pub struct FunctionStmt {
    pub name: Token,
    pub params: Vec<Token>,
    pub body: Vec<Stmt>,
}

#[derive(Clone)]
pub struct ReturnStmt {
    pub keyword: Token,
    pub value: Option<Expression>,
}

impl ResolvableFunction for FunctionStmt {
    fn params(&self) -> &Vec<Token> {
        &self.params
    }
    fn body(&self) -> &Vec<Stmt> {
        &self.body
    }

    fn name(&self) -> &str {
        &self.name.lexeme
    }
}

pub trait StmtVisitor {
    type Item;

    fn visit_empty_stmt(&mut self) -> Self::Item;
    fn visit_expression_stmt(&mut self, stmt: &ExpressionStmt) -> Self::Item;
    fn visit_print_stmt(&mut self, stmt: &PrintStmt) -> Self::Item;
    fn visit_variable_stmt(&mut self, stmt: &VariableStmt) -> Self::Item;
    fn visit_block_stmt(&mut self, stmt: &BlockStmt) -> Self::Item;
    fn visit_if_stmt(&mut self, stmt: &IfStmt) -> Self::Item;
    fn visit_while_stmt(&mut self, stmt: &WhileStmt) -> Self::Item;
    fn visit_break_stmt(&mut self, keyword: &Token) -> Self::Item;
    fn visit_continue_stmt(&mut self, keyword: &Token) -> Self::Item;
    fn visit_function_stmt(&mut self, stmt: &FunctionStmt) -> Self::Item;
    fn visit_return_stmt(&mut self, stmt: &ReturnStmt) -> Self::Item;
}

impl Stmt {
    pub fn accept<V: StmtVisitor>(&self, visitor: &mut V) -> V::Item {
        match self {
            Stmt::EmptyStmt => visitor.visit_empty_stmt(),
            Stmt::ExpressionStmt(expression_stmt) => visitor.visit_expression_stmt(expression_stmt),
            Stmt::PrintStmt(print_stmt) => visitor.visit_print_stmt(print_stmt),
            Stmt::VariableStmt(variable_stmt) => visitor.visit_variable_stmt(variable_stmt),
            Stmt::BlockStmt(block_stmt) => visitor.visit_block_stmt(block_stmt),
            Stmt::IfStmt(if_stmt) => visitor.visit_if_stmt(if_stmt),
            Stmt::WhileStmt(while_stmt) => visitor.visit_while_stmt(while_stmt),
            Stmt::BreakStmt(keyword) => visitor.visit_break_stmt(keyword),
            Stmt::ContinueStmt(keyword) => visitor.visit_continue_stmt(keyword),
            Stmt::FunctionStmt(function_stmt) => visitor.visit_function_stmt(function_stmt),
            Stmt::ReturnStmt(return_stmt) => visitor.visit_return_stmt(return_stmt),
        }
    }
}
