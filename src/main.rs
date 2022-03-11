use std::{env, process};

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        println!("Usage: minigrep <search string> <filename>");
        process::exit(1);
    });

    println!("Searching for {}", config.query_str);
    println!("In file {}", config.path);

    if let Err(e) = minigrep::run_main(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}

