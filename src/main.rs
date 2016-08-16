use std::thread::Thread;
use std::sync::{Arc, RwLock};


type NodeRef<T> = Arc<RwLock<_Node<T>>>;

struct _Node<T> {
    // parent: Option<NodeRef<T>>,
    children: Vec<NodeRef<T>>,
    value: T,
}

pub struct Node<T>(NodeRef<T>);

impl<T> Node<T> {
    pub fn new(parent_: Option<Node<T>>, value_: T) -> Node<T> {
        let node = _Node {
            // parent: parent_,
            children: Vec::new(),
            value: value_,
        };
        Node(Arc::new(RwLock::new(node)))
    }
    pub fn add_child(&self, child: Node<T>) {
        self.0
            .write()
            .expect("Failed to get write lock on node")
            .children
            .push(child.0.clone())
    }
}

fn main() {
    let parent = Node::new(Option::None, 1u8);
    // let child = Node::new(Option::Some(parent), 2u8);
    // parent.add_child(NodeRef(child.clone()));
}
