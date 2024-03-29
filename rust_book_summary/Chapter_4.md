# Chapter 4 - Understanding Ownership

Rust ownership defines **a set of rules, checked at compile time, that govern how a Rust program manages (heap) memory**  (this differs from garbage collectors and programmatic (de)allocations). 

## Stack and Heap

### Stack

- Data with a known, fixed size.
- LIFO 
- The pointer to the stack is a known, fixed size.
- Function parameters are pushed onto the stack, and popped off the stack when the function is over.

### Heap

- Data with an unknown size at compile time, or a size that might change.
- Addressable by an allocated pointer.
- Allocation and Access to the heap is slower since it requires: (i) finding a big enough space, (ii) bookkeeping to prepare for the next allocation, and (iii) following a pointer to get there.

## Ownership Rules

- Each value in Rust has an *owner*.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.

## The String Type

- It manages data allocated on the heap, and as such is able to store an amount of text that is unknown at compile time.

```
    {
        let mut s = String::from("hello"); // memory is requested to the allocator

        s.push_str(", world!"); // appends a literal to a String

        println!("{}", s); // `hello, world!` 
    }
    // s, which owns the String allocated memory goes out of scope
    // thus, the memory is returned to the allocator by calling the *drop* function
```

- In the case of an inmutable string literal, the contents are known at compile time, so the text is hardcoded directly into the final executable. This makes string literals fast and efficient.

- With the String type, in order to support a mutable, growable piece of text, it is needed to allocate an amount of memory on the heap, unknown at compile time.

```
    let s1 = String::from("hello");
    let s2 = s1; // s1 is moved into s2
    // To ensure memory safety, Rust considers s1 as no longer valid.
```

<p align="center">
    <img src="./img/trpl04-04.svg" width="250" />
</p>
<p align="center">Representation in memory of the variable s2 that has a copy of the pointer, length, and capacity of s1 (source: https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#variables-and-data-interacting-with-move)</p>

## Variables and Data Interacting with Clone

The *Clone* method deeply copies the heap data of the String, not just the stack data.

```
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
```

## Stack-Only Data: Copy

Types such as integers that have a known size at compile time are stored entirely on the stack, resulting in quick copies of the actual values.

```
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);
```

Rust has a special annotation called the *Copy* trait that can be placed on types that are stored on the stack, as integers are. If a type (typically any group of simple scalar values) implements the *Copy* trait, variables that use it do not move, but rather are trivially copied, making them still valid after assignment to another variable.

## Return Values and Scope

```
fn main() {
    let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope

    a_string  // a_string is returned and moves out to the calling function
}
```

The ownership of a variable follows the same pattern every time: assigning a value to another variable moves it. When a variable that includes data on the heap goes out of scope, the value will be cleaned up by drop unless ownership of the data has been moved to another variable.

**However, Rust has a feature for using a value without transferring ownership, called references.**

## 4.2 References and Borrowing

> At any given time, you can have either one mutable reference or any number of immutable references.

> References must always be valid.

A reference is like a pointer but it guarantees to point to a valid value of a particular type for the life of that reference.

```
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
```

The &s1 syntax lets create a reference that refers to the value of s1 but does not own it. Because it does not own it, the value it points to will not be dropped when the reference stops being used.

The action of creating a reference is called *borrowing*.

References (as variables) are immutable by default, to allow modifying a borrowed value ***a mutable reference*** would be needed:

```
    let mut s = String::from("hello");
    change(&mut s);

    fn change(some_string: &mut String) {
```

Mutable references have one big restriction: if you have a mutable reference to a value, you can have no other references to that value within the same scope, which is determined by curly brackets or by use (e.g. if the variable is no longer used the scope ends). This prevents data races at compile time. A data race is similar to a race condition and happens when these three behaviors occur:

- Two or more pointers access the same data at the same time.
- At least one of the pointers is being used to write to the data.
- There's no mechanism being used to synchronize access to the data.

### Dangling References

In languages with pointers, it’s easy to erroneously create a dangling pointer — a pointer that references a location in memory that may have been given to someone else — by freeing some memory while preserving a pointer to that memory. In Rust, by contrast, the compiler guarantees that references will never be dangling references: if you have a reference to some data, the compiler will ensure that the data will not go out of scope before the reference to the data does.

## 4.3 The Slice Type

*Slices* let you reference a contiguous sequence of elements in a collection rather than the whole collection. A slice is a kind of reference, so it does not have ownership.

```
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
```

The type that signifies “string slice” is written as &str:

```
    fn first_word(s: &str) -> &str {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }

        &s[..]
    }
```

<p align="center">
    <img src="./img/trpl04-06.svg" width="250" />
</p>
<p align="center">String slice referring to part of a String (source: https://doc.rust-lang.org/book/ch04-03-slices.html)</p>