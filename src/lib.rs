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

    pub fn insert(&mut self, data: T) {
        let mut new_node = Box::new(Node { data, next: None });

        match self.head.as_mut() {
            None => self.head = Some(new_node),
            Some(mut current) => {
                while let Some(ref mut next) = current.next {
                    current = next;
                }
                current.next = Some(new_node);
            }
        }

        self.length += 1;
    }

    pub fn get(&self, index: usize) -> Option<T> {
        if index >= self.length {
            return None;
        }

        let mut current = self.head.as_ref();
        for _ in 0..index {
            current = current.unwrap().next.as_ref();
        }

        Some(current.unwrap().data.clone())
    
    }
}
mod test;