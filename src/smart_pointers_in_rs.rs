use std::cell::RefCell;
use std::ops::Deref;
use std::rc::{Rc, Weak};

pub fn smart_pointers() {
    // Boxes
    boxes_in_rs();

    // # Creating a Tree Data Structure: a Node with Child Nodes
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });
    println!("leaf parent =",);
    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
    println!("leaf parent = ",);
}

// Boxes

// # Using Box<T> to Point to Data on the Heap
// The most straightforward smart pointer is a box, whose type is written Box<T>
// . Boxes allow you to store data on the heap rather than the stack. What remains on the stack is the pointer to the heap data.

// - Situations where Boxes are used most
// - When you have a type whose size can’t be known at compile time and you want to use a value of
//that type in a context that requires an exact size.

// - When you have a large amount of data and you want to transfer ownership but ensure
// the data won’t be copied when you do so.

//  - When you want to own a value and you care only that it’s a type that implements a particular
// trait rather than being of a specific type.

//Using Box<T> to Get a Recursive Type with a Known Size

enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn boxes_in_rs() {
    // #Using a Box<T> to Store Data on the Heap

    let b = Box::new(5);
    println!("b = {}", b);

    // Enabling Recursive Types with Boxes
    // let list = Cons(1, Cons(2, Cons(3, Nil)));

    //Using Box<T> to Get a Recursive Type with a Known Size
    let list = List::Cons(
        1,
        Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))),
    );
}

// #Defining Our Own Smart Pointer

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// #Treating a Type Like a Reference by Implementing the Deref Trait

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

//Creating a Tree Data Structure: a Node with Child Nodes
#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}
