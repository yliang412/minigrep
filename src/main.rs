use std::{env, process};

use minigrep::Config;
use colored::*;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err.red().bold());
        eprintln!("Usage: [CASE_INSENSITIVE=1] minigrep <search string> <filename>");
        process::exit(1);
    });

    eprintln!("Searching for `{}` in file `{}`", config.query_str.green().bold(), config.path.green().bold());

    if let Err(e) = minigrep::run_main(config) {
        eprintln!("Application error: {}", e.to_string().red().bold());

        process::exit(1);
    }
}

