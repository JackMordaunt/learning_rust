# Variables and Immutability

- Immutable by default
- Opt in to mutability using `mut` keyword

## Shadowing

You can shadow a variable by re-assigning to the same name.

```rust 
fn main() {
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The vlaue of x is {}", x);
}
```

This allows us to "re-type" a variable - change the type without changing the name. This is useful for when you want to type cast; you can change the type of a variable without needing to come up with different names such as `spaces_str` and `spaces_num`.

```rust
let spaces = "    ";        // string type.
let spaces = spaces.len();  // number type.
```

