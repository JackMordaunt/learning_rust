# Traits

> Traits are for defining shared behaviour across types.

- Similar to interfaces.
- Explicitly satisfied (not implicit like Go :c).
- Traits can have default implementations.
- You can specialise one or more of the default implementations.
- You can implement traits for any type, even the generic type T (blanket implementation).

```rust
struct Tweet {}
struct WeatherForcast {
    high_temp: f64,
    low_temp: f64,
    precipitation: f64,
}
struct Article {}

// traits define a set of methods akin to an interface definition.
trait Summarizable<T> {
    fn summary(&self) -> String;
}

// You can define default implementations also:
trait Summarizable {
    fn summary(&self) -> String {
        String::from("(Read more...)")
    }
}

// You must implement the trait for a type to associate the two:
impl Summarizable for WeatherForcast {
    fn summary(&self) -> String {
        format!("The high will be {}, and the low will be {}. The chance of precipitation is {}%.", self.high_temp, self.low_temp, self.precipitation)
    }
}

// You can take advantage of the default implementation like so:
impl Summarizable for Article {}
```

- Traits are used to contrain generic types by behaviour.
    - aka specififying _trait bounds_ on a generic type.
    - In order to use behaviours on a generic type you must specify what behaviours you are expecting; there is no duck typing.

```rust
// The angle bracket syntax applies the trait bounds to the generic type T.
fn notify<T: Summarizable>(item: T) {
    println!("Breaking news! {}", item.summary());
}

// An unconstrained version would look like this:
fn notify<T>(item:  T) {
    // We can't use the summary method because we haven't constrained the type. 
    prtinln!("{:?}", item);
}

// For multiple trait bounds we add them:
fn notify<T: Summarizable + Display>(item: T) {
    println!("Breaking news! {}", item.summary());
}

// A more readable syntax when we have a lot of trait bounds:
fn some_func<T, U>(t: T, u: U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{
    // ...
} 
```

## Conditional Method Implementations

```rust
use std::fmt::Display;

// A pair of objects of the same type.
struct Pair<T> {
    x: T,
    y: T,
}

// Methods implemented for a Pair containing any type T.
// Available on any Pair instance.
impl<T> Pair<T> {
    fn new(x: T, y: T) -> self {
        Self {
            x, 
            y,
        }
    }
}

// Methods implemented for a Pair containing any type that satisfies both the Display and PartialOrd traits.
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
```

You can also implement a trait for any type that implements a trait, essentially the above without involving the `Pair` struct.
This is known as a _blanket implementation_.

```rust

// Defines ToString on any type that has the trait Display.
impl<T: Display> ToString for T {
    // ...
}
```

