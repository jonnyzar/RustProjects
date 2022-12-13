fn main() {

    #[allow(unused_variables)]

    {//nothing valid yet

        let s = "ok"; //valid from here

    } //out of scope

    //mutable string

    let mut s = String::from("hello"); // s alive

    s.push_str(", world!"); // append to string

    println!("{}", s); // print hello world

    let string1 = "ok";

    println!("{}", string1); //unmutable string


    let _s_nok = s; // s dead

    // println!("{}", s); // no more valide since value moved
    // from s to _s_nok


    //deep clone

    let s_2 = String::from("hello2");

    let s_2_deep = s_2.clone(); //not efficient

    println!("{} and clone {}", s_2, s_2_deep);

    //Ownership and functions

    let test_string = String::from("test_string alive");

    consume_test_string(test_string);

    //println!("{}", test_string); //test_string dead due to borrowing


    let xxx = 555;

    use_xxx_fixed_variable(xxx);

    // let xxx = 555; // can be redefined for more clarity

    println!("{xxx}"); //fixed variable still alive

    //return and ownership

    let given_ownership = some_giver();

    println!("{}", given_ownership);

    let provider =  String::from("I provided");

    let taker = provide(provider);

    println!("{}", taker);

    let (x,y) = return_tuple(taker);

    println!("{}, {}", x,y);

}

fn return_tuple (s: String) -> (usize, usize){
    let x = s.len();
    let y = x + 1;

    (x,y)
}

fn provide (s: String) -> String {
    let out = s;
    out
}

fn some_giver() -> String {

    let s = String::from("Hello given");

    s // return value
}

fn consume_test_string (some_string: String){
    println!("{}", some_string);
}

fn use_xxx_fixed_variable (x: i32){
    println!("{x}");
}
