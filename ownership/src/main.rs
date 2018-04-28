fn main() {
    as_str();
    borrowed();
    
    // Ownership is not transfered, v is only borrowed.
    let v = String::from("value");
    longest(&v, &v);
    println!("value: {}", v);

    // v is moved (ownership is transfered to the function).
    let v = String::from("value");
    finders_keepers(v); // moved.
    // println!("value: {}", v); // v is now invalid.

    // v is mutably borrowed (ownership is not transfered to the function).
    let mut v = String::from("value");
    borrowers_return(&mut v); // mutable borrow.
    longest(&v, &v); // immutable borrow.
    println!("got it back: {}", v); // v is still valid.
    
}

fn as_str() {
    let outer = String::from("hello, ");
    {
        let inner = String::from("world!");
        let result = longest(outer.as_str(), inner.as_str());
        println!("result: {}", result);
    }
}

// How does &String satisfy &str? Rust coerces &String into &str through the Deref trait.
// String.deref() returns a &str; so passing &String to &str works as expected.
fn borrowed() {
    let outer = String::from("hello, ");
    {
        let inner = String::from("world!");
        let result = longest(&outer, &inner);
        println!("result: {}", result);
    }
}

fn longest<'a>(left: &'a str, right: &'a str) -> &'a str {
    if left.len() > right.len() {
        left
    } else {
        right
    }
}

// (mut v: T) declares a new variable that takes ownership.
// If you take ownership you can redeclare an object as mutable even if it 
// was previously immutable.
fn finders_keepers(mut v: String) {
    v.push_str(", mine!");
    println!("{}", v);
}

// (v: &mut T) decalares a new reference that borrows mutably.
// Because we aren't taking ownership we can't redeclare mutability; the argument
// must already be declared mutable by it's owner.
fn borrowers_return(v: &mut String) {
    v.push_str(", have it back!");
    println!("{}", v);
}