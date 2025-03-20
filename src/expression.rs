use crate::{stmt::Stmt, Object, ResolvableFunction, Token};

#[derive(Clone)]
pub enum Expression {
    Comma(Box<Comma>),
    Lambda(Box<Lambda>),
    Assignment(Box<Assignment>),
    Ternary(Box<Ternary>),
    Logical(Box<Logical>),
    Set(Box<Set>),
    Binary(Box<Binary>),
    Unary(Box<Unary>),
    Call(Box<Call>),
    Get(Box<Get>),
    Grouping(Box<Expression>),
    Variable(Box<Token>),
    This(Box<Token>),
    Literal(Box<Object>),
}

#[derive(Clone)]
pub struct Comma {
    pub left: Expression,
    pub right: Expression,
}

#[derive(Clone)]
pub struct Lambda {
    pub name: Token,
    pub params: Vec<Token>,
    pub body: Vec<Stmt>,
}

#[derive(Clone)]
pub struct Assignment {
    pub name: Token,
    pub value: Expression,
}

#[derive(Clone)]
pub struct Ternary {
    pub condition: Expression,
    pub truth: Expression,
    pub falsy: Expression,
}

#[derive(Clone)]
pub struct Logical {
    pub operator: Token,
    pub left: Expression,
    pub right: Expression,
}

#[derive(Clone)]
pub struct Set {
    pub object: Expression,
    pub name: Token,
    pub value: Expression,
}

#[derive(Clone)]
pub struct Binary {
    pub left: Expression,
    pub operator: Token,
    pub right: Expression,
}

#[derive(Clone)]
pub struct Unary {
    pub operator: Token,
    pub right: Expression,
}

#[derive(Clone)]
pub struct Call {
    pub callee: Expression,
    pub arguments: Vec<Expression>,
    pub paren: Token,
}

#[derive(Clone)]
pub struct Get {
    pub object: Expression,
    pub name: Token,
}

impl ResolvableFunction for Lambda {
    fn params(&self) -> &Vec<Token> {
        &self.params
    }
    fn body(&self) -> &Vec<Stmt> {
        &self.body
    }

    fn name(&self) -> &str {
        "anonymous"
    }
}

pub trait ExpressionVisitor {
    type Item;

    fn visit_comma(&mut self, expr: &Comma) -> Self::Item;
    fn visit_lambda(&mut self, expr: &Lambda) -> Self::Item;
    fn visit_assignment(&mut self, expr: &Assignment) -> Self::Item;
    fn visit_ternary(&mut self, expr: &Ternary) -> Self::Item;
    fn visit_logical(&mut self, expr: &Logical) -> Self::Item;
    fn visit_set(&mut self, expr: &Set) -> Self::Item;
    fn visit_binary(&mut self, expr: &Binary) -> Self::Item;
    fn visit_unary(&mut self, expr: &Unary) -> Self::Item;
    fn visit_call(&mut self, expr: &Call) -> Self::Item;
    fn visit_get(&mut self, expr: &Get) -> Self::Item;
    fn visit_grouping(&mut self, expr: &Expression) -> Self::Item;
    fn visit_variable(&mut self, expr: &Token) -> Self::Item;
    fn visit_this(&mut self, expr: &Token) -> Self::Item;
    fn visit_literal(&mut self, expr: &Object) -> Self::Item;
}

impl Expression {
    pub fn accept<V: ExpressionVisitor>(&self, visitor: &mut V) -> V::Item {
        match self {
            Expression::Comma(comma) => visitor.visit_comma(comma),
            Expression::Lambda(lambda) => visitor.visit_lambda(lambda),
            Expression::Assignment(assignment) => visitor.visit_assignment(assignment),
            Expression::Ternary(ternary) => visitor.visit_ternary(ternary),
            Expression::Logical(logical) => visitor.visit_logical(logical),
            Expression::Set(set) => visitor.visit_set(set),
            Expression::Binary(binary) => visitor.visit_binary(binary),
            Expression::Unary(unary) => visitor.visit_unary(unary),
            Expression::Call(call) => visitor.visit_call(call),
            Expression::Get(get) => visitor.visit_get(get),
            Expression::Grouping(grouping) => visitor.visit_grouping(grouping),
            Expression::Variable(variable) => visitor.visit_variable(variable),
            Expression::This(token) => visitor.visit_this(token),
            Expression::Literal(literal) => visitor.visit_literal(literal),
        }
    }
}
