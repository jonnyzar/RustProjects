#![allow(dead_code)]

fn main() {

    /*
    Problem description
        - get some morse code like this ".--...-."
        - output all possible combination for replacement of two dots with two dashes
        - if less that two dots in the morse string then output empty list
    */

    let sample_input: String = ".--...-.".to_string();


    //test_string(&sample_input);

    //split_str(&sample_input);

    //This creates a new String value that is separate from the original
    combine_morse(&sample_input);

}

fn test_string (s: &str){
    println!("{}", s);
}

fn test_char_vec (t: Vec<char>){
    
    for c in t { 
        print!("{}", c);
    }
    
}

fn refactor_str (s: &str) -> Vec<char>{
    
    let mut output_vec: Vec<char> = Vec::new();
    
    for c in s.chars(){
        output_vec.push(c);
    }

    return  output_vec
}

fn combine_morse (in_morse: &str){

    // refactor the input string to a Vec<char>
    let orig_str_vec: Vec<char> = refactor_str(in_morse);

    // created copy to later alter it as morse code changes
    let mut copy_orig: Vec<char> = orig_str_vec.clone();

    //create vector to store possible combinations 
    let mut options: Vec<Vec<char>> = Vec::new();

    //track detection of first and second dots
    let mut det_dot: bool = false;

    //add combination
    options.push(copy_orig);

    //options.push(in_morse);
    //options.push(String::from("Hello"));

    
    for code in options {
        test_char_vec(code);
    }
    
}

