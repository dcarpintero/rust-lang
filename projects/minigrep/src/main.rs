use std::env; // bring std::env module into scope

fn main() {
    // the collect method on an iterator turns it into a collection, such as a vector
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", file_path);

    // print the vector using the debug macro
    dbg!(args);
}
