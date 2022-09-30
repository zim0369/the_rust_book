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
{{#rustdoc_include ../listings/05_using_structs_to_structure_related_data/l01-structs/src/main.rs:25}}
```

*   Accessing these fields will cause an error:

```rust
{{#rustdoc_include ../listings/05_using_structs_to_structure_related_data/l01-structs/src/main.rs:29}}
```

*   The field `new` is still valid and can be accessed.

```rust
{{#rustdoc_include ../listings/05_using_structs_to_structure_related_data/l01-structs/src/main.rs:30}}
```

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
