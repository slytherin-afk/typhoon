use super::{
    binary::Binary,
    comma::Comma,
    grouping::Grouping,
    literal::{Literal, LiteralValue},
    ternary::Ternary,
    unary::Unary,
    Expression, ExpressionVisitor,
};

struct TreeNode {
    name: String,
    children: Vec<TreeNode>,
}

pub struct PrettyTreePrinter;

impl PrettyTreePrinter {
    pub fn print(expression: &mut Expression) -> String {
        let tree = expression.accept(&Self);
        Self::print_tree(&tree, &String::new(), true)
    }

    fn print_tree(node: &TreeNode, prefix: &str, is_last: bool) -> String {
        let mut output = String::new();
        let branch = if is_last { "└── " } else { "├── " };

        output.push_str(&format!("{}{}{}\n", prefix, branch, node.name));

        let new_prefix = if is_last {
            format!("{}    ", prefix)
        } else {
            format!("{}│   ", prefix)
        };

        let count = node.children.len();
        for (i, child) in node.children.iter().enumerate() {
            output.push_str(&Self::print_tree(child, &new_prefix, i == count - 1));
        }

        output
    }

    fn to_tree_node<T: ExpressionVisitor<TreeNode>>(
        visitor: &T,
        name: &str,
        expressions: &mut Vec<&mut Expression>,
    ) -> TreeNode {
        let mut node = TreeNode {
            name: name.to_string(),
            children: Vec::new(),
        };

        for expr in expressions.iter_mut() {
            node.children.push(expr.accept(visitor));
        }

        node
    }
}

impl ExpressionVisitor<TreeNode> for PrettyTreePrinter {
    fn visit_comma(&self, expr: &mut Comma) -> TreeNode {
        let mut expressions = vec![&mut expr.right, &mut expr.left];
        Self::to_tree_node(self, ",", &mut expressions)
    }

    fn visit_ternary(&self, expr: &mut Ternary) -> TreeNode {
        let mut expressions = vec![&mut expr.falsy, &mut expr.truth, &mut expr.condition];
        Self::to_tree_node(self, "?:", &mut expressions)
    }

    fn visit_binary(&self, expr: &mut Binary) -> TreeNode {
        let mut expressions = vec![&mut expr.right, &mut expr.left];
        Self::to_tree_node(self, &expr.operator.lexeme, &mut expressions)
    }

    fn visit_unary(&self, expr: &mut Unary) -> TreeNode {
        let mut expressions = vec![&mut expr.right];
        Self::to_tree_node(self, &expr.operator.lexeme, &mut expressions)
    }

    fn visit_grouping(&self, expr: &mut Grouping) -> TreeNode {
        let mut expressions = vec![&mut expr.expression];
        Self::to_tree_node(self, "group", &mut expressions)
    }

    fn visit_literal(&self, expr: &mut Literal) -> TreeNode {
        TreeNode {
            name: match expr.value {
                LiteralValue::True => "True".to_string(),
                LiteralValue::False => "False".to_string(),
                LiteralValue::Number(number) => number.to_string(),
                LiteralValue::String(value) => value.to_string(),
                LiteralValue::None => "None".to_string(),
            },
            children: vec![],
        }
    }
}
