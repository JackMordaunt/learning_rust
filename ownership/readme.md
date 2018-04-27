# Ownership

- This what makes Rust unique.
- Memory safety without need for garbage collector. 
- Borrowing, slices, and memory layout.

## What is Ownership?

- All programs have to manage the way they use a computer's memory.
    - Garbage collection (Go, Python, Javascript, Ruby).
    - Explicit allocation and frees (C, C++).
    - Ownership rules (Rust).

## Rules

1. Each value has a variable that's called its _owner_.
2. There can only be one owner at a time.
3. When the owner goes out of scope, the value will be dropped.

## Variable Scope

- A scope is the range within a program for an item is valid.

```rust
{ // s is not valid here, it hasn't yet been declared.
    let s = "hello"; // s is valid from this point forward.
    // Use s.
    println!("{}", s);
} // The scope is now over; s is no longer valid.
println!("{}", s); // error.
```

## Memory and Allocation

- `let s = "Hello";` is hardcoded; placed on the stack.
- `let s = String::from("Hello")` is an object; placed on the heap.
- Allocating memory is the same as other languages; automatic.
- De-allocation is the unique part.
    - When `s` goes out of scope the memory is automatically de-allocated by rust. 
    - Internally, rust inserts a statement which calls the `drop` function.

## Variable-Data interactions

### Move

```rust
let x = String::from("Hello");
let y = x;
```

In the above, `x` and `y` would point to the same underlying data since the `String` type only owns a pointer to the buffer.

Hypothetically, if both `x` and `y` are valid and get freed when they fall out of scope you would get a double free on the shared buffer.

Rust avoids this double free by invalidating `x`. It "moves" the data from `x` to `y`. This means that if you try to use `x` after moving it rust will slap you on the wrist with a "use of moved value" error.

> An implication of this is that you can guarantee that rust wont deep copy any structure without you knowing about it. Deep copies can be very expensive.  

### Clone

If we do want to copy the entire object, aka a deep copy, we can use a common method called `clone`.

```rust
let x = String::from("Hello");
let y = x.clone();

println!("x = {}, y = {}", x, y);
```

`clone` allows you visually, explicitly, see that all the data is being copied; which can often be a very expensive operation.

Since primitives are small, they simply get shallow cloned automatically. Hence we don't _need_ to call `clone` on, say, an `i32`.

If a type has the `Copy` trait, an older variable is still usable after assignment, since the data gets cloned. If the type or any of its parts satisfy the `Drop` trait rust wont let you satisfy the `Copy` trait.

## Ownership and Functions

Similar to assigning to a variable, passing a variable to a function will move or copy.

```rust 
fn main() {
    let s = String::from("Hello");
    takes_ownership(s);
    let x = 5;
    makes_copy(x);
} // x goes out of scope, then s. Since s was moved nothing happens. x is freed.

fn takes_ownership(a_string: String) {
    println!("{}", a_string);
} // a_string goes out of scope and `drop` is called; the memory freed.

fn makes_copy(an_int: i32) {
    println!("{}", an_int);
} // an_int goes out of scope; freed.
```

Trying to use `s` after passing it to `takes_ownership`, rust would throw an error.

## Return Values and Scope

Returning values can also transfer ownership.

```rust
fn main() {
    let s1 = gives_ownership(); // moves return value into s1.
    let s2 = String::from("Hello"); // s2 comes into scope.
    let s3 = takes_and_gives_back(s2); // s2 is moved into the function and the function's return value is moved into s3.
} // s3 goes out of scope and is dropped. s2 goes out of scope but was moved so nothing happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {
    let a_string = String::from("Hello"); // a_string comes into scope.
    a_string // a_string iis returned and moves out to the calling function.
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string // a_string is returned and moves out to the calling function.
}
```

- Data belonging to a variable that goes out of scope is dropped.
- To keep data alive you must move it to a variable that is in-scope.

## References and Borrowing

- To use a value without taking ownership of it we borrow a reference.

```rust
fn main() {
    let s1 = String::from("Hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

- Because the function only owns a reference, the real value won't be dropped when the function exits. 
- Borrowed references can not be mutated.

## Mutable References

We can mutate a borrowed reference with the `mut` keyword.

```rust
fn main() {
    let mut s = String::from("Hello");
    change(&mut s);
}

fn change(a_string: &mut String) {
    a_string.push_str(", world");
}
```

- Mutable references come with a caveat: there can only exist _one_ mutable reference to a value.

```rust
let mut s = String::from("Hello");
let r1 = &mut s;
let r2 = &mut s;
```

- Rust will throw you an error: "cannot borrow `s` as mutable more than once at a time".
- This caveat is the feature that prevents data races.

## The Rules of References


1. At any given time, you can have either but not both of:
    - One mutable reference.
    - Any number of immutable references.
2. References must always be valid.