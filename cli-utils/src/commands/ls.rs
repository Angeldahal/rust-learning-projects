use std::fs;
use std::path::Path;

pub fn run(path: Option<String>) {
    let dir = path.unwrap_or_else(|| ".".to_string());

    match fs::read_dir(Path::new(&dir)) {
        Ok(entries) => {
            for entry in entries.flatten() {
                println!("{}", entry.file_name().to_string_lossy());
            }
        }
        Err(_) => eprintln!("Error: Could not access '{}'", dir),
    }
}