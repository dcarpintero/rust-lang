# Error Handling

Rust groups errors into two major categories: recoverable and unrecoverable errors. For a recoverable error, such as a file not found error, we most likely just want to report the problem to the user and retry the operation. Unrecoverable errors are always symptoms of bugs, like trying to access a location beyond the end of an array, and so we want to immediately stop the program.

Rust doesn’t have exceptions. Instead, it has the type *Result<T, E>* for recoverable errors and the *panic! macro* that stops execution when the program encounters an unrecoverable error.

* By default, when a panic occurs, the program starts unwinding, which means Rust walks back up the stack and cleans up the data from each function it encounters. However, this walking back and cleanup is a lot of work. Rust, therefore, allows you to choose the alternative of immediately aborting in the *Cargo.toml*. This will end the program without cleaning up.

```
    [profile.release]
    panic = 'abort'
```

* A backtrace is a list of all the functions that have been called to get to this point. 

```
    RUST_BACKTRACE=1 cargo run
```

## Recoverable Errors with Result

The Result enum is defined as having two variants, Ok and Err, as follows:

```
    enum Result<T, E> {
        Ok(T),
        Err(E),
    }
```

```
    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };
```

```
    use std::fs::File;
    use std::io::ErrorKind;

    fn main() {
        let greeting_file_result = File::open("hello.txt");

        let greeting_file = match greeting_file_result {
            Ok(file) => file,
            Err(error) => match error.kind() {
                ErrorKind::NotFound => match File::create("hello.txt") {
                    Ok(fc) => fc,
                    Err(e) => panic!("Problem creating the file: {:?}", e),
                },
                other_error => {
                    panic!("Problem opening the file: {:?}", other_error);
                }
            },
        };
    }
```

```
    use std::fs::File;
    use std::io::ErrorKind;

    fn main() {
        let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
            if error.kind() == ErrorKind::NotFound {
                File::create("hello.txt").unwrap_or_else(|error| {
                    panic!("Problem creating the file: {:?}", error);
                })
            } else {
                panic!("Problem opening the file: {:?}", error);
            }
        });
    }
```

## To panic! or Not to panic!

In situations such as examples, prototype code, and tests, it’s more appropriate to write code that panics instead of returning a *Result*.

The unwrap and expect methods are very handy when prototyping, before you’re ready to decide how to handle errors. They leave clear markers in your code for when you’re ready to make your program more robust.

* When your code performs an operation that could put a user at risk if it’s called using invalid values, your code should verify the values are valid first and panic if the values aren’t valid. This is mostly for safety reasons: attempting to operate on invalid data can expose your code to vulnerabilities. 