//RUST uses snake style

fn another_func(){
    println!("Some other function's print");
}

fn func_args(x: i32, some_letter: char){
    println!("arguments of this are {x} and {some_letter}");
}

/*
Functions can return values to the code that calls them.
 We don’t name return values, but we must declare their
  type after an arrow (->)
*/

fn returning_int_vals (x: i32) -> i32 {
    500 + x
}


fn main() {
    println!("Hello, world!");

    //chars use single quotes in rust
    func_args(500, 'a');

    another_func();

    //expression returning a value
    let y = {
        let x = 5;
        x + 1
    };

    println!("expression is {y}");

    let ret = returning_int_vals(10000000);

    println!("return is {ret}");

}


