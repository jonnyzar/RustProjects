fn main() {
    //println!("Hello, world!");

    let in_str = String::from("Hello, world!");

    let mut num_dots: Vec<usize> = Vec::new();

    //let mut dot_chars = in_str.chars().collect::<Vec<char>>();

    

    for (i, c) in in_str.chars().enumerate(){

        if c == '.' {
            num_dots.push(i);
        }

    }

    
    for item in num_dots {
        println!(" {} ", item);
    }
    

    /*
    for (i,dot) in dot_chars.iter_mut().enumerate() {

        //prepare input string for analysis with previous dots replaced
        dot_chars[i] = 'x';
      
    }

    in_str = dot_chars.into_iter().collect();

    println!("{}", in_str);
*/
}
