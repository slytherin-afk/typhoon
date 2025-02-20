mod scanner;

use rustyline::Editor;

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
        let mut rl =
            Editor::<(), rustyline::history::FileHistory>::new().expect("Failed to create editor");

        loop {
            let readline = rl.readline("> ");

            match readline {
                Ok(line) => {
                    let input = line.trim();

                    if input == "exit()" {
                        break;
                    }

                    self.run(input.to_string());
                }
                Err(_) => {
                    println!("Error reading input");
                    break;
                }
            }
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
