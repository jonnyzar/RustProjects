use rand::prelude::*;

fn main() {

    /*
    Problem description
        - get some morse code like this ".--...-."
        - output all possible combination for replacement of two dots with two dashes
        - if less that two dots in the morse string then output empty list
    */

    //generate some random number within a boundary
    let length: usize = rand::thread_rng().gen_range(5, 150);

    let sample_input = generate_morse(length);

    get_options(&sample_input);

}

fn generate_morse (needed_length: usize) -> Vec<char> {
    /*
    This should generate a random combination of dots and dashes of a given length
    */

    let mut generated_vector = vec!['-'; needed_length];

    for i in 0..needed_length {

        if rand::random(){//boolean random generation

            generated_vector[i] = '.';

        }

    }


    test_char_vec(&generated_vector);

    return generated_vector;

}


fn test_char_vec (t: &Vec<char>){

    print!("Generated morse code is ");
    
    for c in t { 
        print!("{}", c);
    }
    
}


fn get_options (orig_str_vec: &Vec<char>){
    /*
    Here all possible combinations of dots and dashes are calculated
    And total number of resulting options is returned
    */

    // find dot positions
    let dots_collection: Vec<usize> = get_dots_num(&orig_str_vec);

    //find_pattern(&orig_str_vec, );

    //cloning original vector into the one that should be mutated over time
    let mut new_str_vec = orig_str_vec.clone();

    let mut total_options = 0;

    //while dots available
    for dot in dots_collection {

        //finding options for current dot pattern
        total_options += find_pattern(&new_str_vec, &dot);
        
        //replacing the first dot within current pattern with a dash to exclude it
        new_str_vec[dot] = '-';
    }

    println!("The total number of options equals {total_options}");

}

fn find_pattern (in_str: &Vec<char>, current_dot: &usize) -> usize {
    //takes input string fro a single given dot
    //and outputs vector with the options for this dot

    //ooptions for one dot
    let mut options_4dot: Vec<Vec<char>> = Vec::new();

    //track detection of first and second dots
    let mut dot_detected: bool = false;

    //index of finding
    let mut found_i: usize = 0;

    //iterate through the copy
    for (i, c) in in_str.iter().enumerate(){
        // println!("index {} char {}", i, c);

            if (*c == '.') && !(dot_detected) {
                //first ever dot in the morse sequence
                found_i = i;
                dot_detected = true;
            }
            else if (*c == '.') && dot_detected {
                //println!("found pattern at {} and {}", found_i, i);

                //temporary option string filled with '-'
                let mut option:Vec<char> = vec!['-'; in_str.len()];
            
                //place relevant dots within option
                option[found_i] = '.';
                option[i] = '.';

                //add option string to options collection for the current dot
                options_4dot.push(option);    
            }
        }

    let num_options = options_4dot.len();

    if num_options > 0 {
        println!("\n {num_options} options for dot {current_dot}");

        for opt in &options_4dot{
            test_char_vec(&opt);
            println!("\n");
        }

    }

    return num_options;

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

    return num_dots;
}

