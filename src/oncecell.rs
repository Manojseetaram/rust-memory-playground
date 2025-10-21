use std::{cell::OnceCell, rc::Rc};

//Once cell meaning you can only achieve interior mutability
#[derive(Debug)]
struct Node {
    data: i32,
    pointers: OnceCell<Option<Rc<Node>>>,
}
pub fn oncecell() {
    let node2 = Rc::new(Node {
        data: 20,
        pointers: OnceCell::new(),
    });
    let node1 = Rc::new(Node {
        data: 30,
        pointers: OnceCell::new(),
    });
    node1
        .pointers
        .set(Some(Rc::clone(&node2)))
        .expect("Already set");
    println!("All Good Value Set");
    node1.pointers.set(None).expect("Already set");
}
