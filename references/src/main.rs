fn main() {

     let s1 = String::from("hello");

     let len = calc_length(&s1); // passing reference to a func

}

fn calc_length(s: &String) -> usize { //func accepts some reference to a string
    s.len()
}
