use minigrep::{run, Config};
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("parse arguments error: {}", err);
        process::exit(1);
    });
    println!(
        "Searching for '{}' in file {}",
        config.query, config.filename
    );
    if let Err(e) = run(&config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
