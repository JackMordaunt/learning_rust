# How Functions Work

- `fn` keyword.
- `snake_case` is idiomatic naming style.

## Expressions & Statements

- Expressions return values; statements do not.
- Blocks `{ /* ... */ }` are expressions.

```rust
fn main() {
    let x = 5; // Statement.
    let y = {
        let x = 3;
        x + 1
    };
    // The block expressions returns the value 4.
    // The let statement binds y to the result of the block expression.
}
```

- Expressions do not use semicolons; semicolons delimit _statements_.

## Return Values

- The last expression of the function is returned implicitly.
- `return` keyword can be used to explicitly return.

```rust
// returns 6.
fn implicit() -> i32 {
    5 + 1
}

// returns 5.
fn explicit() -> i32 {
    return 5
}

// Produces error "expected i32, found ()".
fn statement() -> i32 {
    5 + 1;
}
```
