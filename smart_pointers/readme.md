# Smart Pointers

- Smart pointers act as a reference type with additonal capabilities, eg owning their data.
- Very common examples:
  - `String`
  - `Vec<T>`
- Smart pointers are implemented using structs.
  - `Deref` and `Drop` traits allow a type to be a smart pointer.
  - `Deref` allows us to use smart pointers where we use references.
  - `Drop` allows us to customize the code that is run when the smart pointer goes out of scope.
- Well known: 
  - `Box<T>`: for allocating values on the heap.
  - `Rc<T>`: a reference counted type that enables multiple ownership.
  - `Ref<T>` and `RefMut<T>`, access through `RefCell<T>`, a type that enforces borrowing rules at runtime. 

## `Box<T>`

> `Box<T>` points to data on the heap and has a known size.

- No performance overhead outside of heap access vs stack access.
- Must use when:
  - A type's size cannot be known at compile time.
  - You have a large amount of data to transfer ownership and you don't want to copy it when you do so.
  - You want to own a value and only care that it's a type that implements a trait rather than knowing it's concrete type (basically your traditional oop interface).

### Using `Box<T>` to Store Data on the Heap

```rust
// Store a primitive on the heap by boxing it.
fn main() {
    let b = Box::new(5);
    println!("b = {}", b);
}
```

- When the box goes out of scope both it and the data it owns is dropped.

### Boxes Enable Recursive Types

At compile time, Rust needs to know how much space a type takes up.

An example of a type that doesn't have a known size at compile time is a _recursive type_, where a value can have part of itself another value of the same type.

Error demonstrated for recursive types:

```rust
enum List {
    Cons(i32, List),
    Nil,
}

// error: recursive type `List` has infinite size.
fn main() {
    let list = Cons(1, Cons(2, Cons(3, Nil)));
}
```

Use indirection (in this case with a Box) to solve:

```rust 
enum List {
    Cons(i32, Box<List>),
    Nil,
}

// Woohoo no error!
fn main() {
    let list = Cons(1,
    	Box::new(Cons(2,
    		Box::new(Cons, 3)
    			Box::new(Nil))));
}
```

## Treating Smart Poitners Like Regular References with the `Deref` Trait

- Implementing the `Deref` trait allows us to customize the behaviour of the _dereference operator_, `*`. 

```rust
// Tuple struct just holds the value.
struct MyBox<T>(T); 

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> ::std::ops::Deref for MyBox<T> {
	type Target = T;
    fn deref(&self) -> &T {
		&self.0
    }
}

fn main() {
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
    // The smart pointer gets deref coerced into a &str.
    // &MyBox<String> => &String => &str.
}

fn hello(name: &str) {
    println!("Hello, {}", name);
}
```

### Deref and Mutability

- From `&T` to `&U` when `T: Deref<Target=U>`
- From `&mut T` to `&mut U` when `T: DerefMut<Target=U>`
- From `&mut T` to `&U` when `T: Deref<Target=U>`

## The `Drop` Trait Runs Code on Cleanup

- Cusomize what happens when an object goes out of scope.
- We can use it to release resources such as files or network connections.
- `Drop` is almost always implemented on smart pointers.

```rust
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("Jack"),
    };
    let d = CustomSmartPointer {
        data: String::from("Mordaunt"),
    };
    println!("CustomSmartPointers created");
}
```

## `Rc<T>`: Share Data Immutably

> `Rc<T>` is a reference counting smart pointer.

- Allow multiple immutable borrows of one piece of heap data at the same time.
  - Specifically: useful when you want to allocate some data on the heap for the rest of the program to read, and can't determine at compile time which part will finish using the data last.
- Only for single threaded contexts.

```rust
use std::rc::Rc;

enum List {
    Cons(Rc<List>),
    Nil,
}

// a, b and c all share data through Rc<T>.
// When you clone an Rc it bumps the reference counter.
// When an Rc is dropped it decrements the reference counter.
fn main() {
    let a = Rc::new(Cons(5,
    	Rc::new(Cons(10,
    		Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a));
    {
        let c = Cons(4, Rc::clone(&a));
    }
}
```

- You could call `Rc.clone()`, but the idiomatic way is `Rc::clone(..)` with the associated function.

## `RefCell<T>`: Share Data Immutably

_Interior mutability_ is a design pattern in Rust that allows you to mutate data even when there are immutable references to that data. 

This uses `unsafe` code inside a data structure to bend the rules.

> We can use types that use interior mutability when we can ensure that the borrowing rules will be followed at runtime, even though the compiler can't guarantee that.  The `unsafe` code is then wrapped in a safe API, and the outer type is still immutable.

### Enforcing Borrowing Rules at Runtime

- Since the Rust compiler is conservative there are some cases which validly follow the borrowing rules yet are rejected by the compiler because it can't understand.
- `RefCell<T>` allows us to implement these types of programs by adhering to the borrowing rules at runtime.
- If the code is actually incorrect and breaks the rules you will get a runtime panic. 
- Like `Rc<T>`, `RefCell<T>` is only for single threaded contexts.

