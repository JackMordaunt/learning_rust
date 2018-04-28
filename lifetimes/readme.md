# Lifetimes

- Most of the time lifetimes are implicit and inferred just as most types are inferred.
- Lifetimes prevent dangling references.
- The borrow checker compares scopes to determine all borrows are valid.

## Generic Lifetimes in Functions

A function that borrows two or more arguments and returns data from any one of them is an ambiguous case: which lifetime is the return asociated with?

The borrow checker needs some help.

```rust
// This wont compile.
// The returned data can either be from x, or from y. The borrow checker needs our help to determine the correct lifetime of the return.
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

## Lifetime Annotation Syntax

- Lifetime annotations relate lifetimes of multiple references to each other.
- The lifetime parameter starts with an apostraphe `'`.
- The names of the lifetime parameters are usually all lowercase and short.
- `'a` is the most common default.
- Lifetime parameters go after the `&` of a reference.

```rust
&i32 // a reference.
&'a i32 // a reference with an explicit lifetime.
&'a mut i32 // a mutabile reference with an explicit lifetime.
```

So we tell rust which variables have what lifetimes.

```rust
fn longest<a'>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

For some lifetime `'a` both of the function parameters will ilve at least as long as the lifetime `'a`.

By annotating lifetimes you're not changing the lifetimes; you're simply associating lifetimes between variables.

```rust
// This compiles because all the values exist for the length of the scope required.
fn main() {
    let outer = String::from("longest long string");
    {
        let inner = String::from("short");
        let result = longest(outer, inner);
        println!("result is: {}", result);
    }
}

// This fails to compile. Even though we can see that the longest string does in fact live long enough to be valid; we've told rust via the annotations that both arguments and the return value are tied to the same lifetime. 
// Since the inner string is dropped at the end of the block, rust sees that the lifetime is not satisfied and throws and error.
fn main() {
    let outer = String::from("longest long");
    let result;
    {
        let inner = String::from("short");
        result = longest(outer, inner);
    }
    println!("result is {}", result);
}
```

## Thinking in Terms of Lifetimes

- The exact way to specify lifetime parameters depends on what your function is doing. 
- Lifetimes are only relevant when borrowing and returning references.

```rust
fn longest<'a>(x: &'a str, y: &str) -> &'a str {
    x
}
```

When returning a reference from a function, thelifetime parameter of the return type needs to match the lifetime parameter of one of the arguments.

If the reference returned does not refer to one of the arguments, the only other possibility is that it refers to a value created within the function, which would be a dangling reference since the value will go out of scope when the function returns.

```rust
// Error "'result' does not live long enough".
fn longest<'a>(x: &str, y: &str) -> &'a str {
    let result = String::from("really long string");
    result.as_str()
}

// The solution in this case would be to move the data out of the function into the calling scope.
// The calling scope would therefore be responsible for cleanup.
fn longest(x: &str, y: &str) -> str {
    let result = String::from("really long string");
    result.as_str()
}
```

## Lifetime Annotations in Struct Definitions

- Structs can hold references; and references require lifetime annotations.

```rust
// The annotation ties the lifetime of the struct to that of the string reference.
struct Excerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Jack. Some years ago...");
    let first_sentence = novel.split(".")
        .next()
        .expect("Could not find any sentences.");
    let i = Excerpt {
        part: first_sentence
    };
}
```

## Generic Type Parameters, Trait Bounds, and Lifetimes Together

```rust
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T
) -> &'a str
    where T: Display
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

## Summary

- Generic type parameters means code can be applied to different types.
- Trait and trait bounds ensure that generic types will have the requisite behaviours.
- Relationships between the lifetimes of references ensure that there can be no dangling references.