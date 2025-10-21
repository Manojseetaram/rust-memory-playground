//Rc in Rust
use std::rc::Rc;

#[derive(Debug)]

struct Node {
    data: i32,
    points: Option<Rc<Node>>,
}
pub fn smartpointers_opaeration() {
    let node3 = Rc::new(Node {
        data: 10,
        points: None,
    });
    dbg!(Rc::strong_count(&node3));
    let node2 = Node {
        data: 20,
        points: Some(Rc::clone(&node3)),
    };

    dbg!(Rc::strong_count(&node3));
    drop(node2);
    let node1 = Node {
        data: 30,
        points: Some(Rc::clone(&node3)),
    };
    dbg!(Rc::strong_count(&node3));
}
