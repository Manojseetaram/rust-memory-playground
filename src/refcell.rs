use std::cell::Cell;
use std::rc::Rc;

#[derive(Debug)]
struct Node {
    data: Cell<i32>,
    points: Option<Rc<Node>>,
}
pub fn refCell_operation() {
    let node3 = Rc::new(Node {
        data: Cell::new(10),
        points: None,
    });
    let node2 = Node {
        data: Cell::new(20),
        points: Some(Rc::clone(&node3)),
    };
    let node1 = Node {
        data: Cell::new(30),
        points: Some(Rc::clone(&node3)),
    };

    node1.data.set(888);
    if let Some(ref node) = node1.points {
        dbg!(node.data.set(10000))
    }
    dbg!(node2);
    dbg!(node1);
}
