fn main() {
    println!("Hello, world!");

    let resp: bool = if_func(30);

    println!("Res is {resp}");

    let out: i32 = doing_loop();

    println!("{out}");

    doing_while();
    doing_for();
    using_range();

    anonymous_loop(11);
}

fn anonymous_loop (max_iter: i32) {
    let mut i: i32 = 1;

    for _ in 0..max_iter {
        println!("{}",i);
        i += 1;
    }
}

fn if_func (number: i32) -> bool {
    
    // With "if" you must be explicit 
    // and always provide if with a Boolean as its condition
    if number < (5+1) {
        true
    }
    else if number % 2 == 0 {
        true
    }
    else {
        false
    }
}

fn doing_loop () -> i32{
    
    let mut counter = 0;
    
    'counter: loop{

        counter +=1;

        println!("Hello again");

        if counter == 5{
            
            break counter  
        }
    }
}

fn doing_while (){

    let mut w_num = 3;

    let arr = [13,14,15];

    while w_num != 0 {
        w_num -=1;
        println!("ok! {}", arr[w_num]);
    }
}

fn doing_for (){

    let big_arr = [1,2,3,4,5,6,7,8,9];

    for element in big_arr {
        println!("{element}");
    }

}

fn using_range (){
    for i in (1..10).rev(){
        println!("Counting down! {i}");
    }
}



