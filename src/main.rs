use minigrep::{build, run};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let closure = |err: &str| {
        eprintln!("Problem parsing input: {}", err);
        std::process::exit(1);
    };

    let config = build(&args).unwrap_or_else(closure);

    if let Err(err) = run(config) {
        eprintln!("Application error: {}", err);
        std::process::exit(1);
    };
}
