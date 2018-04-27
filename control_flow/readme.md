# Control Flow

- `if` will not try to convert numbers or empty objects to booleans like javascript and ruby; you need to be explicit.

## `if let`

- `if` is an expression; which means you can assign the result with `let`.

```rust
fn main() {
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };
    // number == 5.
}
```

- The above means branches in the `if` statement need to return equal types.

```rust 
// Compiler error: "expected type `{integer}`, found type `&str`.
fn main() {
    let condition = true;
    let number = if condition {
        5
    } else {
        "six"
    };
}
```

## Looping

- `loop`
    - `break`
- `while`
    - A combination of `loop` `if` `else` to loop for a given condition.
- `for`
    - Looping over items in an iterable collection.

```rust
fn main() {
    for number in (1..4).rev() {
        println!("{}", number);
    }
    println!("Done.");
}
```

