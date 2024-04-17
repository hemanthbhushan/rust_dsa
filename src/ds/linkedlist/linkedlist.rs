use std::ops::Index;

#[derive(Debug)]
pub struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

#[derive(Debug)]
pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
    size: u64,
}

impl<T: std::fmt::Debug + std::fmt::Display> LinkedList<T> {
    pub fn new() -> LinkedList<T> {
        LinkedList {
            head: None,
            size: 0,
        }
    }

    pub fn add_at_start(&mut self, data: T) {
        let new_node = Node {
            data,
            next: self.head.take(),
        };
        self.head = Some(Box::new(new_node));
        self.size += 1;
    }

    pub fn add_at_end(&mut self, data: T) {
        if self.head.is_none() {
            self.add_at_start(data);
            return;
        }
        let mut temp = &mut self.head;
        while let Some(node) = temp {
            if node.next.is_none() {
                let new_node = Node {
                    data,
                    next: None,
                };
                node.next = Some(Box::new(new_node));
                self.size += 1;
                return;
            }
            temp = &mut node.next;
        }
    }

    pub fn insert_at_index(&mut self, index: u64, data: T) {
        if index == 0 {
            self.add_at_start(data);
            return;
        }
        if index >= self.size {
            self.add_at_end(data);
            return;
        }
        let mut current_node = &mut self.head;
        for _ in 1..index {
            if let Some(node) = current_node {
                current_node = &mut node.next;
            }
        }
        if let Some(node) = current_node {
            let new_node = Node {
                data,
                next: node.next.take(),
            };
            node.next = Some(Box::new(new_node));
            self.size += 1;
        }
    }

    pub fn delete_from_start(&mut self) -> Option<T> {
        self.head.take().map(|mut node| {
            self.head = node.next.take();
            self.size -= 1;
            node.data
        })
    }

    pub fn delete_from_end(&mut self) {
        if self.size <= 1 {
            self.head = None;
            self.size = 0;
            return;
        }
        let mut current_node = &mut self.head;
        for _ in 0..(self.size - 2) {
            if let Some(node) = current_node {
                current_node = &mut node.next;
            }
        }
        if let Some(node) = current_node {
            node.next = None;
            self.size -= 1;
        }
    }
}
