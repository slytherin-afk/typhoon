pub mod expression;
pub mod parser;
pub mod scanner;

use expression::visitor_interpreter::{Interpreter, RuntimeError};
use parser::{Counter, Parser};
use rustyline::DefaultEditor;
use scanner::{token::Token, token_type::TokenType, Scanner};

pub struct Typhoon {
    had_error: bool,
    had_runtime_error: bool,
    version: &'static str,
}

impl Typhoon {
    pub fn new() -> Self {
        Self {
            had_error: false,
            had_runtime_error: false,
            version: "Beta 0.0.1",
        }
    }

    pub fn run_file(&mut self) {
        todo!()
    }

    pub fn run_prompt(&mut self) {
        println!("Typhoon {}", self.version);

        let mut rl = DefaultEditor::new().expect("failed to create editor");

        loop {
            let input = rl.readline("> ").expect("input is read correctly");

            if input == "exit()" {
                break;
            }

            rl.add_history_entry(&input)
                .expect("input added to history");
            self.run(input);
            self.had_error = false;
        }
    }

    fn run(&mut self, source: String) {
        let scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens(self);

        if self.had_error {
            return;
        }

        let parser = Parser::new(tokens);
        let mut counter = Counter::new();
        let expression = parser.parse(&mut counter, self);

        if expression.is_err() {
            return;
        }

        let mut expression = expression.expect("must have a valid expression");
        Interpreter::interpret(&mut expression, self);
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

    pub fn runtime_error(&mut self, runtime_error: &RuntimeError) {
        println!(
            "{}\n[line {}]",
            runtime_error.message, runtime_error.token.line
        );

        self.had_error = true;
    }

    fn report(&mut self, line: usize, wheres: &str, message: &str) {
        println!("[{line}] Error {wheres}: {message}");
        self.had_error = true;
    }
}
