use std::ptr::NonNull;

// Define a simple Node struct
#[derive(Debug)]
struct Node<T> {
    data: T,
    next: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    // Constructor for Node
    fn new(data: T) -> Self {
        Node {
            data,
            next: None,
        }
    }
}

// Define a LinkedList struct
#[derive(Debug)]
pub struct LinkedList<T> {
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    length: usize,
}

impl<T : std::fmt::Debug + std::fmt::Display> LinkedList<T> {
    // Constructor for LinkedList
    pub fn new() -> Self {
        LinkedList {
            head: None,
            tail: None,
            length: 0,
        }
    }

    // Method to push a new element onto the end of the list
    pub fn push(&mut self, data: T) {
        let new_node = Box::new(Node::new(data));
        let new_node_ptr = Some(NonNull::from(Box::leak(new_node)));
         println!("new_node_ptr -----------{:?}",new_node_ptr);
        // Update head and tail pointers
        if let Some(mut tail_ptr) = self.tail {
            unsafe {
                tail_ptr.as_mut().next = new_node_ptr;
            }
            self.tail = new_node_ptr;
        } else {
            self.head = new_node_ptr;
            self.tail = new_node_ptr;
        }
        self.length += 1;
    }

    // Method to print the list
    pub fn print(&self) {
        let mut current_node = self.head;
        print!("LinkedList: ");
        while let Some(node_ptr) = current_node {
            unsafe {
                print!("{:?}, ", node_ptr.as_ref().data);
                current_node = node_ptr.as_ref().next;
            }
        }
        println!();
    }
}


