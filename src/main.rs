use std::env;
use std::fs::File;
use std::io::prelude::*; // many crates have a prelude, a set of commonly used types and functions into scope
use std::process;
use std::error::Error;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for... {}", config.query);
    println!("In file... {}", config.filename);

    if let Err(e) = run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<Error>> { // Box<Error> Indicates that function will return a type that implements the Error trait, if an error occurs
    let mut f = File::open(config.filename)?; // ? replaces .expect; on panic!, ? returns error value from current function

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    
    println!("With text:\n{}", contents);

    Ok(())
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        // originally was using & to indicate that we want a reference to the item at the specified index, not the actual item
        // now removing the & because we need to pass an actual String, not a ref; using clone to create a copy
        // using .clone() has the disadvantage of using more memory and time, but it means we don't have to manage lifetimes of the references
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}


