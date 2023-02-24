# Enums and Pattern Matching

Enums allow to define a type by enumerating its possible variants. Each variant can have different types and amounts of associated data. It is also possible to define methods on enums.

```
    enum IpAddrKind {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
```

## The Option Enum and Its Advantages Over Null Values

The Option enum type encodes the very common scenario in which a value could be something or it could be nothing. This functionality can prevent bugs that are extremely common in other programming languages. Rust doesn’t have the null feature that many other languages have. 

*In his 2009 presentation “Null References: The Billion Dollar Mistake,” Tony Hoare, the inventor of null, referred to the creation of the Null reference as his billion-dollar mistake.*

As such, Rust does not have nulls, but it does have an enum that can encode the concept of a value being present or absent. This enum is Option<T>, and it is defined by the standard library as follows:

```
    enum Option<T> {
        None,
        Some(T),
    }

    let x: Option<u32> = Some(2);
    assert_eq!(x.is_some(), true);

    let x: Option<u32> = None;
    assert_eq!(x.is_some(), false);
```

In order to have a value that can possibly be null, you must explicitly opt in by making the type of that value Option<T>.

## The Match Control Flow Construct

Rust has an extremely powerful control flow construct called match that allows you to compare a value against a series of patterns (literal values, variable names, wildcards) and then execute code based on which pattern matches.

```
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => {
                println!("Lucky penny!");
                1
            }
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }
```

## Concise Control Flow with if let

```
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }

    // The following code behaves the same as the previous:
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

```