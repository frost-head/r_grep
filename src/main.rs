use std::env;
use std::process;
use r_grep::{Config, run};

fn main() {
    let args: Vec<String> = env::args().collect(); // Reading CMD line args
    let config: Config = args_parse(&args); // Creating config
    run(config); // Searching and retriving content
}


fn args_parse(args: &[String]) -> Config{
    // Checking for arguments
    if args[1] == "-h"{ //  help menu
        println!("There must be 2 arguments");
        println!("arg 1 : file name");
        println!("arg 2 : what you are searching for");
        process::exit(0);
    }
    else if args.len() < 3 { // not sufficent arguments
        eprintln!("Oops Problem with argumets....! use -h for help.");
        process::exit(1);
    }
    else { 
        Config::new(&args) // creating config
    }
}