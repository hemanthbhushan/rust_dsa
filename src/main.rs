mod ds {
    pub mod linkedlist;
}

use ds::linkedlist::{
    linkedlist::LinkedList , 
    linkedlist_alter::LinkedList as LinkedList_alter , 

};



fn  main() {

    let mut linkedlist : LinkedList<i32>= LinkedList::new();
    //  println!("linkedlist created : {:?}" , linkedlist);
     linkedlist.add_at_start(11);
      linkedlist.add_at_start(33332);
      linkedlist.add_at_start(344546542);
      println!("linkedlist created : {:?}" , linkedlist);
     linkedlist.insert_at_index(2 ,32);

    println!("linkedlist created : {:?}" , linkedlist);

     linkedlist.delete_from_end();
     linkedlist.delete_from_end();

     linkedlist.delete_from_end();
     linkedlist.delete_from_end();
     


    let mut list = LinkedList_alter::new();
    list.push(1);
    list.push(2);
    list.push(3);
    list.print(); // Output: LinkedList: 1, 2, 3,


    
     
    // linkedlist.pop();
    println!("linkedlist created : {:?}" , linkedlist);

}