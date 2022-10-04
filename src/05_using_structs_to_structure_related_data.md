# Using Structs To Structure Related Data

## Structs

*   Structs are similar to tuples, in that both hold multiple related values.

```rust
{{#rustdoc_include ../listings/05_using_structs_to_structure_related_data/l01-structs/src/main.rs:all}}
```

<details>
<summary>Output</summary>

```console
{{#include ../listings/05_using_structs_to_structure_related_data/l01-structs/output.txt}}
```

</details>

*   field `_name` is prefixed with `_` to avoid warnings because we aren't going to use it.

*   The field `scores` is a tuple struct

*   On this line the fields `name` & `scores` were moved into `gir` because `String`
    and `scores` do not implement the `Copy` trait:

```rust
{{#rustdoc_include ../listings/05_using_structs_to_structure_related_data/l01-structs/src/main.rs:27}}
```

*   Accessing these fields(moved) will cause an error:

```rust
{{#rustdoc_include ../listings/05_using_structs_to_structure_related_data/l01-structs/src/main.rs:31}}
```

*   The field `new` is still valid and can be accessed.

```rust
{{#rustdoc_include ../listings/05_using_structs_to_structure_related_data/l01-structs/src/main.rs:32}}
```

*   Structs can't be formatted with default formatter or `{:?}` since Display &
    Debug traits both aren't implemented for our struct.

```rust
{{#rustdoc_include ../listings/05_using_structs_to_structure_related_data/l01-structs/src/main.rs:34:36}}
```

*   But we added `#[derive(Debug)]` on top of both the structs. Rust provides
    more "derivable"(To use with `derive` attribute) traits.

```rust
{{#rustdoc_include ../listings/05_using_structs_to_structure_related_data/l01-structs/src/main.rs:04:05}}
```

```rust
{{#rustdoc_include ../listings/05_using_structs_to_structure_related_data/l01-structs/src/main.rs:07:08}}
```

*   So we can print the structs with `dbg!` or using `{:?}` in `println!`. It prints to the `stderr` as opposed to `println!` which prints to `stdout`

```rust
{{#rustdoc_include ../listings/05_using_structs_to_structure_related_data/l01-structs/src/main.rs:38}}
```

*   We can also use `{:#?}` for pretty print

*   There are [more attributes other than `derive`](https://doc.rust-lang.org/reference/attributes.html)

*   Unit-like structs can be useful when you need to implement a trait on some type but don’t have any data that you want to store in the type itself:

```rust
struct AlwaysEqual;

fn main() {
    let subject = AlwaysEqual;
}
```

## Ownership of Struct Data

*   In the User struct definition we used the owned String type rather than the `&str` string slice type. This is a deliberate choice because we want each instance of this struct to own all of its data and for that data to be valid for as long as the entire struct is valid.

*   It’s also possible for structs to store references to data owned by something else, but to do so requires the use of lifetimes, a Rust feature that we’ll discuss in Chapter 10. Lifetimes ensure that the data referenced by a struct is valid for as long as the struct is. Let’s say you try to store a reference in a struct without specifying lifetimes, like the following; this won’t work:

```rust
{{#rustdoc_include ../listings/05_using_structs_to_structure_related_data/l02-structs-lifetimes/src/main.rs:all}}
```

The compiler will complain that it needs lifetime specifiers:

<details>
<summary>Error</summary>

```console
{{#include ../listings/05_using_structs_to_structure_related_data/l02-structs-lifetimes/output.txt}}
```

</details>

## Method Syntax

*   First parameter is always self, which represents the instance of the struct the method is being called on.

```rust
{{#rustdoc_include ../listings/05_using_structs_to_structure_related_data/l03-method-syntax/main.rs:all}}
```

The compiler will complain that it needs lifetime specifiers:

<details>
<summary>Error</summary>

```console
{{#include ../listings/05_using_structs_to_structure_related_data/l03-method-syntax/output.txt}}
```

</details>

*   In the signature for area, we use \&self instead of rectangle: \&Rectangle. The \&self is actually short for self: \&Self. Within an impl block, the type Self is an alias for the type that the impl block is for. Methods must have a parameter named self of type Self for their first parameter, so Rust lets you abbreviate this with only the name self in the first parameter spot. Note that we still need to use the & in front of the self shorthand to indicate this method borrows the Self instance, just as we did in rectangle: \&Rectangle. Methods can take ownership of self, borrow self immutably as we’ve done here, or borrow self mutably, just as they can any other parameter.

*   If we wanted to change the instance that we’ve called the method on as part of what the method does, we’d use \&mut self as the first parameter. Having a method that takes ownership of the instance by using just self as the first parameter is rare; this technique is usually used when the method transforms self into something else and you want to prevent the caller from using the original instance after the transformation.
