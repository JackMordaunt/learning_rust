# Lifetimes

Notes from the New Rustacean podacast.

- Lifetime specifier `'a`.
- Only explicitly required for references.
- Rust's way of letting us reason about and specify how long a piece of data needs to be alive.
- Most of the time the lifetime specifiers are elided because the borrow checker can figure it out without our help. When it can't we'll get an error telling us such. 
- "a" is just a convention. 
  - When you have more than one you cycle through the alphabet `'b, 'c, 'd`.

```rust
// The reference contained in the Option must live at least as long as the 'out' parameter.
fn foo<'a, 'b>(in: &mut'a T, out: &'b U) Option<'b String> {}

// The references contained in the fields must live at least as long as the instance.
struct Foo<'a> {
    name: &'a str,
    password &'a str,
}
```

- For data on the heap the container type has it's lifetime tracked.
  - Smart pointers like `Box` fall into this category. 
  - Once the container goes out  of scope the memory is deallocated from the heap along with it.