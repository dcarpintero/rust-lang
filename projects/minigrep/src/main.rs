// bring modules into scope
use std::env; 
use std::process;

use minigrep::Config;

fn main() {
    // the collect method on an iterator turns it into a collection, such as a vector
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for '{}'", config.query);
    println!("In file '{}'", config.file_path);

    // Because run returns () in the success case, we only handle an error
    if let Err(e) = minigrep::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }

    // print the vector using the debug macro
    dbg!(args);
}