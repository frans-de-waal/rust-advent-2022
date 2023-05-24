# Notes

*[Bookmark](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html)*

---

## Installation

### `rustup`

https://doc.rust-lang.org/book/ch01-01-installation.html#installation

A command line tool for managing Rust versions and associated tools.

- Updating Rust:  `$ rustup update`

### Local docs

The installation of Rust also includes a local copy of the documentation so that you can read it offline. Run `rustup doc` to open the local documentation in your browser.

## Intro

- Rust source files use the `.rs` file extension.
    - Convention is to use underscores for whitespace in filenames.
- `main` function is always the first code that runs in every executable Rust program.
- `rustfmt` code formatter
    - 4 space indentation convention
- Using `!` means calling a macro in stead of a function.
- Compiling and Running are seperate steps.
    - Compile using `rustc main.rs`. It outputs a binary executable.
    - Run the application by calling the executable `./main`.
- **Cargo** is Rust's build system and package manager.
    - Cargo handles a lot of tasks for you, such as building your code, downloading the libraries your code depends on, and building those libraries.
- Create a new project using Cargo: `cargo new project_name`
    - Creates a `Cargo.toml` file, Cargo's configuration file (*see [TOML](https://toml.io/en/)*).
    - Creates a `src` directory with a `main.rs` file.
    - Initialises a Git repository if not already in one.
    - Use `cargo init` to initialise a new project in an existing directory.
- Building and running a Cargo project
    - Build: `cargo build`
        - Creates an executable `target/debug/` directory (*default build is a debug build*).
        - Creates a `Cargo.lock` file, keeps track of the exact version dependencies in the project.
    - Run: `cargo run` compiles the code and runs the resultant executable.
    - `cargo check` quickly checks your code to make sure it compiles but doesn’t produce an executable.
    - Build (and optimise) for release: `cargo build --release`. 
        - Will create an executable in `/target/release`
- [Reserved words in Rust](https://doc.rust-lang.org/book/appendix-01-keywords.html)

## Package configuration

### `[package]`

Section defining the project.

- `name`: Name of the project.
- `version`: Version of the project.
- `edition`: Edition of Rust to use.

### `[dependencies]`

Dependencies for the the project. Packages of code are referred to as *crates*.

## Variables and Mutability

[Docs](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html)

```rust
let x = 5;
```

Variables are immutable by default.

- Use the `mut` keyword before the variable name to make mutable.

## Constants

Constant are always immutable.

```rust
const X: u32 = 5;
```

- The type of the value *must* be annotated.
- Constants may be set only to a constant expression, not the result of a value that could only be computed at runtime.

## Data Types

[Docs](https://doc.rust-lang.org/book/ch03-02-data-types.html)

### Strings

[Docs](https://doc.rust-lang.org/book/ch08-02-strings.html#storing-utf-8-encoded-text-with-strings)

String slice: `str`

Split a string:
- By separator: `str.split('separator')`
- By whitespace: `str.split_whitespace()` or `str.split_ascii_whitespace()`
- By line: `str.lines()`

## Shadowing

## Reading files

[Docs](https://doc.rust-lang.org/book/ch12-02-reading-a-file.html)

`std::fs` to handle files.

`fs::read_to_string(FILE_PATH)` to read to a string.
