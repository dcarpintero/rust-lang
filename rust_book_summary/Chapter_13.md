# Functional Language Features: Iterators and Closures

Rust's design has taken inspiration from many existing languages and techniques, and one significant influence is functional programming. Programming in a functional style often includes using functions as values by passing them in arguments, returning them from other functions, assigning them to variables for later execution, and so forth.

## Closures

- Rust´s closures are anonymous functions you can save in a variable or pass as arguments to other functions. You can create the closure in one place and then call the closure elsewhere to evaluate it in a different context. Unlike functions, closures can capture values from the scope in which they´re defined.

- Closures don´t usually require you to annotate the types of the parameters or the return value like *fn* functions do. Note that the compiler will infer and lock the types into the closure, trying to use a different type with the same closure will result in an error.

```
    fn  add_one_v1   (x: u32) -> u32 { x + 1 }
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x|             { x + 1 };
    let add_one_v4 = |x|               x + 1  ;
```

- Closures can capture values from their environment in three ways: borrowing immutably, borrowing mutably, and taking ownership (by using the *move* keyword).

## Iterators

```
    #[cfg(test)]
    mod tests {
        #[test]
        fn iterator_sum() {
            let v1 = vec![1, 2, 3];

            let v1_iter = v1.iter();

            let total: i32 = v1_iter.sum();

            assert_eq!(total, 6);
        }
    }

```

Iterators are one of Rust’s zero-cost abstractions, i.e. the abstraction imposes no additional runtime overhead. This is analogous to how Bjarne Stroustrup, the original designer and implementor of C++, defines zero-overhead in “Foundations of C++” (2012).

