#![allow(dead_code)]
//! This illustrates how to use weak references for bi-directional relationships
//! while maintaining uni-directional ownership. This is important so that we 
//! don't leak memory with a reference cycle.

use std::rc::{Rc, Weak};
use std::cell::RefCell;

/// Node is a recursive data structure that we can use to build a tree where
/// the same node can be referenced in many places at once.
/// 
/// 1. A node should own it's children.
/// 2. A node should be owned by it's parent.
/// 3. A node should not own it's parent.
/// 
/// If a node is dropped it's children should be dropped with it and it's parent
/// should be unaffected.
#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>, // Rc<Node> would cause a ref cycle.
    children: RefCell<Vec<Rc<Node>>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exercise() {
        println!("\nTree test");
        // Initialise foo.
        // No parent.
        // No children.
        let foo = Node {
            value: 3,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        };
        // foo is a node that we want to be owned by multiple things.
        // So we wrap it in an Rc<T>.
        // leaf Rc now owns foo.
        let leaf = Rc::new(foo);
        // To access the parent we need to immutably borrow a weak reference and
        // upgrade it to a strong reference.
        println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
        // branch Rc owns a different Node.
        // branch.children owns an Rc pointing to foo by cloning leaf.
        // Now foo is owned by leaf and branch.children via an Rc.
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });
        // To set the leaf's parent to the new owner, branch, we grab a mutable
        // weak reference of the leaf's parent and assign a weak reference of 
        // branch to it.
        // Thus branch becomes the new parent of leaf via a weak reference.
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
        // Again, to get the parent value we need to immutably borrow the weak
        // reference and upgrade it to a strong reference.
        println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
        println!("\n");
    }
}