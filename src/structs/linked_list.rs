use std::fmt::Debug;
#[derive(Debug)]
pub struct LinkedList<T> {
    pub head: Option<Box<LinkedNode<T>>>,
    length: usize,
}

#[derive(Debug)]
pub struct LinkedNode<T> {
    pub data: T,
    pub next: Option<Box<LinkedNode<T>>>,
}

impl<T> LinkedNode<T> {
    fn new(data: T, next: Option<Box<LinkedNode<T>>>) -> LinkedNode<T> {
        LinkedNode { data, next }
    }
}

impl<T: std::fmt::Debug> LinkedList<T> {
    pub fn new() -> LinkedList<T> {
        LinkedList {
            head: None,
            length: 0,
        }
    }

    pub fn print(&self) {
        let mut current = &self.head;
        while let Some(node) = current {
            println!("{:?}", node.data);
            current = &node.next;
        }
    }

    pub fn append(&mut self, data: T) {
        let current = &mut self.head;
        let new_node = Box::new(LinkedNode::new(data, current.take()));
        *current = Some(new_node);
        self.length += 1;
    }

    pub fn count(&self) -> usize {
        self.length
    }
}
