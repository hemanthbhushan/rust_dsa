use std::{ptr::NonNull};
use std::fmt::Debug;
#[derive(Debug)]
pub struct Node<T: Debug>{
    pub value : T , 
    pub next : Option<NonNull<Node<T>>> , 
    pub prev : Option<NonNull<Node<T>>>
}

impl<T:Debug>  Node<T> {

    pub fn new(value : T) -> Self{
        Self{
             value , 
             next :None , 
            prev : None 
        }
    }
}
#[derive(Debug)]
pub struct LinkedList<T: Debug>{
    pub head : Option<NonNull<Node<T>>>  , 
    pub tail : Option<NonNull<Node<T>>>  ,
    length: u64
}

impl<T:Debug> LinkedList<T> {

pub fn new() -> Self{
        Self{
            head : None , 
            tail : None , 
            length : 0
        }
    }    
}

impl<T:Debug> LinkedList<T> {

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
        self.length += 1;
    }


    pub fn delete_head(&mut self){
        if let Some(mut ptr) = self.head {
            unsafe {
              match ptr.as_mut().next{
                Some(mut nxt_ptr) => {
                    nxt_ptr.as_mut().prev = None;
                    self.head = Some(nxt_ptr);
                }
                None =>{
                    self.tail = Some(ptr)
                }
              }
        }
            
        }else {
           println!("Unable to delete head: List is empty")
            
        }
         
    }
    pub fn delete_tail_match(&mut self){
        match self.tail {
            Some(mut ptr) =>unsafe {
                match ptr.as_mut().prev  {
                    Some(mut ptr) => {
                        ptr.as_mut().next = None;
                        self.tail = Some(ptr);
                        self.length -= 1;
                    },
                    None => {
                        self.tail = None;
                        self.head = None;
                    }
           
                } 
            },
            None =>  println!("Unable to delete Tail: List is empty")
        }

    }

    pub fn delete_tail_map(&mut self){
        self.tail.map(|mut ptr|
            unsafe {
                match ptr.as_mut().prev  {
                    Some(mut ptr) => {
                      ptr.as_mut().next = None;
                      self.tail = Some(ptr);
                      self.length -= 1;
                    },
                    None => {
                        self.tail = None;
                        self.head = None;
                    }
                } 

            }
         );

    }

    pub fn display_from_head(&self) {
        let mut current_ptr = &self.head;
        while  let Some(prt) =  current_ptr{
            unsafe{
                print!("{:?}->",prt.as_ref().value);
                current_ptr = &prt.as_ref().next;
            } 
        }
        println!("END")
    }

    pub fn display_from_tail(&self) {
        let mut current_ptr = &self.tail;
        while  let Some(prt) =  current_ptr{
            unsafe{
                print!("{:?}->",prt.as_ref().value);
                current_ptr = &prt.as_ref().prev;
            } 
        }
        println!("START")
    }



     
}
