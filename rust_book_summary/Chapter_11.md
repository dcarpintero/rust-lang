# Automated Tests

In his 1972 essay “The Humble Programmer,” Edsger W. Dijkstra said that “Program testing can be a very effective way to show the presence of bugs, but it is hopelessly inadequate for showing their absence.” That doesn’t mean we shouldn’t try to test as much as we can!

- Each test is run in a new thread.

- Under the surface, the assert_eq! and assert_ne! macros use the operators == and !=, respectively. When the assertions fail, these macros print their arguments using debug formatting, which means the values being compared must implement the *PartialEq* and *Debug* traits. All primitive types and most of the standard library types implement these traits. 

- For self-defined structs and enums implement *PartialEq* to assert equality of those types. And *Debug* to print the values when the assertion fails. Because both traits are derivable traits, add the #[derive(PartialEq, Debug)] annotation to the struct or enum definition.

## Annotations

The **cargo test** command builds a test runner binary that runs the annotated functions and reports on whether each test function passes or fails.

```
    #[test]
    #[should_panic(expected = "failure message")]
    #[ignore]
    #[cfg(test)]    // as unit tests are defined in each file with the code they are testing
```

## Macros

```
    assert!
    assert_eq!
    assert_ne!
    panic!

    // Any arguments specified after the required arguments are passed along to the format! 

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`",
            result
        );
    }

    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
```

## Running Tests in Parallel or Consecutively

By default tests are run in parallel using threads. Thus, make sure the tests don't depend on each other or on any shared state, including a shared environment, such as the current working directory or environment variables.

```
    cargo test
    cargo test -- --test-threads=1    // set the number of test threads to 1, do not to use any parallelism
    cargo test -- --show-output       // show the output of successful tests
    cargo test [TEST_NAME]            // only run tests containing this string in their names
    cargo test -- --ignored           // only ignored tests
    cargo test -- --include-ignored   // all tests
```

## Unit Tests

The convention is to create a module named tests in each file to contain the test functions and to annotate the module with cfg(test).

No that Rust's privacy rules do allow you to test private functions. In other words, all unit tests in a given file have access to that file's private functions, regardless of being in a mod tests or not.

## Integration Tests

To create integration tests, you first need a *tests* directory. 

Each file in the tests directory is then compiled as its own separate crate. However, files in subdirectories of the tests directory don’t get compiled as separate crates or have sections in the test output.