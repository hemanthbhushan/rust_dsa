// Define a Node struct representing each element in the linked list
struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

// Define a LinkedList struct to hold the head of the list
struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T : std::fmt::Display > LinkedList<T> {
    // Constructor to create an empty linked list
    fn new() -> Self {
        LinkedList { head: None }
    }

    // Method to push a new element to the front of the linked list
    fn push(&mut self, data: T) {
        let new_node = Node {
            data: data,
            next: self.head.take(),
        };
        self.head = Some(Box::new(new_node));
    }

    // Method to pop the first element from the linked list
    fn pop(&mut self) -> Option<T> {
        self.head.take().map(|mut node| {
            self.head = node.next.take();
            node.data
        })
    }

    // Method to check if the linked list is empty
    fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    // Method to print the linked list
    fn print(&self) {
        let mut current = &self.head;
        print!("List: ");
        while let Some(node) = current {
            print!("{} -> ", node.data);
            current = &node.next;
        }
        println!("None");
    }
}

fn main() {
    // Create a new linked list
    let mut list: LinkedList<i32> = LinkedList::new();

    // Push some elements to the linked list
    list.push(1);
    list.push(2);
    list.push(3);

    // Print the linked list
    list.print();

    // Pop an element from the linked list
    if let Some(data) = list.pop() {
        println!("Popped element: {}", data);
    }

    // Print the linked list again
    list.print();
}
