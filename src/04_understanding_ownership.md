# Understanding Ownership

*   Pushing to the stack is faster than allocating on the heap
*   Accessing data in the heap is slower than accessing data on the stack because you have to follow a pointer to get there
*   When your code calls a function, the values passed into the function (including, potentially, pointers to data on the heap) and the function’s local variables get pushed onto the stack. When the function is over, those values get popped off the stack.
*   There can only be one owner of a value at a time
*   When owner goes out of scope, the value will be dropped

## The String Type

*   We use string literals(str) to store data that is known at compile time else we use the "String" type

*   `String` can be mutated but not literals

*   With the String type, in order to support a mutable, growable piece of text, we need to allocate an amount of memory on the heap, unknown at compile time, to hold the contents. This means:

    *   The memory must be requested from the memory allocator at runtime.
    *   We need a way of returning this memory to the allocator when we’re done with our String.
    *   That first part is done by us: when we call String::from, its implementation requests the memory it needs.

*   Common Methods

    *   `variable.push_str(", world!");`

     <!-- TODO: Add more -->

*   Let's look at the ways variables and data interact

```rust
{{#rustdoc_include ../listings/04_understanding_ownership/string-memory-representation/src/main.rs:all}}
```

<details>
<summary>Output</summary>

```console
{{#include ../listings/04_understanding_ownership/string-memory-representation/output.txt}}
```

</details>

*   This is how the memory representation looks like for:

```rust
{{#rustdoc_include ../listings/04_understanding_ownership/string-memory-representation/src/main.rs:s1}}
```

![s1 points to data on heap](images/04/s1_heap.png)

*   This is how the memory representation would look like if you clone `s1` i.e.

    ```rust
    let s2 = s1.clone()
    ```

    ![s2 is a copy of s1](images/04/clone_s1.png)

*   If the memory representation looked like below then that would be a bug. When one of the variables goes out of scope and the value is dropped, the other variable would still be pointing at that value which was dropped.

    ![Double Free Error](images/04/double_free_error.png)

*   The correct memory representation would like like this instead. `s1` is moved into `s2` and is no longer valid.

    ![s1 moved into s2](images/04/moved_to_s2.png)

*   Scalar types like `bool`, `u32`, `(u32, f64)`, etc. implement the `Copy` trait.

*   The code below works and `x` wasn't moved into `y` because the size of types like integers is known at compile time so they are stored on stack.

```rust
{{#rustdoc_include ../listings/04_understanding_ownership/scalar-type-memory-representation/src/main.rs:all}}
```

<details>
<summary>Output</summary>

```console
{{#include ../listings/04_understanding_ownership/scalar-type-memory-representation/output.txt}}
```

</details>

## Ownership and Functions

The mechanics of passing a value to a function are similar to those when assigning a value to a variable.

```rust
{{#rustdoc_include ../listings/04_understanding_ownership/ownership-and-functions/src/main.rs:all}}
```

<details>
<summary>Output</summary>

```console
{{#include ../listings/04_understanding_ownership/ownership-and-functions/output.txt}}
```

</details>

## References and Borrowing

*   All references are stored on the stack. Although you can store references on
    the heap, you might need another reference to refer to it. Basically, you need a
    reference to refer to anything on the heap.
*   Unlike a pointer, a reference is guaranteed to point to a valid value of a particular type for the life of that reference.

```rust
{{#rustdoc_include ../listings/04_understanding_ownership/references-and-borrowing/src/main.rs:all}}
```

<details>
<summary>Output</summary>

```console
{{#include ../listings/04_understanding_ownership/references-and-borrowing/output.txt}}
```

</details>

*   The memory representation for the above code looks like:

![s points to s1](images/04/reference.png)

*   We will look at dereferencing(`*`) in later chapters <!-- TODO: Add a link to the dereference operator chapter -->

*   To modify a borrowed value, make a reference mutable

