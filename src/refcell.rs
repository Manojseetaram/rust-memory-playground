use std::{cell::RefCell, rc::Rc};

//Rec Cell in Rust
#[derive(Debug)]

struct Node {
    data: i32,
    points: Option<Rc<RefCell<Node>>>,
}

pub fn refoperation() {
    let node3 = Rc::new(RefCell::new(Node {
        data: 10,
        points: None,
    }));
    let node2 = Rc::new(RefCell::new(Node {
        data: 20,
        points: Some(Rc::clone(&node3)),
    }));
    let node1 = Rc::new(RefCell::new(Node {
        data: 30,
        points: Some(Rc::clone(&node3)),
    }));
    println!("Before Node1 : {:?}", node1.borrow());
    println!("Before Node3 : {:?}", node3.borrow());

    node1.borrow_mut().points = None;
    node3.borrow_mut().points = Some(Rc::clone(&node1));
    println!("After Node1 : {:?}", node1.borrow());
    println!("After Node3 : {:?}", node3.borrow());
}

