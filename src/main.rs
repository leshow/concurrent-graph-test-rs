use std::thread;
use std::sync::{Arc, RwLock};
use std::sync::mpsc::channel;

type NodeRef<T> = Arc<RwLock<_Node<T>>>;

struct _Node<T> {
    parent: Option<NodeRef<T>>,
    children: Vec<NodeRef<T>>,
    pub value: T,
}

pub struct Node<T>(NodeRef<T>);

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

struct Container<T> {
    nodes: Vec<Node<T>>,
}
impl<T> Container<T> {
    fn new(nodes: Vec<Node<T>>) -> Self {
        Container { nodes: nodes }
    }
}

fn main() {
    let parent = Node::new(1u8);
    let child = Node::new(2u8);

    parent.add_child(&child);
    child.set_parent(&parent);

    let container = Arc::new(Container::new(vec![parent, child]));

    let (tx, rx) = channel();
    let container_ref = container.clone();
    thread::spawn(move || {
        for n in 3..10u8 {
            let Node(new_node) = Node::new(n);
            tx.send(new_node).unwrap();
        }

    });

    loop {
        if let Ok(node_to_add) = rx.recv() {
            let unlock_node = node_to_add.read().expect("rwlock");
            let val = unlock_node.value;
            println!("got node {:?}", val);
        } else {
            break;
        }
    }
}
