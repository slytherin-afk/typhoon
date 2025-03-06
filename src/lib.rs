pub mod expression;
pub mod parser;
pub mod scanner;

use expression::visitor_interpreter::Interpreter;
use parser::{Counter, Parser};
use rustyline::DefaultEditor;
use scanner::{token::Token, token_type::TokenType, Scanner};

pub struct Typhoon {
    have_error: bool,
}

impl Typhoon {
    pub fn new() -> Self {
        Self { have_error: false }
    }

    pub fn run_file(&mut self) {
        todo!()
    }

    pub fn run_prompt(&mut self) {
        let mut rl = DefaultEditor::new().expect("failed to create editor");

        loop {
            let input = rl.readline("> ").expect("input is read correctly");

            if input == "exit()" {
                break;
            }

            rl.add_history_entry(&input)
                .expect("input added to history");
            self.run(input);
            self.have_error = false;
        }
    }

    fn run(&mut self, source: String) {
        let scanner = Scanner::new(&source);
        let tokens = scanner.scan_tokens(self);
        let parser = Parser::new(tokens);
        let mut counter = Counter::new();
        let mut expression = parser
            .parse(&mut counter, self)
            .expect("a valid expression");
        let result = Interpreter::evaluate(&mut expression);

        println!("{}", result);
    }

    pub fn error_one(&mut self, line: usize, message: &str) {
        self.report(line, "", message);
    }

    pub fn error_two<'a>(&mut self, token: &'a Token, message: &str) {
        if token.token_type == TokenType::Eof {
            self.report(token.line, "at end", message);
        } else {
            let wheres = format!("at '{}'", token.lexeme);
            self.report(token.line, &wheres, message);
        }
    }

    fn report(&mut self, line: usize, wheres: &str, message: &str) {
        println!("[{line}] Error {wheres}: {message}");
        self.have_error = true;
    }
}
