use std::collections::HashMap;

struct VectorMode {
    key: Option<i32>,
    count: u32, 
}

fn main() {
    let v = vec![100, 32, 57, 100, 58, 65, 57, 50, 57, 50];
    println!("The mode of {:?} is: {}", v, calculate_mode(&v));
}

fn calculate_mode(v : &Vec<i32>) -> i32 {
    let mut hash_map = HashMap::new();
    let mut vector_mode = VectorMode {
        key: None,
        count: 0,
    };

    for x in v {
        let count = hash_map.entry(x).or_insert(0);
        *count += 1;

        if count > (&mut vector_mode.count) {
            vector_mode.key = Some(*x);
            vector_mode.count = *count;
        }
    }

    vector_mode.key.unwrap()
}