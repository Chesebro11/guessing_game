// input output library
use std::io;
// random library
use rand::Rng;
// main function
fn main() {
    //print these lines to the screen
    println!("Guess the number!");

    // number generator using the rand::Rng library
                                        // this is the range (1..=100)
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

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
    println!("You guessed: {guess}");
}
