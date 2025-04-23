use std::fmt::Debug;

struct LinkedListNode<T> {
    data: T,
    next: Option<Box<LinkedListNode<T>>>,
}

pub struct DynamicLinkedList<T> {
    head: Option<Box<LinkedListNode<T>>>,
    length: usize,
}

impl<T: PartialEq + Debug + Clone> DynamicLinkedList<T> {
    /// Creates a new, empty linked list.
    pub fn new() -> Self {
        Self { head: None, length: 0 }
    }

    pub fn insert(&mut self, data: T) {
        let mut new_node = Box::new(LinkedListNode { data, next: None });

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

        let mut new_node = Box::new(LinkedListNode { data, next: None });

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
        let mut current = self.head.as_mut();

    while let Some(node) = current {
        if node.data == old_data {
            node.data = new_data;
            return true;
        }
        current = node.next.as_mut();
    }
        false
    }

    pub fn update_element_at_index(&mut self, index: usize, data: T) -> bool {
        if index >= self.length {
            return false;
        }
    
        let mut current = self.head.as_mut().unwrap();
        for _ in 0..index {
            current = current.next.as_mut().unwrap();
        }
    
        current.data = data;
        true
    
    }

    pub fn find(&self, data: T) -> bool {
        let mut current = self.head.as_ref();

        while let Some(node) = current {
            if node.data == data {
                return true;
            }
            current = node.next.as_ref();
        }

        false
    }
}

// Static list
const MAX_SIZE: usize = 100;

#[derive(Clone, Debug)]
struct Node<T: Clone + PartialEq> {
    data: T,
    next: Option<usize>,
}

pub struct StaticLinkedList<T: Clone + PartialEq> {
    nodes: [Option<Node<T>>; MAX_SIZE],
    head: Option<usize>,
    free: Vec<usize>,
    length: usize,
}

impl<T: Clone + PartialEq> StaticLinkedList<T> {
    pub fn new() -> Self {
        Self {
            nodes: array_init::array_init(|_| None),
            head: None,
            free: (0..MAX_SIZE).rev().collect(),
            length: 0,
        }
    }

    fn allocate_node(&mut self, data: T) -> Option<usize> {
        self.free.pop().map(|index| {
            self.nodes[index] = Some(Node { data, next: None });
            index
        })
    }

    fn free_node(&mut self, index: usize) {
        self.nodes[index] = None;
        self.free.push(index);
    }

    pub fn insert(&mut self, data: T) {
        if let Some(new_index) = self.allocate_node(data) {
            if self.head.is_none() {
                self.head = Some(new_index);
            } else {
                let mut current = self.head.unwrap();
                while let Some(next_index) = self.nodes[current].as_ref().unwrap().next {
                    current = next_index;
                }
                self.nodes[current].as_mut().unwrap().next = Some(new_index);
            }
            self.length += 1;
        } 
    }

    pub fn get(&self, index: usize) -> Option<T> {
        if index >= self.length {
            return None;
        }

        let mut current = self.head?;
        for _ in 0..index {
            current = self.nodes[current].as_ref()?.next?;
        }
        Some(self.nodes[current].as_ref()?.data.clone())
    }

    pub fn delete_element(&mut self, data: T) -> bool {
        let mut current = self.head;
        let mut prev: Option<usize> = None;

        while let Some(idx) = current {
            let node = self.nodes[idx].as_ref().unwrap();
            if node.data == data {
                if let Some(prev_idx) = prev {
                    self.nodes[prev_idx].as_mut().unwrap().next = node.next;
                } else {
                    self.head = node.next;
                }
                self.free_node(idx);
                self.length -= 1;
                return true;
            }
            prev = current;
            current = node.next;
        }
        false
    }
 }  
mod test;