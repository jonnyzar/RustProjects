fn main() {
    //println!("Hello, world!");

    let mut in_str = String::from("Hello, world!");

    let mut dot_chars = in_str.chars().collect::<Vec<char>>();

    let mut num_dots: Vec<usize> = Vec::new();

    for (i, c) in dot_chars.iter().enumerate(){
        // println!("index {} char {}", i, c);

        if *c == '.' {
            num_dots.push(i);
        }

    }

    for dot in &mut dot_chars {

        //prepare input string for analysis with previous dots replaced
        dot_chars[*dot as usize] = 'x';
      
    }

    in_str = dot_chars.into_iter().collect();

    println!("{}", in_str);

}
