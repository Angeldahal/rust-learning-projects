use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name="cli-utils")]
#[command(about="A simple Rust CLI tool", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Repeats input text
    Echo { text: String },

    /// Displays file contents
    Cat { filename: String },

    /// Lists files in a directory
    Ls { path: Option<String> },

    Find { path: String, name: String},

    Grep { 
        pattern: String, 
        filename: String,
        #[arg(short, long)] case_insensitive: bool,
        #[arg(short = 'r', long)] regex: bool,
        #[arg(short = 'w', long)] whole_word: bool
    },
}