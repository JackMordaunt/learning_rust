# Data Types

- Statically typed.
- Usually types are inferred.
- Cases where a variable may be more than one type you need to explicitly annotate.
    - `let guess: u32 = "42".parse().expect("not a number!");`

## Scalar Types

### Integers

- All number literales except the byte literal allow a type suffix, such as `57u8` and a visual separator, such as `1_000`.

| Number Literals  | Example       |
| ---------------- | ------------- |
| Decimal          | `98_222`      |
| Hex              | `0xff`        |
| Octal            | `0o77`        |
| Binary           | `0b1111_0000` |
| Byte (`u8` only) | `b'A'`        |

## The Character Type

- Rust's `char` is a Unicode Scalar Value.
- Use single quotes to denote a `char`.

```rust
fn main() {
    let c = 'z';
    let z = 'ž';
    let heart_eyed_cat = '😻';
}
```

## Compound Types

- Tuple
- Array

## Tuple

- Group together values of various types.
- Comma separated list inside parens.

```rust
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}
```

- You can use pattern matching to grab the individual elements (destructuring).

```rust
fn main() {
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("x: {}, y: {}, z: {}", x, y, z);
}
```

- You can also use period index to access a single value.

```rust
fn main() {
    let x: (i32, f64, u8) = (500, 6.3, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
}
```

## Arrays

- Group together elements of the same type.
- Comma separated list inside square parens.

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];
}
```

- Useful when you want to allocate on the stack.
- Most apt when you have a fixed array; data that doesn't change over time, such as months of the year.

```rust
let months = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Oct", "Sep", "Nov", "Dec"];
```

- Rust panics at _runtime_ when you attempt to index out of bounds.
