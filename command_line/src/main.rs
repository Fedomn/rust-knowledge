use std::{env, process};

use command_line::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Encounter err when parse arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    println!("================");

    if let Err(e) = command_line::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1)
    }
}
