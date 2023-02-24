# Managing Growing Projects with Packages, Crates, and Modules

- Packages: A Cargo feature that lets you build, test, and share crates
- Crates: A tree of modules that produces a library or executable
- Modules and use: Let you control the organization, scope, and privacy of paths
- Paths: A way of naming an item, such as a struct, function, or module

## Packages and Crates

A *crate* is the smallest amount of code that the Rust compiler considers at a time.

- **Binary** crates: programs that compile to an executable (such as a command-line program or a server). Each must have a function called main that defines what happens when the executable runs.

- **Library** crates don’t have a main function, and they don’t compile to an executable. Instead, they define functionality intended to be shared with multiple projects.

- A **package** is a bundle of one or more crates that provides a set of functionality. A package contains a Cargo.toml file that describes how to build those crates. Cargo is actually a package that contains the binary crate for the command-line tool, and a library crate that the binary crate depends on.

## Defining Modules to Control Scope and Privacy

- [https://doc.rust-lang.org/stable/book/ch07-02-defining-modules-to-control-scope-and-privacy.html](https://doc.rust-lang.org/stable/book/ch07-02-defining-modules-to-control-scope-and-privacy.html)