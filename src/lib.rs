pub mod environment;
pub mod expression;
pub mod object;
pub mod parser;
pub mod scanner;
pub mod stmt;
pub mod visitor;

use std::{cell::RefCell, rc::Rc};

use environment::Environment;
use parser::{Counter, Parser};
use rustyline::DefaultEditor;
use scanner::{token::Token, token_type::TokenType, Scanner};
use visitor::interpreter::{Interpreter, RuntimeError};

#[allow(dead_code)]
pub struct Lib {
    had_error: bool,
    had_runtime_error: bool,
    version: &'static str,
}

impl Lib {
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
        let global_env = Rc::new(RefCell::new(Environment::new(None)));

        loop {
            let input = rl.readline("> ").expect("input is read correctly");
            rl.add_history_entry(&input)
                .expect("input added to history");
            self.run(input, Rc::clone(&global_env));
            self.had_error = false;
        }
    }

    fn run(&mut self, source: String, global_env: Rc<RefCell<Environment>>) {
        let scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens(self);

        if self.had_error {
            return;
        }

        let parser = Parser::new(tokens);
        let mut counter = Counter::new();
        let statements = parser.parse(&mut counter, self);

        if statements.is_err() {
            return;
        }

        let mut statements = statements.expect("got valid statements");

        Interpreter::interpret(&mut statements, self, global_env);
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
        println!("[{}] {}", runtime_error.token.line, runtime_error.message);

        self.had_error = true;
    }

    fn report(&mut self, line: usize, wheres: &str, message: &str) {
        println!("[{line}] Error {wheres}: {message}");
        self.had_error = true;
    }
}
