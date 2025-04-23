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

/// # Description
/// Creates a new, empty static linked list.
/// 
/// # Returns
/// A new list with capacity for `MAX_SIZE` elements.
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

    /// # Description
    /// Inserts a new element at the end of the list.
    /// 
    /// # Parameters
    /// - data: the element to insert.
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

    /// # Description
    /// Retrieves an element at a specific index.
    /// 
    /// # Parameterss
    /// - index: the position to access.
    /// 
    /// # Returns 
    /// The element at the specified index
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

    /// # Description
    /// Deletes the first occurrence of an element.
    /// 
    /// # Parameters
    /// - data: the element to delete.
    /// 
    /// # Returns
    /// True if the element was found and deleted, false otherwise.
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

    /// # Description
    /// Inserts a new element at a specific index.
    /// 
    /// # Parameters
    /// - index: the position to insert the element.
    /// - data: the element to insert.
    pub fn insert_at_index(&mut self, index: usize, data: T) {
        if index > self.length {
            return;
        }

        if let Some(new_index) = self.allocate_node(data) {
            if index == 0 {
                self.nodes[new_index].as_mut().unwrap().next = self.head;
                self.head = Some(new_index);
            } else {
                let mut current = self.head.unwrap();
                for _ in 0..index - 1 {
                    current = self.nodes[current].as_ref().unwrap().next.unwrap();
                }
                self.nodes[new_index].as_mut().unwrap().next =
                    self.nodes[current].as_ref().unwrap().next;
                self.nodes[current].as_mut().unwrap().next = Some(new_index);
            }
            self.length += 1;
        } 
    }

    /// # Description
    /// Deletes an element at a specific index.
    ///     
    /// # Parameters
    /// - index: the position to delete the element.
    /// 
    /// # Returns
    /// True if the element was found and deleted, false otherwise.
    pub fn delete_at_index(&mut self, index: usize) -> bool {
        if index >= self.length {
            return false;
        }

        let mut current = self.head.unwrap();
        if index == 0 {
            self.head = self.nodes[current].as_ref().unwrap().next;
            self.free_node(current);
        } else {
            let mut prev = current;
            for _ in 0..index - 1 {
                prev = self.nodes[prev].as_ref().unwrap().next.unwrap();
            }
            current = self.nodes[prev].as_ref().unwrap().next.unwrap();
            self.nodes[prev].as_mut().unwrap().next = self.nodes[current].as_ref().unwrap().next;
            self.free_node(current);
        }

        self.length -= 1;
        true
    }

    /// # Description
    /// Updates the first occurrence of an element.
    /// 
    /// # Parameters
    /// - old_data: the element to update.
    /// - new_data: the new value.
    /// 
    /// # Returns
    /// True if the element was found and updated, false otherwise.
    pub fn update_element(&mut self, old_data: T, new_data: T) -> bool {
        let mut current = self.head;

        while let Some(index) = current {
            let node = self.nodes[index].as_mut().unwrap();
            if node.data == old_data {
                node.data = new_data;
                return true;
            }
            current = node.next;
        }

        false
    }

    /// # Description
    /// Updates an element at a specific index.
    /// 
    /// # Parameters
    /// - index: the position to update the element.
    /// - data: the new value.
    /// 
    /// # Returns
    /// True if the element was found and updated, false otherwise.
    pub fn update_element_at_index(&mut self, index: usize, data: T) -> bool {
        if index >= self.length {
            return false;
        }

        let mut current = self.head.unwrap();
        for _ in 0..index {
            current = self.nodes[current].as_ref().unwrap().next.unwrap();
        }

        self.nodes[current].as_mut().unwrap().data = data;
        true
    }

    pub fn find(&self, data: T) -> bool {
        let mut current = self.head;

        while let Some(index) = current {
            if self.nodes[index].as_ref().unwrap().data == data {
                return true;
            }
            current = self.nodes[index].as_ref().unwrap().next;
        }

        false
    }
 }  
mod test;