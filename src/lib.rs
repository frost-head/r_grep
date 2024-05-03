use std::fs;

pub fn search<'a>(token: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new(); // creating empty vector

    for line in contents.lines() { // checking if token is contained
        if line.contains(token) {
            results.push(line);
        }
    }

    results
}

pub fn run(config : Config){
    // retriving content
    let content: String =  fs::read_to_string(config.file_name).expect("Error reading the file");
    // searching for matches
    for line in search(&config.token, &content) {
        println!("{line}");
    }
}

pub struct Config {
    // containes the configuration made of arguments passed by user
    pub file_name: String,
    pub token: String,
}

impl Config{
    pub fn new(args: &[String]) -> Config{
        // creates a new config given arguments
        Config{
            file_name: args[1].clone(),
            token: args[2].clone(),
        }
    }
}
