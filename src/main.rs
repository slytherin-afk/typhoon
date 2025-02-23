use clap::{Arg, Command};
use typhoon::{
    expression::{
        binary::Binary, grouping::Grouping, literal::Literal, tree_printer::TreePrinter,
        unary::Unary, Expression,
    },
    scanner::{token::Token, token_type::TokenType},
    Typhoon,
};

fn main() {
    let matches = Command::new("MyApp")
        .arg(
            Arg::new("filenames")
                .help("List of filenames (optional)")
                .num_args(0..)
                .required(false),
        )
        .get_matches();

    let filenames: Vec<_> = matches
        .get_many::<String>("filenames")
        .unwrap_or_default()
        .map(|s| s.to_string())
        .collect();

    let mut compiler = Typhoon::new();

    if filenames.is_empty() {
        println!("Welcome to Typhoon");
        compiler.run_prompt();
    } else {
        println!("Filenames: {:?}", filenames);
    }
}

fn _main_2() {
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

    let result = TreePrinter::print(&mut expression);

    println!("{}", result);
}
