mod ds {
    pub mod linkedlist;
}

use ds::linkedlist::{
    linkedlist::LinkedList , 
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



    
     
    // linkedlist.pop();
    println!("linkedlist created : {:?}" , linkedlist);


}