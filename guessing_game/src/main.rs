// using standard library
use std::io;
use rand::Rng;
use std::cmp::Ordering;


fn main() {
    println!("Guess!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop { //infinite loop
        println!("Input your guess");

        //creating new MUTABLE variable, as default rust variables are immutable
        let mut guess = String::new(); //new() is function of String type

        io::stdin()
            .read_line(&mut guess) //making mutable reference
            .expect("Failed"); //

        //println!("You guessed: {guess}");

        // let guess = guess.trim().parse::<u32>(); //type casting
        // trim() eliminates whitespaces
        // parse() with turbofish ... wont work
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };


        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;}
            }
    }

}
