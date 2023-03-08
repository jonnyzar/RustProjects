use crate::address::Nil;

//generates an implementation of the Clone trait for a struct or enum.
#[derive(Clone)]
enum address{
    address(Box<Node>),
    Nil,
}

#[derive(Clone)]
struct Node{
    value: u32,
    next: address,
}

impl Node{

    //create a node
    fn new(value: u32) -> Self {
        Self { 
            value: value,
            next: Nil,
         }
    }

    //add a new node to the end of the existing

    fn append(&mut self, next_value: u32) {

        // create new node an point it to Nil
        let next_node = Node {
            value: next_value,
            next: address::Nil,
        };

        //point current node to new node
        self.next = address::address(Box::new(next_node))

    }


}

fn main(){
    
    let my_list = Node::new(732);

    assert_eq!(732, my_list.value);

    println!("{}", my_list.value);
   
}