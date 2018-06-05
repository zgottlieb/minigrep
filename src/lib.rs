use std::error::Error;
use std::fs::File;
use std::io::prelude::*; // many crates have a prelude, a set of commonly used types and functions into scope

pub fn run(config: Config) -> Result<(), Box<Error>> { // Box<Error> Indicates that function will return a type that implements the Error trait, if an error occurs
    let mut f = File::open(config.filename)?; // ? replaces .expect; on panic!, ? returns error value from current function

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    for line in search(&config.query, &contents) {
        println!("{}", line);
    }

    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    // TODO: Write tests for `new`
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
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

//<'a> is a lifetime parameter, which specifies which argument lifetime is connected to the return value
// This effectively tells rust that the data returned by `search` will live as long as the data passed via the contents arg
// See Listing 12-16 in the Rust book
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    
    for line in contents.lines() { // .lines() allows for line-by-line iteration of string
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."], 
            search(query, contents)
        );
    }
}