//! This illustrates how the `RefCell<T>` can by used to bypass the mutability 
//! restrictions imposed by a trait definition.
//! 
//! `RefCell<T>` is used on the mock object such that we can satisfy the trait
//! which includes an immutable reference to self, while still allowing it's
//! fields to be mutable at runtime.
//! 
//! The caveat with this kind of code is that the borrow checking rules are 
//! enforced at runtime; so if you're not careful, you can get yourself into 
//! trouble with runtime panics.
//! 
//! Abuse of `RefCell<T>` is likey a code smell. Consider using `RefCell<T>` as
//! a last resort.

#![allow(dead_code)]

// A Messenger can send the msg.
trait Messenger {
    // method "send" borrows an immutable reference to a Messenger and a string.
    // This immutable reference means that no implementation can mutate self
    // without resorting to RefCell<T>.
    fn send(&self, msg: &str);
}


// LimitTracker for lifetime 'a over type T for lifetime 'a bounded by the
// Messenger trait.
struct LimitTracker<'a, T>
    where T: 'a + Messenger,
{
    // messenger field borrows an immutable reference to T for lifetime 'a.
    messenger: &'a T, 
    value: usize,
    max: usize,
}

// given lifetime 'a and type T, implement LimitTracker for lifetime 'a and
// type T where T satisfies the Messenger trait.
impl<'a, T> LimitTracker<'a, T>
    where T: Messenger,
{
    // public associated function "new" borrows a reference to T and a quota.
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    // public method "set_value" borrows a mutable reference to a LimitTracker
    // and and takes ownership of a usize value.
    pub fn set_value(&mut self, value: usize) {
        self.value = value;
        let percentage_of_max = self.value as f64 / self.max as f64;
        match percentage_of_max {
           p if (p >= 0.75 && p < 0.9) => {
               self.messenger.send("Warning: You've used up over 75% of your quota!");
           },
           p if (p >= 0.9 && p < 1.0) => {
               self.messenger.send("Critical: You've used up over 90% of your quota!");
           },
           p if (p >= 1.0) => {
               self.messenger.send("Error: You've maxed out your quota!");
           },
           _ => {},
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    // MockMessenger is a struct which owns a sent field of type Vec<String>.
    struct MockMessenger {
        sent: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger { sent: RefCell::new(Vec::new()) }
        }
    }

    impl Messenger for MockMessenger {
        // send cannot take a mutable reference since that would not satisfy the
        // trait.
        // In order to mutate the field of an immutably borrowed struct, we
        // can use RefCell<T>.
        fn send(&self, msg: &str) {
            // Vec<T>.push(T) requires self to be mutable.
            //
            // If we weren't concerned about implementing a trait method we 
            // could change the signature to "send(&mut self, msg: &str)".
            //
            // Basically, when you're trying to implement a trait method that
            // immutably borrows self and your implementation requires 
            // mutability, you can wrap whatever needs to be mutated in a
            // RefCell<T> and enforce the borrow checking at runtime. 
            //
            // Using "borrow_mut" returns you a mutable reference.
            // If you try to borrow again (before the mutable reference is
            // dropped) you will get a runtime panic.
            self.sent.borrow_mut().push(String::from(msg));
        }
    }

    #[test]
    fn send_over_70_percent_message() {
        let messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&messenger, 100);
        limit_tracker.set_value(80);
        assert_eq!(messenger.sent.borrow().len(), 1);
    }
}