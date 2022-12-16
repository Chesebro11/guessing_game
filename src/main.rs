// input output library
use std::io;
// random library
use rand::Rng;

use std::cmp::Ordering;
// main function
fn main() {
    //print these lines to the screen
    println!("Guess the number!");

    // number generator using the rand::Rng library
                                        // this is the range (1..=100)
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");
// this allows the program to loop through this logic
loop {
    println!("Please input your guess.");

    // creating a variable to store user input let/mut allows change?
    // assigning a new empty string to variable
    let mut guess = String::new();

    // using the input output library to handle user input
    io::stdin()
        // call the read_line method to pass the user's input into guess
        // & indicates a reference, allowing you yo access code without copying data into memory
        .read_line(&mut guess)
        // fail handling using a result instance?
        .expect("Failed to read line");
        // prints the users guess back to them by passing the variable into a string using {}
        let guess: u32 = guess.trim().parse().expect("Please type a number");

    println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You win!");
                // creates the condition to stop the loop
                break;
            }
        }  
    }
}


// running cargo doc --open can take you to documentation