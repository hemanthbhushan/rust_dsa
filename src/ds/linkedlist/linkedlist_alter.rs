// Define a Node struct to represent each element in the linked list
struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    // Constructor method to create a new node
    fn new(data: T , next :Option<Box<Node<T>>> ) -> Self {
        Node { data, next }
    }
}

// Define a LinkedList struct to manage the list
pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T: std::fmt::Display> LinkedList<T> {
   pub fn new(item : T) -> Self {
    Self{
        head : Some(Box::new(Node::new(item , None)))
    }
   }

   pub fn add_val_at_start(&mut self , item : T){
      let next  = self.head.take();

      self.head = Some(Box::new(Node::new(item , next)
    )) 
}

   pub fn add_val_at_end(&mut self , item : T){

    let current_node = &mut self.head;

    while let Some(node) = current_node{
        if let None = node.next {
        }
    }
   }
}


