use super::{
    binary::Binary, grouping::Grouping, literal::Literal, unary::Unary, Expression,
    ExpressionVisitor,
};

pub struct TreePrinter;

impl TreePrinter {
    pub fn print(expression: &mut Expression) -> String {
        let visitor = TreePrinter;
        expression.accept(&visitor)
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
        expr.value.to_string()
    }

    fn visit_unary(&self, expr: &mut Unary) -> String {
        Self::parenthesize(self, &expr.operator.lexeme, vec![&mut expr.right])
    }
}
