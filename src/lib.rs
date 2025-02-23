pub mod expression;
pub mod scanner;

use rustyline::DefaultEditor;
use scanner::Scanner;

pub struct Typhoon {
    have_error: bool,
}

impl Typhoon {
    pub fn new() -> Self {
        Self { have_error: false }
    }

    pub fn run_file(&mut self) {}

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
        }
    }

    fn run(&mut self, source: String) {
        let mut scanner = Scanner::new(self, source);
        let tokens = scanner.scan_tokens();

        for token in tokens {
            println!("{}", token)
        }
    }

    pub fn add_error(&mut self, line: usize, message: &str) {
        self.have_error = true;
        self.report_error(line, message);
    }

    fn report_error(&mut self, line: usize, message: &str) {
        println!("{line} {message}");
    }
}
