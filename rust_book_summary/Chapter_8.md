# Common Heap Collections (Vector, String, Hash Map)

## Vector

Vectors store values of the same type next to each other in memory.

```
    let v: Vec<i32> = Vec::new();
```
More often, you’ll create a *Vec<T>* with initial values and Rust will infer the type of value you want to store, so you rarely need to do this type annotation. 

```
    let v = vec![1, 2, 3];

    let mut v = Vec::new();
    v.push(5);
```

### Reading Elements of Vectors

```
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }
```

### Iterating over the Values in a Vector

Iterating over a vector, whether immutably or mutably, is safe because of the borrow checker's rules.

```
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
```

### Using an Enum to Store Multiple Types

Vectors can only store values that are the same type. Enums can be used to store a list of items of different types.

```
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
```

### Dropping a Vector Drops Its Elements

Like any other struct, a vector is freed when it goes out of scope.

## Storing UTF-8 Encoded Text with Strings

### str
- *str* is a string type in the core language, UTF-8 encoded.

### String

- *String* is provided by Rust’s standard library rather than coded into the core language. 
- Growable, mutable, owned, UTF-8 encoded string type. 
- Implemented as a wrapper around a vector of bytes with some extra guarantees, restrictions, and capabilities
- Accessing individual characters in a string by referencing them by index is not valid.

```
    let mut s1 = String::new();

    let s2 = String::from("initial contents");
    s2.push_str("bar");
    s2.push('l');
```

## Hash Maps

The type HashMap<K, V> stores a mapping of keys of type K to values of type V using a hashing function, which determines how it places these keys and values into memory.

Iterating over a hash map happens in an arbitrary order.

```
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // insert can also be used to overwrite a value
    scores.insert(String::from("Blue"), 15);

    // insert or overwrite
    // Entry is defined to return a mutable reference to the value for the corresponding Entry key if that key exists
    scores.entry(String::from("Yellow")).or_insert(70);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }
```

### Hashed Maps and Ownership

For types that implement the Copy trait, like i32, the values are copied into the hash map. For owned values like String, the values will be moved and the hash map will be the owner of those values

### Hashing Functions

By default, HashMap uses a hashing function called SipHash that can provide resistance to Denial of Service (DoS) attacks involving hash tables1. This is not the fastest hashing algorithm available, but the trade-off for better security that comes with the drop in performance is worth it. If you profile your code and find that the default hash function is too slow for your purposes, you can switch to another function by specifying a different hasher.