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

}

fn change (s: &mut String) {
    s.push_str(" and Bye!");
}

fn calc_length(s: &String) -> usize { //func accepts some reference to a string
    s.len() // borrowing
}// s goes out of scope but is not dropped



