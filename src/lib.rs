use std::error::Error;
use std::{env, fs};
pub struct Config {
    pub query_str: String,
    pub path: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let query_str = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query_str,
            path,
            case_sensitive,
        })
    }
}

pub fn run_main(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.path)?;
    let res = if config.case_sensitive {
        search(&config.query_str, &content)
    } else {
        search_case_insensitive(&config.query_str, &content)
    };

    for line in res {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query_str: &str, content: &'a str) -> Vec<&'a str> {
    content
        .lines()
        .filter(|line| line.contains(query_str))
        .collect()
}

pub fn search_case_insensitive<'a>(query_str: &'a str, content: &'a str) -> Vec<&'a str> {
    let query_str = query_str.to_lowercase();
    let mut res = Vec::new();

    for line in content.lines() {
        if line.to_lowercase().contains(&query_str) {
            res.push(line);
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query_str = "duct";
        let content = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        assert_eq!(vec!["safe, fast, productive."], search(query_str, content));
    }

    #[test]
    fn case_insensitive() {
        let query_str = "rUsT";
        let content = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query_str, content)
        );
    }
}
