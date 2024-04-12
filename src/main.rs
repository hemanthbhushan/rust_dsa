#[derive(Debug)]
struct Node<T> {
    data : T ,
    next : Option <Box<Node<T>>>
}

#[derive(Debug)]
struct Linkedlist<T> {
    head :  Option<Box<Node<T>>>,
}

impl<T : std::fmt::Debug +  std::fmt::Display> Linkedlist<T> {
    fn new() -> Linkedlist<T>{
        Linkedlist{
            head : None
        }
    }

    fn push(&mut self , data : T) {
        let next = self.head.take();
        let node = Node{
            data  ,
            next
        }; 
        self.head = Some(Box::new(node))
    }

    fn pop(&mut self) { 
    //     let current_node = self.head.take().unwrap();
    //    self.head = current_node.next 
    let data = self.head.take().map(| node| {
         self.head = node.next; 
        node.data
    });
}
}
    

fn  main() {

    let mut linkedlist : Linkedlist<i32>= Linkedlist::new();
     println!("linkedlist created : {:?}" , linkedlist);

     linkedlist.push(32);
    //  linkedlist.push(33332);
    //  linkedlist.push(344546542);
     
     println!("linkedlist created : {:?}" , linkedlist);
    linkedlist.pop();
    println!("linkedlist created : {:?}" , linkedlist);


}