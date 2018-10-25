use std::vec::Vec;

pub struct Node {
    children: Vec<Node>
}

impl Node {
    pub fn new() -> Node {
        Node{children: Vec::new()}
    }

    pub fn add_child(&mut self, n: Node) -> &mut Node {
        self.children.push(n);
        self.children.last_mut().unwrap()
    }


}