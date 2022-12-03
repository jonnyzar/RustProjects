fn main() {

    //normal immutable
    let y = 1;
    println!("immutable y is {y}");

    //mutable 
    let mut x = 5;
    println!("mutable x is {x}");
    
    x = 6;
    println!("new value assigned to x is {x}");

    //constant
    const MILLISECONDS_IN_HOUR: u32 = 1000 * 3600;


    println!("MILLISECONDS_IN_HOUR = {MILLISECONDS_IN_HOUR}");

    /*
    
    Shadowing
    
    */

    let x = 90;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");

    }

    println!("The final value of x is: {x}");


}
