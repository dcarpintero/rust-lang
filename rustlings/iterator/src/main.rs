fn main() {
    let v1 = vec![1, 2, 3];
    let total: i32 = v1.iter().sum();

    println!("Vector: {:?}", v1);
    println!("Total: {}", total);

    // Map create a new iterator, collect consumes the new iterator and creates a vector
    // map takes a closure to define the operation to perform on each item
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    println!("Map: {:?}", v2);
}
