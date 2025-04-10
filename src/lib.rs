use std::fmt::Debug;

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

pub struct DynamicLinkedList<T> {
    head: Option<Box<Node<T>>>,
    length: usize,
}

impl<T: PartialEq + Debug + Clone> DynamicLinkedList<T> {
    /// Creates a new, empty linked list.
    pub fn new() -> Self {
        Self { head: None, length: 0 }
    }
}
mod test;