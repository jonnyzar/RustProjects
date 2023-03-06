//generates an implementation of the Clone trait for a struct or enum.

#[derive(Clone)]
enum address{
    address(Box<myList>),
    Nil,
}

#[derive(Clone)]
struct myList{
    value: u32,
    next: address,
}


fn main(){
    
}