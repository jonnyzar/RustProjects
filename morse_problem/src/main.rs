#![allow(dead_code)]

fn main() {

    /*
    Problem description
        - get some morse code like this ".--...-."
        - output all possible combination for replacement of two dots with two dashes
        - if less that two dots in the morse string then output empty list
    */

    let sample_input: String = String::from(".--...-.");

    println!("Original {}", sample_input);


    //test_string(&sample_input);

    //split_str(&sample_input);

    //This creates a new String value that is separate from the original
    combine_morse(&sample_input);

}

fn test_string (s: &str){
    println!("{}", s);
}

fn test_char_vec (t: &Vec<char>){
    
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
    /*
    Here all possible combinations of dots and dashes are calculated
    */

    // refactor the input string to a Vec<char>
    let mut orig_str_vec: Vec<char> = refactor_str(in_morse);

    //create vector to store possible combinations 
    let mut options: Vec<Vec<char>> = Vec::new();

      // find dot positions
    let dots_collection: Vec<usize> = get_dots_num(&orig_str_vec);

    //while dots available
    // use iter_mut
    for dot in &dots_collection {

        //prepare input string for analysis with previous dots replaced
        let mut dot_string = &orig_str_vec;
        &dot_string[dot] = 'x';

        find_pattern(&dot_string);
      
    }



    //add combination
    //options.push(copy_orig);

    //options.push(in_morse);
    //options.push(String::from("Hello"));

    /*
    for code in options {
        test_char_vec(code);
    }
    */
}

fn find_pattern (in_str: &Vec<char>) -> Vec<char> {
    //takes input string and outputs vector with the options for the detected dot

    //mutable vector copy for later input of possible option
    let mut option = in_str.clone();

    //track detection of first and second dots
    let mut dot_detected: bool = false;

    //index of finding
    let mut found_i: usize = 0;

    //iterate through the copy
    for (i, c) in in_str.iter().enumerate(){
        // println!("index {} char {}", i, c);

            if (*c == '.') && !(dot_detected) {
                //first ever dot in the morse sequence
                println!("found '{}' at {}", c, i);
                found_i = i;
                option[i] = '-'; 
                dot_detected = true;
            }
            else if (*c == '.') && dot_detected {
                println!("found pattern at {} and {}", found_i, i);
                option[i] = '-';
                break;
            }



        }

    test_char_vec(&option);

    return option;

}

fn get_dots_num(in_str: &Vec<char>) -> Vec<usize> {
    /*
    Get amount of dots from the code to determine amount of iteration
    AND
    their indexes
    */

    let mut num_dots: Vec<usize> = Vec::new();

    for (i, c) in in_str.iter().enumerate(){
        // println!("index {} char {}", i, c);

        if *c == '.' {
            num_dots.push(i);
        }

    }

    /*
    for item in &num_dots {
        print!(" {} ", item);
    }
    */

    return num_dots;

}

