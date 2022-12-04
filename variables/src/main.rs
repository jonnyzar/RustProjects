fn main() {

    //normal immutable
    let y = 1;
    println!("immutable y is {y}");

    //mutable 
    let mut x = 5;
    println!("mutable x is {x}");

     //mutable max int
     let mut k = u32::MAX;
     println!("mutable k is {k}");

     k -= 1;
     println!("mutable k after '-1' {k}");
     // if you put +1 you see that overflow here is impossible
    
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


    //conver String to int


    let guess: u32 = "42".parse().expect("Not a number!");

    println!("guess from string is {guess}");

    let heart_eyed_cat = '😻';

    println!("{heart_eyed_cat}");

    let some_tuple: (i32, f64, u8) = (50, 56.3, 23);


    //destructure tuple

    let (x,y,z) = some_tuple;

    println!("tup is {x}, {y}, {z}");

    //accessing values one by one

    let first = some_tuple.0;
    let second = some_tuple.1;
    let third = some_tuple.2;

    println!("tup is {first}, {second}, {third}");

    let _unit_tup = ();

/************************* ARRAY TYPE ***************************/

let some_little_array = [1,2,3,4,5];

// or like so

let type_array: [i32;5] = [11,22,33,44,55];

// define array with same values of elements

let zero_point = [0;3]; // [0,0,0]

//accessing array elements

let first = type_array[0];

println!("{first}");




}
