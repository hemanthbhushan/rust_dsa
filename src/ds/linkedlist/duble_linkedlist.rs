use std::ptr::NonNull;
#[derive(Debug)]
pub struct Node<T>{
    pub value : T , 
    pub next : Option<NonNull<Node<T>>> , 
    pub prev : Option<NonNull<Node<T>>>
}

impl<T>  Node<T> {

    pub fn new(value : T) -> Self{
        Self{
             value , 
             next :None , 
            prev : None 
        }
    }
}
#[derive(Debug)]
pub struct LinkedList<T>{
    pub head : Option<NonNull<Node<T>>>  , 
    pub tail : Option<NonNull<Node<T>>>  ,
    length: u64
}

impl<T> LinkedList<T> {

pub fn new() -> Self{
        Self{
            head : None , 
            tail : None , 
            length : 0
        }
    }    
}

impl<T:std::fmt::Display> LinkedList<T> {

    pub fn insert_at_start(&mut self ,value : T){
        let mut node = Box::new(Node::new(value));
        node.next = self.head;
        node.prev = None;

        let node_ptr = NonNull::new(Box::into_raw(node));

        match self.head {
            None => self.tail = node_ptr,
            Some(mut head_ptr) => unsafe {
                head_ptr.as_mut().prev = node_ptr
            }
        }

        self.head = node_ptr;
        self.length += 1;
    }

    pub fn insert_at_tail(&mut self , value : T) {

        let mut node = Box::new(Node::new(value));
        node.next = None ; 
        node.prev = self.tail;
        let node_ptr = NonNull::new(Box::into_raw(node));
        match  self.tail {
            None => self.head = node_ptr,
            Some(mut tail_prt) => unsafe {
                tail_prt.as_mut().next = node_ptr
            } 
        }
        
        self.tail = node_ptr;
        self.length -= 1;
    }

    

     
}
