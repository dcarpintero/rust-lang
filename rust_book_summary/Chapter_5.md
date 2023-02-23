*Summary of [https://doc.rust-lang.org/book/ch05-00-structs.html](https://doc.rust-lang.org/book/ch05-00-structs.html)*

# Chapter 5 - Using Structs to Structure Related Data

Structs are similar to tuples, in that both hold multiple related values. Like tuples, the pieces of a struct can be different types. Unlike with tuples, in a struct you’ll name each piece of data so it’s clear what the values mean. Adding these names means that structs are more flexible than tuples: you don’t have to rely on the order of the data to specify or access the values of an instance.

```
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");
```

## Using the Field Init Shorthand

```
    fn build_user(email: String, username: String) -> User {
        User {
            active: true,
            username,
            email,
            sign_in_count: 1,
        }
    }
```

## Creating Instances from Other Instances with Struct Update Syntax

```
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
    // user1 can no longer be used
```

## Using Tuple Structs Without Named Fields to Create Different Types

Tuple structs are useful when naming each field as in a regular struct would be verbose or redundant.

```
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
```