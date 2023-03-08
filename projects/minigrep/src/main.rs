// bring modules into scope
use std::env; 
use std::process;

use minigrep::Config;

fn main() {
    // the collect method on an iterator turns it into a collection, such as a vector
    // let args: Vec<String> = env::args().collect();
    
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // Because run returns () in the success case, we only handle an error
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }

    // print the vector using the debug macro
    // dbg!(args);
}