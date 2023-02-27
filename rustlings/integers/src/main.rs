use std::collections::HashMap;

fn main() {
    let v = vec![100, 32, 57, 100, 58, 65, 57, 50, 57];
    println!("The mode of {:?} is {}", v, calculate_mode(&v));
}

fn calculate_mode(v : &Vec<i32>) -> i32 {
    let mut hash_map = HashMap::new();

    for x in v {
        let count = hash_map.entry(x).or_insert(0);
        *count += 1;
    }

    let mut key_mode = 0;
    let mut value_mode = 0;

    for (key, value) in hash_map.iter() {
        if *value > value_mode {
            key_mode = **key;
            value_mode = *value;
        }
    }

    key_mode
}