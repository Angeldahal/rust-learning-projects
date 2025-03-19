use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;
use colored::Colorize;
use rayon::prelude::*;

pub fn run(pattern: String, filename: String, case_insensitive: bool, regex: bool, whole_word: bool) {
    if let Err(e) = search(&pattern, &filename, case_insensitive, regex, whole_word) {
        eprintln!("{}: {}", "Error".red(), e);
    }
}

fn search(pattern: &str, filename: &str, case_insensitive: bool, regex: bool, whole_word: bool) -> Result<(), io::Error> {
    let path = Path::new(filename);
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut pattern = pattern.to_string();
    if case_insensitive {
        pattern = pattern.to_lowercase();
    }
    if whole_word {
        pattern = format!(r"\b{}\b", pattern);
    }

    let re = if regex {
        Regex::new(&pattern).ok()
    } else {
        None
    };

    reader
        .lines()
        .enumerate()
        .par_bridge()
        .filter_map(|(line_number, line)| line.ok().map(|l| (line_number, l)))
        .filter(|(_, line)| {
            let line_to_match = if case_insensitive {
                line.to_lowercase()
            } else {
                line.clone()
            };

            if let Some(re) = &re {
                re.is_match(&line_to_match)
            } else {
                line_to_match.contains(&pattern)
            }
        })
        .for_each(|(line_number, line)| {
            let highlighted = line.replace(pattern.as_str(), &pattern.yellow().to_string());
            println!("{}: {}", (line_number + 1).to_string().blue(), highlighted);
        });

    Ok(())
}