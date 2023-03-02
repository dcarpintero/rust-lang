use std::env; // bring std::env and std::fs modules into scope
use std::fs;

fn main() {
    // the collect method on an iterator turns it into a collection, such as a vector
    let args: Vec<String> = env::args().collect();
    let config = parse_config(&args);

    println!("Searching for {}", config.query);
    println!("In file: '{}'", config.file_path);

    let contents = fs::read_to_string(config.file_path)
        .expect("Should have been able to read the file");
    println!("With text:\n{contents}");

    // print the vector using the debug macro
    dbg!(args);
}

struct Config {
    query: String,
    file_path: String,
}

fn parse_config(args: &[String]) -> Config {
    // In practice clone tends to be avoided to fix ownership problems because of its runtime cost
    let query = args[1].clone();
    let file_path = args[2].clone();

    Config { query, file_path }
}