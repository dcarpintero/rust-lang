# Cargo and Crates

## Release Profiles

```
$ cargo build
    Finished dev [unoptimized + debuginfo] target(s) in 0.0s
$ cargo build --release
    Finished release [optimized] target(s) in 0.0s
```

- The opt-level setting controls the number of optimizations Rust will apply to your code, with a range of 0 to 3. Applying more optimizations extends compiling time, so if you’re in development and compiling your code often, you’ll want fewer optimizations to compile faster even if the resulting code runs slower.

```
[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3
```

## Publishing a Crate to Crates.io

- Documentation comments use three slashes, ///, instead of two and support Markdown notation for formatting the text. Typically documentation comments include: examples, panics, errors, and safety.

- The style of doc comment //! adds documentation to the item that *contains* the comments rather than to the items following the comments. Useful for describing crates and modules especially. 

- The command *cargo doc* runs the rustdoc tool distributed with Rust and puts the generated HTML documentation in the target/doc directory:

```
    cargo doc
    cargo doc --open
```

### Documentation Comments as Tests

- Running cargo test runs also the code examples in the documentation as tests!

### Exporting a Convenient Public API with **pub use**

- **pub use** re-exports items to make a public structure that's different from the private structure. Choosing **pub use** gives you flexibility in how you structure your crate internally and decouples that internal structure from what you present to your users.

### Publishing to Crates.io

```
    // create an account on crates.io
    cargo login API_KEY
```

- Add metadata to Cargo.toml. The [Linux Foundation’s Software Package Data Exchange (SPDX)](https://spdx.org/licenses/)lists the identifiers you can use as a license.

```
[package]
    name = "guessing_game"
    version = "0.1.0"
    edition = "2021"
    description = "A fun game where you guess what number the computer has chosen."
    license = "MIT OR Apache-2.0"

[dependencies]
```

-  Publishing to [crates.io](https://crates.io/) is permanent:

```
    cargo publish
```

- Yanking a version prevents new projects from depending on that version while allowing all existing projects that depend on it to continue. Essentially, a yank means that all projects with a *Cargo.lock* will not break, and any future *Cargo.lock* files generated will not use the yanked version.

```
    cargo yank --vers 1.0.1
```