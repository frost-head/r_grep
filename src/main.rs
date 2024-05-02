use std::env;
use std::process;
fn main() {
    let args: Vec<String> = env::args().collect(); // Reading CMD line args
    let config: Config = args_parse(&args);

    println!("Searching in : {}",config.file_name);
    println!("Searching for : {}",config.token);
 
}

struct Config {
    file_name: String,
    token : String,
}

impl Config{
    fn new(args: &[String]) -> Config{
        
        Config{
            file_name : args[1].clone(),
            token : args[2].clone(),
        }
    }
}

fn args_parse(args: &[String]) -> Config{
    if args[1] == "-h"{
        println!("arg 1 : file name");
        println!("arg 2 : what you are searching for");
        process::exit(0);
    }
    else if args.len() < 3 {
        eprintln!("Oops Problem with argumets....! use -h for help.");
        process::exit(1);
    }
    else {
        Config::new(&args)
    }
}