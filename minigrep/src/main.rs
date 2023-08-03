use std::{env, process};

use minigrep::Args;

fn main() {
    let args: Vec<String> = env::args().collect();
    let parsed_args = Args::build(&args).unwrap_or_else(|err| {
        // write to standard error
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    if let Err(e) = minigrep::run(parsed_args) {
        // write to standard error
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
