use std::fs;
use std::io::{self, Read};

pub fn run(filename: String) {
    match fs::read_to_string(&filename) {
        Ok(contents) => print!("{}", contents),
        Err(_) => eprintln!("Error: Could not read file '{}", filename),
    }
}