use std::fs;
use std::path::Path;
use colored::Colorize;

pub fn run(path: Option<String>) {
    let dir_path = path.unwrap_or_else(|| ".".to_string());
    if let Err(e) = list_files(&dir_path) {
        eprintln!("{}: {}", "Error".red(), e);
    }
}

fn list_files(path: &str) -> Result<(), std::io::Error> {
    let entries = fs::read_dir(Path::new(path))?;

    for entry in entries {
        let entry = entry?;
        let file_name = entry.file_name();
        let file_name = file_name.to_string_lossy();

        if entry.path().is_dir() {
            println!("{}", file_name.blue());
        } else {
            println!("{}", file_name);
        }
    }

    Ok(())
}