# Chapter 5 - Using Structs to Structure Related Data

Structs are similar to tuples, in that both hold multiple related values. Like tuples, the pieces of a struct can be different types. Unlike tuples, in a struct each piece of data is named. This results in more flexibily when accessing the values of an instance.

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

## Methods Syntax

Methods are similar to functions, but defined within the context of a struct.

```
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }

        fn square(size: u32) -> Self {    // Self is an alias for Rectangle
            Self {
                width: size,
                height: size,
            }
        }
    }

    let sq = Rectangle::square(3);
```

All functions defined within an impl block are called associated functions because they're associated with the type named after the impl. 

It is also possible to define associated functions not having self as their first parameter
(and thus are not methods) because they don’t need an instance of the type to work with (e.g. String::from).

## Summary

Structs let you create custom types that are meaningful for your domain. By using structs, you can keep associated pieces of data connected to each other and name each piece to make your code clear. In impl blocks, you can define functions that are associated with your type, and methods are a kind of associated function that let you specify the behavior that instances of your structs have.