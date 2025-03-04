use super::{
    binary::Binary,
    grouping::Grouping,
    literal::{Literal, LiteralValue},
    unary::Unary,
    Expression, ExpressionVisitor,
};

pub struct TreePrinter;

impl TreePrinter {
    pub fn print(expression: &mut Expression) -> String {
        expression.accept(&TreePrinter)
    }

    fn parenthesize<T: ExpressionVisitor<String>>(
        visitor: &T,
        name: &str,
        expressions: Vec<&mut Expression>,
    ) -> String {
        let mut builder = format!("({name}");

        expressions.into_iter().for_each(|ex| {
            builder += " ";
            builder += &ex.accept(visitor);
        });

        builder.push(')');
        builder
    }
}

impl ExpressionVisitor<String> for TreePrinter {
    fn visit_binary(&self, expr: &mut Binary) -> String {
        Self::parenthesize(
            self,
            &expr.operator.lexeme,
            vec![&mut expr.left, &mut expr.right],
        )
    }

    fn visit_grouping(&self, expr: &mut Grouping) -> String {
        Self::parenthesize(self, "group", vec![&mut expr.expression])
    }

    fn visit_literal(&self, expr: &mut Literal) -> String {
        match expr.value {
            LiteralValue::True => "True".to_string(),
            LiteralValue::False => "String".to_string(),
            LiteralValue::Number(number) => number.to_string(),
            LiteralValue::String(value) => value.to_string(),
            LiteralValue::None => "None".to_string(),
        }
    }

    fn visit_unary(&self, expr: &mut Unary) -> String {
        Self::parenthesize(self, &expr.operator.lexeme, vec![&mut expr.right])
    }
}
