# Common Programming Concepts

## Variables, Mutability, Constants

- by default, *let* variables are immutable, add *mut* to make them mutable:
```
    let mut x = 5;
    x = 7;
```

- constants, declared by *const*, are always immutable.
```
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```

## Shadowing

- Refers to declaring a new variable with the same name as a previous variable:
```
    let x = 5;
    let x = x + 1;                                                // x is 6
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");    // x is 12
    }
```

- Shadowing allows to change the type of the value but reuse the same name:
```
    let spaces = "   ";
    let spaces = spaces.len();
```

## Data Types (Scalar and Compound)

- Rust is a statically typed language, which means that it must know the types of all variables at compile time. 

### Scalar

Represents a single value.
- Integers (i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize)
- Floating-point (f32, f64)
- Booleans (true, false)
- Characters (char)

### Compound

- Tuple:
```
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let six_point_four = x.1;
```

- Arrays (fixed length, allocated on the stack):
```
    let a: [i32; 5] = [1, 2, 3, 4, 5];
```