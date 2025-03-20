pub mod errors;
pub mod expression;
pub mod globals;
pub mod operations;
pub mod stmt;

mod environment;
mod interpreter;
mod object;
mod parser;
mod resolver;
mod scanner;
mod token;
mod token_type;

use colored::Colorize;
use rustyline::DefaultEditor;

pub use environment::Environment;
pub use interpreter::Interpreter;
pub use object::{Callable, Class, ClassInstance, Function, Object, ResolvableFunction};
pub use token::{LiteralType, Token};
pub use token_type::TokenType;

use errors::RuntimeError;
use parser::Parser;
use resolver::Resolver;
use scanner::Scanner;

pub struct Lib {
    interpreter: Interpreter,
}

static mut HAD_ERROR: bool = false;
static mut HAD_RUNTIME_ERROR: bool = false;
static VERSION: &'static str = "Beta 0.0.1";

impl Lib {
    pub fn new() -> Self {
        Self {
            interpreter: Interpreter::new(),
        }
    }

    pub fn run_file(&mut self) {
        todo!()
    }

    pub fn run_prompt(&mut self) {
        println!("{}", VERSION);

        let mut rl = DefaultEditor::new().expect("failed to create editor");

        loop {
            let input = rl.readline("> ").expect("input is read correctly");
            rl.add_history_entry(&input)
                .expect("input added to history");
            self.run(input);

            unsafe {
                HAD_ERROR = false;
            }
        }
    }

    fn run(&mut self, source: String) {
        let scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();

        if unsafe { HAD_ERROR } {
            return;
        }

        let mut parser = Parser::new(tokens);
        let statements = parser.parse();

        if unsafe { HAD_ERROR } {
            return;
        }

        let mut resolver = Resolver::new(&mut self.interpreter);

        resolver.resolve_stmts(&statements);

        if unsafe { HAD_ERROR } {
            return;
        }

        self.interpreter.interpret(&statements);
    }

    pub fn error_one(line: usize, message: &str) {
        Lib::report(line, "", message);
    }

    pub fn error_two(token: &Token, message: &str) {
        if token.token_type == TokenType::Eof {
            Lib::report(token.line, "at end", message);
        } else {
            let wheres = format!("at '{}'", token.lexeme);
            Lib::report(token.line, &wheres, message);
        }
    }

    pub fn runtime_error(runtime_error: &RuntimeError) {
        println!(
            "[{}] {}",
            runtime_error.token().line.to_string().bold().blue(),
            runtime_error.message().bright_red()
        );

        unsafe {
            HAD_RUNTIME_ERROR = true;
        }
    }

    fn report(line: usize, wheres: &str, message: &str) {
        println!(
            "{} {} {}: {}",
            format!("[{}]", line).bold().blue(),
            "Error:".bold().red(),
            wheres.yellow(),
            message.bright_white()
        );

        unsafe {
            HAD_ERROR = true;
        }
    }

    pub fn warn_two(token: &Token, message: &str) {
        if token.token_type == TokenType::Eof {
            Lib::report_warning(token.line, "at end", message);
        } else {
            let wheres = format!("at '{}'", token.lexeme);
            Lib::report_warning(token.line, &wheres, message);
        }
    }

    fn report_warning(line: usize, wheres: &str, message: &str) {
        println!(
            "{} {} {}: {}",
            format!("[{}]", line).bold().blue(),
            "Warning".truecolor(199, 79, 25).bold(),
            wheres.yellow(),
            message.bright_white()
        );
    }
}
