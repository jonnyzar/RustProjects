fn main() {

    let s1 = String::from("hello");

    let len = calc_length(&s1); // passing reference to a func

    println!("{}", len);

    let mut s_change = String::from("hi");

    change(&mut s_change);

    println!("{s_change}");
}

fn change (s: &mut String) {
    s.push_str(" and Bye!");
}

fn calc_length(s: &String) -> usize { //func accepts some reference to a string
    s.len() // borrowing
}// s goes out of scope but is not dropped
