use std::env;
use std::process;

use mingrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Parsing Error: {}", err);
        process::exit(1);
    });

    if let Err(err) = mingrep::run(config) {
        eprintln!("Application Error: {}", err);
        process::exit(1);
    }
}
