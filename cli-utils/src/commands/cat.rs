use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use colored::Colorize;

pub fn run(filename: String) {
    if let Err(e) = read_file(&filename) {
        eprint!("{}: {}", "Error".red(), e);
    }
}

fn read_file(filename: &str) -> Result<(), io::Error> {
    let path = Path::new(filename);
    let file = File::open(path)?;

    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        println!("{}", line?);
    }

    Ok(())
}
