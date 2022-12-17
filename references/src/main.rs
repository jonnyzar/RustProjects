fn main() {

    let s1 = String::from("hello");

    let len = calc_length(&s1); // passing reference to a func

    println!("{}", len);

    let mut s_change = String::from("hi");

    change(&mut s_change);

    println!("{s_change}");

// creating multiple mutable varibles
    {
        let _k = &mut s_change;
    }

    let _f = &mut s_change;

// but for immutable no need to set scope

    let mut x = 5;
    
    let ref_x = &x; //immutable reference of a mutable variable
    let ref_xx = &x; 

    println!("{x}, {ref_x} and {ref_xx}"); // scope of immutable refs end when the println is used

    let ref_yyy = &mut x;

    println!("{ref_yyy}");

/*

Dangling References

*/

  //  let dangle = creating_dangling();

/*

Slice

*/

/* Problem: find first word in the string before space and count number of letters in it*/

let mut hello = String::from("Helloo Worlds!");

let word = first_word(&hello);

println!("{word}");


hello.clear();

// BUT after hello is cleared word is there without use and wastes ressources

/*  SOLUTION: String slices  */

let slice_str = String::from("Hello String Slice");

let slice = &slice_str[0..10]; //start from zero to second element

println!("{slice}");

first_word_opt(&hello);

}

fn first_word_opt (s: &str) -> &str { //str string slice and string! better

    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate(){
        if item == b' ' {
            return &s[..i];
        }
    }

    return &s[..];

}


fn first_word(s: &String) -> usize {

    let bytes = s.as_bytes(); // represent string as bytes array

    for (i, &item) in bytes.iter().enumerate(){ //enumerate returns tuple (index, &element)
        if item == b' ' {
            return i;
        }
    }

    s.len()
}


/*
//DO NOT USE
fn creating_dangling() -> &String {
    //this function is false and going to dangle

    let s = String::from("Hello, I dangle!");

    &s
}
*/

fn change (s: &mut String) {
    s.push_str(" and Bye!");
}

fn calc_length(s: &String) -> usize { //func accepts some reference to a string
    s.len() // borrowing
}// s goes out of scope but is not dropped



