use std::{fs, error::Error};

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &Vec<String>) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Expected at least 3 arguments");
        }

        let query: &String = &args[1];
        let file_path: &String = &args[2];

        Ok(Config {
            query: query.clone(),
            file_path: file_path.clone(),
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    println!("With file contents:\n{contents}");

    Ok(())
}
