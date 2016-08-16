use std::thread::Thread;
use std::sync::{Arc, RwLock};


pub type Children<T> = Vec<NodeRef<T>>;
pub struct NodeRef<T>(Arc<RwLock<Node<T>>>);
// pub struct Children<T>(Arc<RwLock<Vec<T>>>);
//
// impl<T> Children<T> {
//     fn new() -> Children<T> {
//         Children(Arc::new(RwLock::new(Vec::new())))
//     }
// }

pub struct Node<T> {
    parent: Option<NodeRef<T>>,
    children: Arc<RwLock<Children<T>>>,
    value: T,
}

impl<T> Node<T> {
    pub fn new(parent_: Option<NodeRef<T>>, value_: T) -> Node<T> {
        Node::<T> {
            parent: parent_,
            children: Arc::new(RwLock::new(Vec::new())),
            value: value_,
        }
    }
    pub fn add_child(&self, child: Node<T>) {
        self.children
            .write()
            .expect("Failed to get write lock on node")
            .push(child);
    }
}

fn main() {
    let parent = Node::new(Option::None, 1u8);
    let child = Node::new(parent.parent, 2u8);
    parent.add_child(child);
}
