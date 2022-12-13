fn main() {

    let s1 = String::from("hello");

    let len = calc_length(&s1); // passing reference to a func

    println!("{}", len);

}

fn calc_length(s: &String) -> usize { //func accepts some reference to a string
    s.len() // borrowing
}// s goes out of scope but is not dropped
