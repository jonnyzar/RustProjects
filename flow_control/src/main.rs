#![allow(dead_code, unused_labels)]

fn main() {
    //println!("Hello, world!");

   // let resp: bool = if_func(30);

    //println!("Res is {resp}");

   // let out: i32 = doing_loop();

   // println!("{out}");

   // doing_while();
   // doing_for();
   // using_range();

   // anonymous_loop(11);

//always need to initialize array first
    let mut sample_arr: [i32; 10] = [0;10];
    
    fill_arr(&mut sample_arr);

    print_arr(&sample_arr);

    // so array can be still changed
    sample_arr[1] = 1000;

    print_arr(&sample_arr);

}


fn fill_arr (arr: &mut [i32]) {

    //even if array is passed like mutable reference it can still be accessed normally
    //unlike C...
    
    arr[0] = 1;

    for i in 1..arr.len(){
        arr[i] = arr[i-1] + 1; 
    }
}

//passing array as a reference to a function
fn print_arr (arr: &[i32]){

    for item in arr{
        println!("{item}");
    }


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



