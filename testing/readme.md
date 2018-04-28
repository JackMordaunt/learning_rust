# Unit Testing

```rust
#[derive(Debug)]
pub struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.height > other.height && self.width > other.width
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { length: 8, width: 7 };
        let smaller = Rectangle { length: 5, width: 1 };
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle { length: 8, width: 7 };
        let smaller = Rectangle { length: 5, width: 1 };
        assert!(!smaller.can_hold(&larger));
    }
}
```

- The private test module is commonly included in the file containing the code for which you intend to test; this keep code and their tests in close proximity. 
- `#[cfg(test)]` annoations tells Rust to compile the following test module only during `cargo test`.
- `mod tests` declares a private test module for this crate.
- `#[test]` annotation tells cargo you intend to test the following function.
- `use super::*` imports the outer scope (the rest of the crate) into the inner scope so it's easy to use.
- `assert!` panics when the expression evaluates to `false`.
    - `assert!` accepts a string format as a second param for a nice message.
- A tests fails only when the thread running it panics. 

## Integration Tests

- Cargo looks for a `tests` directory next to `src`.
- Cargo compiles each test file in `tests` as an individual crate.
- Test modules in `tests` exercise your code as normal a consumer of it.
    - You need to import it as such; `extern crate mycrate`.
