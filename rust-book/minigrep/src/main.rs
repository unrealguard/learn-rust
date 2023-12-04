use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = parse_config(&args);

    println!("Searching for value: {}", config.query);
    println!("In path: {}", config.file_path);

    let contents = fs::read_to_string(config.file_path)
        .expect("Should have been able to read the file");

    println!("With file contents:\n{contents}");
}

struct Config {
    query: String,
    file_path: String,
}

fn parse_config(args: &Vec<String>) -> Config {
    let query: &String = &args[1];
    let file_path: &String = &args[2];

    Config {
        query: query.clone(),
        file_path: file_path.clone(),
    }
}
