use std::thread::Thread;
use std::sync::{Arc, RwLock};


type NodeRef<T> = Arc<RwLock<_Node<T>>>;

struct _Node<T> {
    parent: Option<NodeRef<T>>,
    children: Vec<NodeRef<T>>,
    value: T,
}

pub struct Node<T>(NodeRef<T>);

impl<T> Node<T> {
    pub fn new(value_: T) -> Node<T> {
        let node = _Node {
            parent: Option::None,
            children: Vec::new(),
            value: value_,
        };
        Node(Arc::new(RwLock::new(node)))
    }
    pub fn add_child(&self, child: &Node<T>) {
        self.0
            .write()
            .expect("Failed to get write lock on node")
            .children
            .push(child.0.clone())
    }
    pub fn set_parent(&self, parent: &Node<T>) {
        self.0
            .write()
            .expect("Failed to get write lock on node")
            .parent = Option::Some(parent.0.clone());

    }
}

fn main() {
    let parent = Node::new(1u8);
    let child = Node::new(2u8);
    child.set_parent(&parent);
    parent.add_child(&child);


}
