use std::fs;
use std::path::Path;

pub fn run(path: String, name: String) {
    if let Err(e) = search(Path::new(&path), &name) {
        eprintln!("Error: {}", e);
    }
}

fn search(dir: &Path, name: &str) -> Result<(), std::io::Error> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let entry_path = entry.path();
            if entry_path.file_name().map_or(false, |n| n.to_string_lossy() == name) {
                println!("{}", entry_path.display());
            }
            if entry_path.is_dir() {
                search(&entry_path, name)?;
            }
        }
    }
    Ok(())
}