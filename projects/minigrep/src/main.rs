// bring modules into scope
use std::env; 
use std::fs;
use std::process;
use std::error::Error;

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
    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }

    // print the vector using the debug macro
    dbg!(args);
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    println!("\n{contents}");

    Ok(())
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        // In practice clone tends to be avoided to fix ownership problems because of its runtime cost
        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}