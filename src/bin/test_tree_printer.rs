use typhoon::{
    expression::{
        binary::Binary, grouping::Grouping, literal::Literal, tree_printer::TreePrinter,
        unary::Unary, Expression,
    },
    scanner::{token::Token, token_type::TokenType},
};

fn main() {
    let mut expression = Expression::Binary(Box::new(Binary {
        left: Expression::Unary(Box::new(Unary {
            operator: Token {
                token_type: TokenType::Minus,
                lexeme: "-".to_string(),
                literal: None,
                line: 1,
            },
            right: Expression::Literal(Box::new(Literal {
                value: Box::new(123),
            })),
        })),
        operator: Token {
            token_type: TokenType::Star,
            lexeme: "*".to_string(),
            literal: None,
            line: 1,
        },
        right: Expression::Grouping(Box::new(Grouping {
            expression: Expression::Literal(Box::new(Literal {
                value: Box::new(45.67),
            })),
        })),
    }));

    println!("{}", TreePrinter::print(&mut expression));
}
