use minigrep::Config;
use std::{env, process};

// cargo run > output.txt
// cargo run -- to poem.txt > output.txt
fn main() {
    let args_iter = env::args();

    let config = Config::build(args_iter).unwrap_or_else(|err| {
        eprintln!("Problem parsing args: {err}");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application Error: {e}");
        process::exit(1);
    }
}
