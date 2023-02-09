fn main() {

    /*
    Problem description
        - get some morse code like this ".--...-."
        - output all possible combination for replacement of two dots with two dashes
        - if less that two dots in the morse string then output empty list
    */

    let sample_input: String = ".--...-.".to_string();

    test_string(&sample_input);

    split_str(&sample_input);

}

fn test_string (s: &str){
    println!("{}", s);
}

fn split_str (s: &str){
    for c in s.chars(){
        print!("{}", c);
    }
}
