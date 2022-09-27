# Getting Started

## Writing a program

This is how a "Hello Ferris!" program looks like in Rust:

```rust
fn main() {
    println!("Hello, Ferris!");
}
```

Create a file named "hello.rs" and then open your terminal in that directory. Now compile your program with `rustc hello.rs` and run it with `./hello`. It will output: `Hello, Ferris!`(**Ferris** is the Rust programming language's unofficial mascot).

<p style="text-align:center";>
<img src="images/01/cuddlyferris.svg" alt="Ferris" width="200" style="center">
</p>

- In our program:

  - we have the main function starting with `fn main() {` and ending at `}`. It takes no parameters, which is why the `()`(parentheses) are empty.

  - `println!` calls a Rust macro. If it had called a function instead, it would be entered as `println` (without the !).

  - We pass this `"Hello, Ferris!"` as an argument to `println!`, and the string is printed to the screen.

  - [Read about string formatting here](https://doc.rust-lang.org/std/fmt/)

  - We end the line with a semicolon (`;`), which indicates that this expression is over.

  - **NOTE:** _The order in which functions are written doesn't matter but macros have to appear before they are used._

## Cargo

Cargo is Rust's build system and package manager. We use it when we have to use any dependencies. Our previous program was simple so we did not need cargo.

- Let's get familiar with cargo:

  - First make sure that cargo is installed. Try running: `cargo --version`
  - Help: `cargo --help` or
  - If manpages are installed: `man cargo`

- Creating our first cargo project:

  - Run `cargo new hello_cargo`: This will create a new cargo project directory named `hello_cargo` which contains:

    - "**Cargo.toml**" file is Cargo's configuration file written in TOML(Tom's Obvious, Minimal Language) format:

    ```toml
    [package]
    name = "hello_cargo"
    version = "0.1.0"
    edition = "2021"

    [dependencies]
    ```

    The first line, **\[package]**, is a section heading that indicates that the following statements are configuring a package. As we add more information to this file, we’ll add other sections.

    The next three lines set the configuration information Cargo needs to compile your program: the name, the version, and the edition of Rust to use.

    The last line, **\[dependencies]**, is the start of a section for you to list any of your project’s dependencies. In Rust, packages of code are referred to as crates. We won’t need any other crates for this project, but we will use it later.

    - "**Cargo.lock**" file keeps track of the exact versions of dependencies in your project. This project doesn’t have dependencies, so the file is a bit sparse. You won’t ever need to change this file manually; Cargo manages its contents for you.

    - "**src/main.rs**": Cargo contains a dir "src" for source files. The top-level project directory is just for README files, license information, configuration files, and anything else not related to your code. By default the "main.rs" file has:

    ```rust
    fn main() {
        println!("Hello, world!");
    }
    ```

  - While you are inside your Cargo project dir. You can do the following:
    - Build(compile) our program with `cargo build`
    - Run it with `./target/debug/hello_cargo`
    - Or both build and run with `cargo run`
    - Or just check our program for errors with `cargo check` without having to compile and or run. It just checks your code but doesn't create an executable
    - When your project is finally ready for release, you should use `cargo build --release` to compile it with optimizations. This creates an executable under `target/release` instead of `target/debug`. It is slower as compared to just `cargo build`

## Tips

You might come across some rusty projects on github that require you to build the project from scratch to install it for which you usually have to follow the steps similar to:

```console
$ git clone example.org/someproject
$ cd someproject
$ cargo build
```

Usually people just open an editor to edit the source code and a terminal to build the project in which case you could just use "cargo-watch". It watches over your project's source for changes, and runs Cargo commands when they occur.

Install: `cargo install cargo-watch`\
Use: `cargo watch`(inside the project dir)

Now all you have to do is edit the source code and save the file if you have to build the changes.

<!-- TODO: cargo clippy -->
