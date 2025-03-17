mod args;
mod commands;

use clap::Parser;
use args::{Cli, Commands};
use commands::{ echo, cat, ls, find, grep };

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Echo { text } => {
            echo::run(text);
        }
        Commands::Cat { filename } => {
            cat::run(filename);
        }

        Commands::Ls { path } => {
            ls::run(path);
        }

        Commands::Find { path, name } => {
            find::run(path, name);
        },

        Commands::Grep { pattern, filename, case_insensitive, regex, whole_word } => {
            grep::run(pattern, filename, case_insensitive, regex, whole_word)
        },
    }
}