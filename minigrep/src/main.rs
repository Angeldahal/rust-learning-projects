use std::env;
use std::process;
use minigrep::Config;

fn main() {

    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    }); 
    println!("Searching for {0}", config.query);
    println!("In file {0}", config.file_path);
    
    if let Err(e) = minigrep::run(config) {
        eprintln!("Applicaiton Error, {e}");
        process::exit(1);
    } 
}
