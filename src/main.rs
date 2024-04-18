mod ds {
    pub mod linkedlist;
}

use ds::linkedlist::{
    linkedlist::LinkedList , 
    duble_linkedlist::LinkedList as DubleLinkedList , 

};



fn  main() {

    // let mut linkedlist : LinkedList<i32>= LinkedList::new();
    // //  println!("linkedlist created : {:?}" , linkedlist);
    //  linkedlist.add_at_start(11);
    //   linkedlist.add_at_start(33332);
    //   linkedlist.add_at_start(344546542);
    //   println!("linkedlist created : {:?}" , linkedlist);
    //  linkedlist.insert_at_index(2 ,32);

    // println!("linkedlist created : {:?}" , linkedlist);

    //  linkedlist.delete_from_end();
    //  linkedlist.delete_from_end();

    //  linkedlist.delete_from_end();
    //  linkedlist.delete_from_end();
     


    let mut list = DubleLinkedList::new();
    list.insert_at_start(1);
    list.insert_at_start(3);
    list.insert_at_tail(2222);

    println!("{:?}" , list);



     match  list.head {
        Some(ptr) => unsafe {
           match  ptr.as_ref().next {
            Some(ptr)=> println!("{:?}innerrr " , ptr.as_ref()) ,
            None => panic!("nonee") 
           }
        } , 
        None => println!("cheee")
     }



     match  list.tail {
        Some(ptr) => unsafe {
            println!("{:?} tail pttrrrr",ptr.as_ref())
        } , 
        None => panic!("nonee") 
     }
    // linkedlist.pop();
    println!("linkedlist created : {:?}" , list.head);

}