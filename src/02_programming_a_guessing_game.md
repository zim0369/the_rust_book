# Programming A Guessing Game

We’ll implement a guessing game. Here’s how it works: the program will generate a random integer between 1 and 100. It will then prompt the player to enter a guess. After a guess is entered, the program will indicate whether the guess is too low or too high. If the guess is correct, the game will print a congratulatory message and exit.

Create a new project dir with `cargo new guess` and now let's think how we're going to implement the core functionality, generating a random number. Here we will use the capabilities of cargo that we didn't use before. Google search "[ rust crate rand ](https://crates.io/crates/rand)". This crate helps us generate random numbers. You'll find instructions there on how to add this dependency to our program. Let's do that.

We need to edit our "Cargo.toml" file to add a dependency:

[Read about semantic versioning here](https://semver.org/)

Comments in TOML start with `#`(pound). The line `rand = "0.8.4"` adds the rand crate from [crates.io](https://crates.io/) and any other crates `rand` requires.

For our game we need to:

- To obtain user input we need the io library with `use std::io`

- We add the line `use rand::Rng`. The `Rng` trait defines methods that random number generators implement, and this trait must be in scope for us to use those methods.

- By default, Rust has a set of items defined in the standard library that it brings into the scope of every program. This set is called the prelude, and you can see everything in it in the standard library documentation. The io library isn't present in the prelude

Filename: src/main.rs

```rust
{{#rustdoc_include ../listings/02_guessing_game/guessing_game/src/main.rs:all}}
```

- Both variables and references are immutable by default while constants are always immutable
- `read_line()` returns `Result` which is an enum type that has the variants `Ok` and `Err` and it's own methods defined on it.
- **Note:** You won’t just know which traits to use and which methods and functions to call from a crate, so each crate has documentation with instructions for using it. Another neat feature of Cargo is that running the cargo doc --open command will build documentation provided by all of your dependencies locally and open it in your browser. If you’re interested in other functionality in the rand crate, for example, run cargo doc --open and click rand in the sidebar on the left.
