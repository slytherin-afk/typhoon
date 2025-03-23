mod _if;
mod _return;
mod _while;
mod class;
mod function;
mod variable;

pub use _if::If;
pub use _return::Return;
pub use _while::While;
pub use class::Class;
pub use function::Function;
pub use variable::VariableDeclaration;

use crate::{expr::Expr, token::Token};

#[derive(Clone)]
pub enum Stmt {
    Empty,
    Expression(Box<Expr>),
    Print(Box<Expr>),
    Variable(Box<Vec<VariableDeclaration>>),
    Block(Box<Vec<Stmt>>),
    If(Box<If>),
    While(Box<While>),
    Break(Token),
    Continue(Token),
    Function(Box<Function>),
    Return(Box<Return>),
    Class(Box<Class>),
}

pub trait StmtVisitor {
    type Item;

    fn visit_empty_stmt(&mut self) -> Self::Item;
    fn visit_expression_stmt(&mut self, stmt: &Expr) -> Self::Item;
    fn visit_print_stmt(&mut self, stmt: &Expr) -> Self::Item;
    fn visit_variable_stmt(&mut self, stmt: &Vec<VariableDeclaration>) -> Self::Item;
    fn visit_block_stmt(&mut self, stmt: &Vec<Stmt>) -> Self::Item;
    fn visit_if_stmt(&mut self, stmt: &If) -> Self::Item;
    fn visit_while_stmt(&mut self, stmt: &While) -> Self::Item;
    fn visit_break_stmt(&mut self, keyword: &Token) -> Self::Item;
    fn visit_continue_stmt(&mut self, keyword: &Token) -> Self::Item;
    fn visit_function_stmt(&mut self, stmt: &Function) -> Self::Item;
    fn visit_return_stmt(&mut self, stmt: &Return) -> Self::Item;
    fn visit_class_stmt(&mut self, stmt: &Class) -> Self::Item;
}

impl Stmt {
    pub fn accept<V: StmtVisitor>(&self, visitor: &mut V) -> V::Item {
        match self {
            Stmt::Empty => visitor.visit_empty_stmt(),
            Stmt::Expression(stmt) => visitor.visit_expression_stmt(stmt),
            Stmt::Print(stmt) => visitor.visit_print_stmt(stmt),
            Stmt::Variable(stmt) => visitor.visit_variable_stmt(stmt),
            Stmt::Block(stmt) => visitor.visit_block_stmt(stmt),
            Stmt::If(stmt) => visitor.visit_if_stmt(stmt),
            Stmt::While(stmt) => visitor.visit_while_stmt(stmt),
            Stmt::Break(stmt) => visitor.visit_break_stmt(stmt),
            Stmt::Continue(stmt) => visitor.visit_continue_stmt(stmt),
            Stmt::Function(stmt) => visitor.visit_function_stmt(stmt),
            Stmt::Return(stmt) => visitor.visit_return_stmt(stmt),
            Stmt::Class(stmt) => visitor.visit_class_stmt(stmt),
        }
    }
}
