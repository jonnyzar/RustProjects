fn main() {

    #[allow(unused_variables)]

    {//nothing valid yet

        let s = "ok"; //valid from here

    } //out of scope

    //mutable string

    let mut s = String::from("hello");

    s.push_str(", world!"); // append to string

    println!("{}", s); // print hello world

    let string1 = "ok";

    println!("{}", string1); //unmutable string


    let _s_nok = s;

    // println!("{}", s); // no more valide since value moved
    // from s to _s_nok


    //deep clone

    let s_2 = String::from("hello2");

    let s_2_deep = s_2.clone(); //not efficient

    println!("{} and clone {}", s_2, s_2_deep);

}
