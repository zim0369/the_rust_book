# Enums And Pattern Matching

Enums give you a way of saying a value is one of a possible set of values.

An IpAddrKind enumeration listing the possible kinds an IP address can be, V4 and V6. These are the variants of the enum:

```rust
{{#rustdoc_include ../listings/06-enums-and-pattern-matching/enums/src/main.rs:enums}}
```

The name of each enum variant that we define also becomes a function that constructs an instance of the enum. That is, IpAddr::V6() is a function call that takes a String argument and returns an instance of the IpAddr type. We automatically get this constructor function defined as a result of defining the enum.

You can also do some funky stuff. For example:

```rust
{{#rustdoc_include ../listings/06-enums-and-pattern-matching/enums/src/main.rs:funky}}
```

<!-- TODO: Understand the use case for the `Something` in above code  -->

Here, `Nothing` isn't even a unit struct. It has no data associated with it at all.

We’re also able to define methods on enums.

## Null

The problem with null values is that if you try to use a null value as a not-null value, you’ll get an error of some kind. Because this null or not-null property is pervasive, it’s extremely easy to make this kind of error.

However, the concept that null is trying to express is still a useful one: a null is a value that is currently invalid or absent for some reason.

The problem isn’t really with the concept but with the particular implementation. As such, Rust does not have nulls, but it does have an enum that can encode the concept of a value being present or absent. This enum is Option<T>, and it is [defined by the standard library](https://doc.rust-lang.org/std/option/enum.Option.html) as follows:

```rust
enum Option<T> {
    None,
    Some(T),
}
```

The <T> syntax represents a generic type.

<!-- TODO: Add a good enum(with "match") example covering a tonne of fun -->

## if let

You can think of if let as syntax sugar for a match that runs code when the value matches one pattern and then ignores all other values.

*   We can include an else with an if let.

```rust
{{#rustdoc_include ../listings/06-enums-and-pattern-matching/if-let/src/main.rs:all}}
```

<details>
<summary>Output</summary>

```console
{{#include ../listings/06-enums-and-pattern-matching/if-let/output.txt}}
```

</details>

<!-- TODO -->
