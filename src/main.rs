use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let query_str = &args[1];
    let path = &args[2];

    println!("Searching for {}", query_str);
    println!("In file {}", path);

    let content = fs::read_to_string(path).expect("Something went wrong reading the file");

    println!("With text:\n{}", content);
}
