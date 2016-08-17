use std::thread;
use std::sync::{Arc, RwLock};
use std::sync::mpsc::channel;

/// type alias of a node reference
type NodeRef<T> = Arc<RwLock<_Node<T>>>;

/// private struct wrapping node data
#[derive(Debug)]
struct _Node<T> {
    parent: Option<NodeRef<T>>,
    children: Vec<NodeRef<T>>,
    pub value: T,
}

/// our public interface for accessing a node, using newtype
pub struct Node<T>(NodeRef<T>);

/// public node fns
impl<T> Node<T> {
    pub fn new(value_: T) -> Self {
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

    parent.add_child(&child);
    child.set_parent(&parent);

    // sender/receiver from channel
    let (tx, rx) = channel();
    // make worker thread to create nodes
    thread::spawn(move || {
        for n in 3..10u8 {
            let new_node = Node::new(n);
            tx.send(new_node).unwrap();
        }
    });

    // wait to get all the nodes
    while let Ok(node_to_add) = rx.recv() {
        parent.add_child(&node_to_add);
        node_to_add.set_parent(&parent);
        let unlock_node = node_to_add.0.read().expect("rwlock");
        let val = unlock_node.value;
        println!("got node {:?}", val);
    }

    // get read lock on parent and iterate through
    let ref children = parent.0.read().expect("parent read lock").children;
    for child in children {
        let val = child.read().expect("read lock child value").value;
        println!("got child val {:?}", val);
    }
    // try a filter on children vec
    let count = children.iter()
        .filter(|&node| {
            let val = node.read().expect("filter read lock").value;
            val > 5
        })
        .collect::<Vec<_>>();
    println!("{:?} nodes bigger than 5", count.len());
}
