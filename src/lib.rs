extern crate ansi_term;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*; // many crates have a prelude, a set of commonly used types and functions into scope
use std::env;
use ansi_term::Colour::Red;
use ansi_term::{ANSIString, ANSIStrings};

pub fn run(config: Config) -> Result<(), Box<Error>> { // Box<Error> Indicates that function will return a type that implements the Error trait, if an error occurs
    let mut f = File::open(config.filename)?; // ? replaces .expect; on panic!, ? returns error value from current function

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        print_highlighted_text(line, &config.query);
    }

    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    // TODO: Write tests for `new`
    // pub fn new(args: &[String]) -> Result<Config, &'static str> {
    pub fn new(args: env::Args) -> Result<Config, &'static str>
     {

        // TODO: Determine if this is an acceptable way to handle creating a struct (e.g. making it mutable and changing values later)
        // TODO: Determine a better way for handling CLI arguments and flags; create a map of args 
        let mut config = Config {
            query: "".to_string(),
            filename: "".to_string(),
            case_sensitive: true,
        };

        for arg in args.skip(1) {
            if arg == "-i" || arg == "--ignore-case" {
                config.case_sensitive = false;
            } else if config.query.is_empty() {
                config.query = arg.to_string();
            } else {
                config.filename = arg.to_string();
            }
        }

        Ok(config)
    }
}

// TODO: Make this function handle the --ignore-case flag!
fn print_highlighted_text(line: &str, query: &str) {
    let v: Vec<&str> = line.split(&query).collect();

    let mut strings: Vec<ANSIString> = Vec::new();

    if line.starts_with(query) {
        strings.push(Red.bold().paint(query));
    }

    for (index, &chunk) in v.iter().enumerate() {
        strings.push(ANSIString::from(chunk));
        if index < v.len() - 1 {
            strings.push(Red.bold().paint(query));
        }
    }

    if line.ends_with(query) {
        strings.push(Red.bold().paint(query));
    }

    println!("{}", ANSIStrings(&strings));
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

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();
    
    for line in contents.lines() { // .lines() allows for line-by-line iteration of string
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive."], 
            search(query, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}