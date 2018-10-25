use std::vec::Vec;

pub struct Node {
    children: Vec<Node>
}

impl Node {
    pub fn new() -> Node {
        Node{children: Vec::new()}
    }

    pub fn add_child(&mut self, n: Node) {
        self.children.push(n);
    }


}