#### Use Case: Mock Objects

**The problem:**

1. You want to implement a trait with a mock object.

2. The trait defines a method that _immutably_ borrows `self`

   - `fn foo(&self) { /* ... */ }`

3. You want to mutate the state of self inside this method despite no access to a mutable reference, while still conforming to the trait signature.

   - ```rust
     fn foo(&self) {
         // Eg: attempt to mutate a vector.
         self.list.push("can't do this, we don't have mutable reference which push requires");
     }
     ```

**The solution:**

1. Wrap the fields you desire to mutate in your mock with a `RefCell<T>` smart pointer.

2. Use `borrow()` and `borrow_mut()` to get references to the field.

   - ```rust
     fn foo(&self) {
     	self.list.borrow_mut().push("Yay this works! Provided list is not borrowed anywhere else at runtime")
     }
     ```

3. Breaking the borrow checker rules will give you a _runtime_ panic. 

   1. You can borrow only one mutable reference.
   2. You can borrow any number immutable references provided there are no borrowed mutable references.

   - ```rust
     // This code will compile despite the borrow checking error.
     // You will get a nice panic at runtime while attempting to
     // assign to variable d, since there are still active
     // immutable references.
     fn foo(&self) {
         {
             let a = self.list.borrow_mut();
         } // Dropped the mutable reference; so we're okay.
         let b = self.list.borrow();
         let c = self.list.borrow();
     	let d = self.list.borrow_mut(); // Panic!
     }
     ```


## Leaking Memory with Reference Cycles

- Using `Rc<T>` and `RefCell<T>` where the reference count never reaches zero.
  - Be careful when you see nested types which makes use of:
    - Interior mutability.
    - Reference counting.
- The memory leaks are still memory safe.
- Logic bug; use tests to identify. Rust can't help you at runtime.
- Can be solved by making the ownership unideriectional.
  - Owner owns the child (`Rc<T>`).
  - Child borrows a reference to the parent (`Weak<T>`).
  - When the owner goes out of scope, so too does the child. 

### Reference Counting

- Strong references are how we share _ownership_ of an `Rc<T>` instance.
  - `Rc::clone()` produces a  strong reference.
  - `Rc<T>` is not dropped until the strong reference count hits zero, until all the owners are dead.
    - Irrespective of the weak reference count.
- Weak references **don't** express an ownership relationship.
  - You want to use a thing if it's available, but you don't _need_ to use it.
    - It's like making a coffee. You'd like a dash of milk, but you can handle black. So you check the fridge for milk. If there is milk, you add a dash to your coffee; otherwise you enjoy it without milk. In any case you don't have a hard dependency on the milk. You've relegated the milk use to runtime. 
    - Whereas consider ordering a flat white. By definition of what it is, it requires the existence of milk. If there's no milk then you can't reasonably ask for, or expect, a flat white; since it can't actually be constructed. The milk is therefore hard dependency that gets resolved at compile time.
  - `Rc::downgrade()` produces a weak reference.
  - `Rc::upgrade()` attempts to grab a strong reference at runtime; `<Option<Rc<T>>`.
    - If the value referenced has been dropped; option will match `None`.
    - You can't directly use the value reference by `Weak<T>`, you must upgrade it.
      - Analogous to opening the fridge to check for milk.
      - Until you open the fridge you don't know if there's milk inside.
      - Only once you open the fridge and grab the milk can you now use it.
      - You have to be able to handle the case that there is no milk in the fridge.
    - This means you're attempting to grab a reference at _runtime_, where that reference may not be valid when you attempt to use it, since compiler can't guarantee it's validity at compile time.
    - `Weak<T>` is basically a handle to a strong reference `Rc<T>`, that allows you to attempt to resolve a strong reference at runtme. 
    - This lets you reference data without being beholden to it's lifetime; but you must handle the case that the data has been dropped.
    - You're saying "If it's there; great I'll use it. Otherwise I'll do without just fine."

## Summary

**Smart pointers:**

- Push concerns from compile time to run time where there is more information.
  - `Box<Error>` for being polymorphic accross many implementations of trait `Error`.
    - You don't statically know the size of the implementor of the trait at compile time.
      - Polymorphic use of a trait, such as a `Vec<Error>` which might have 10 different concrete types that satisfy the `Error` trait.
    - Implementing recursive types.
  - `RefCell<T>` for enforcing borrow checking rules at runtime.
    - Useful for mutating data within immutable trait methods. 
    - Eg mock objects.
    - Opens the door for memory leaks via reference cycles at runtime. 
    - A reference cycle will also overflow the stack if you attempt to dereference it.
  - `Rc<T>` for when you don't statically know when the last consumer of a piece of data will finish.
    - This means the scope wherein `Rc<T>` is created doesn't have to exist for the entire length of the use of `Rc<T>`.
    - The borrowers are no longer beholden to the lifetime of the owner of the data, the owner has volunteered to be beholden to collective lifetime of the borrowers.