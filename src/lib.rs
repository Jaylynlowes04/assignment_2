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

        let mut current = &mut self.head;
        while let Some(node) = current {
            current = &mut node.next;
        }

        *current = Some(new_node);
        self.length += 1;
    }

    pub fn get(&self, index: usize) -> Option<T> {
        if index >= self.length {
            return None;
        }
    
        let mut current = self.head.as_ref();
        for _ in 0..index {
            current = current?.next.as_ref();
        }
    
        current.map(|node| node.data.clone())
    
    }

    pub fn insert_at_index(&mut self, index: usize, data: T) {
        if index > self.length {
            return;
        }

        let mut new_node = Box::new(Node { data, next: None });

        if index == 0 {
            new_node.next = self.head.take();
            self.head = Some(new_node);
        } else {
            let mut current = self.head.as_mut();
            for _ in 0..index - 1 {
                current = current.unwrap().next.as_mut();
            }
            new_node.next = current.as_mut().unwrap().next.take();
            current.as_mut().unwrap().next = Some(new_node);
        }

        self.length += 1;
    }

    pub fn delete_element(&mut self, data: T) -> bool {
    
        let mut current = &mut self.head;

        while let Some(mut boxed_node) = current.take() {
            if boxed_node.data == data {
                *current = boxed_node.next.take(); // Remove current node
                self.length -= 1;
                return true;
            } else {
                *current = Some(boxed_node); // Put it back unchanged
                current = &mut current.as_mut().unwrap().next;
            }
        }

        false
    }
    
    pub fn delete_at_index(&mut self, index: usize) -> bool {
        if index >= self.length {
            return false;
        }
    
        let mut current = &mut self.head;
    
        for _ in 0..index {
            current = &mut current.as_mut().unwrap().next;
        }
    
        let next = current.as_mut().unwrap().next.take();
        *current = next;
    
        self.length -= 1;
        true
    }
    
    pub fn update_element(&mut self, old_data: T, new_data: T) -> bool {
        todo!("not implemented");
    }
}
mod test;