*   [This Section](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html#mutable-references) from the book marks the point of confusion

*   If you have a mutable reference to a value, you can have no other references to that value.

```rust
{{#rustdoc_include ../listings/04_understanding_ownership/references-and-borrowing-2/src/main.rs:all}}
```

<details>
<summary>Error</summary>

```console
{{#include ../listings/04_understanding_ownership/references-and-borrowing-2/output.txt}}
```

</details>

The benefit of having this restriction is that Rust can prevent data races at compile time. A data race is similar to a race condition and happens when these three behaviors occur:

*   Two or more pointers access the same data at the same time.
*   At least one of the pointers is being used to write to the data.
*   There’s no mechanism being used to synchronize access to the data.

As always, we can use curly brackets to create a new scope, allowing for multiple mutable references, just not simultaneous ones:

```rust
{{#rustdoc_include ../listings/04_understanding_ownership/references-and-borrowing-3/src/main.rs:all}}
```

<details>
<summary>Error</summary>

```console
{{#include ../listings/04_understanding_ownership/references-and-borrowing-3/output.txt}}
```

</details>

Rust enforces a similar rule for combining mutable and immutable references.

```rust
{{#rustdoc_include ../listings/04_understanding_ownership/references-and-borrowing-4/src/main.rs:all}}
```

<details>
<summary>Error</summary>

```console
{{#include ../listings/04_understanding_ownership/references-and-borrowing-4/output.txt}}
```

</details>

Whew! We also cannot have a mutable reference while we have an immutable one to the same value.

Users of an immutable reference don’t expect the value to suddenly change out from under them! However, multiple immutable references are allowed because no one who is just reading the data has the ability to affect anyone else’s reading of the data.

Note that a reference’s scope starts from where it is introduced and continues through the last time that reference is used. So this code compiles:

```rust
{{#rustdoc_include ../listings/04_understanding_ownership/references-and-borrowing-5/src/main.rs:all}}
```

<details>
<summary>Output</summary>

```console
{{#include ../listings/04_understanding_ownership/references-and-borrowing-5/output.txt}}
```

</details>

## Dangling References

```rust
{{#rustdoc_include ../listings/04_understanding_ownership/dangling-references/src/main.rs:all}}
```

<details>
<summary>Error</summary>

```console
{{#include ../listings/04_understanding_ownership/dangling-references/output.txt}}
```

</details>

Specifically:

<mark>this function's return type contains a borrowed value, but there is no value for it to be borrowed from</mark>

## Slice Type: Different kind of references

Let's write a program to output the first word of a string:

```rust
{{#rustdoc_include ../listings/04_understanding_ownership/l01-get-first-word-index/src/main.rs:all}}
```

<details>
<summary>Output</summary>

```console
{{#include ../listings/04_understanding_ownership/l01-get-first-word-index/output.txt}}
```

</details>

*   In the above program `word` still holds the value `3` which is meaningless after
    the `String` gets cleared. If we use `word` further, our program will panic.

*   This can be avoided if our function returned a slice.

```rust
{{#rustdoc_include ../listings/04_understanding_ownership/l02-get-first-word-slice/src/main.rs:all}}
```

<details>
<summary>Error</summary>

```console
{{#include ../listings/04_understanding_ownership/l02-get-first-word-slice/output.txt}}
```

</details>

*   The above program doesn't compile

*   If we have a string slice, we can pass that directly. If we have a String, we can pass a slice of the String or a reference to the String. This flexibility takes advantage of deref coercions, a feature we will cover in the [ "Implicit Deref Coercions with Functions and Methods" ](https://doc.rust-lang.org/book/ch15-02-deref.html#implicit-deref-coercions-with-functions-and-methods) section of Chapter 15.

*   This program can also be improved by using `&str` instead of `&String` in the function signature.

*   Defining a function to take a string slice instead of a reference to a String makes our API more general and useful without losing any functionality

### Other Slices

We can use slices to refer to part of an array:

```rust
{{#rustdoc_include ../listings/04_understanding_ownership/l03-other-slices/src/main.rs:all}}
```

In the above program both `&[2, 3]` and `[2, 3]` because.

[![Amanda Rust Matrix Server](./images/04/partialeq.png)](https://doc.rust-lang.org/stable/std/primitive.array.html#impl-PartialEq<%26\[B]>-for-\[A%3B%20N])
