# Iterators

- Commond methods:

  - `iter`: borrows references from the collection.
  - `into_iter`: owns the values from the collection.
  - `iter_mut`: borrows mutable references from the collection.

- Lazily evaluated.

  - `let v = vec![1, 2, 3].iter();` 

- Abstracts _how_ to traverse an object.

- Can be used with `for in` loop.

- `Iterator` trait defined in standard library.

- Uses assosciated type `Item` so that implementers remain in control of the yielded type.

  - ```rust
    trait Iterator {
        type Item;
        fn next(&mut self) -> Option<Self::Item>;
    }
    ```

- Iterators need to be declared as mutable if it relies on changing state over the course of the iteration (such as updating an index to keep track of the curren item).

  - ```rust
    let mut v_iter = vec![1, 2, 3].iter();
    ```

  - We don't need to do this in a `for` loop because the loop takes ownership of the iterator behind the scenes.

  - The items yielded from a `Vec` are immutable references. To get mutable references you can call `into_iter` instead of `iter`.

## Consuming Iterators

- Methods on the standard `Iterator` trait which call `next` _consume_ the items in the iterator. 

- Once an iterator has been consumed we can't use it any longer.

  - ```rust
    let iterator = vec![1, 2, 3].iter();
    let total: i32 = iterator.sum();
    for _ in iterator { /* ... */ } // Compiler error! 
    ```

  - `sum` takes ownership of the iterator; rendering it unusable by anyone else.

## Iterator Adaptors

- We can transmute iterators into different kinds of iterators by using iterator adaptors.
-  We can chain adaptors to perform complex actions in a readable way.
- Because iterators are lazy we have to call one of the consuming adaptors to get the results.
  - The `collect` method is often used to this effect.
  - `collect` consumes the iterator and collect the resulting values into a collection data type.
  - `collect` is bounded by the trait `::std::iter::FromIterator`.

## Closures

- Anonymous functions which "close over" their environment.
- `|T| { /* ... */ }`

```rust
let n = 2;
let divisible_by_n: Vec<i32> = vec![1, 2, 3, 4, 5, 6].into_iter()
	.filter(|v| v % n == 0)
	.collect();
```

- `into_iter` creates an iterator that takes ownership of the vector.
- `filter` adapts the iterator to a new iterator with the filter applied.
- The closure supplied to filter closes over a variable located in the outer scope.
- `collect` put the items in the iterator into `Vec<i32>`.