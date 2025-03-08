use clap::{Arg, Command};
use typhoon::Typhoon;

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
        compiler.run_prompt();
    } else {
        println!("Filenames: {:?}", filenames);
    }
}
