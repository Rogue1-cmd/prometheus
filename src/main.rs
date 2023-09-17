use std:: io; // bring the io input/output library into scope.
use rand::Rng; // Import the rand crate for random number generation.
use std::cmp::Ordering; // Import the Ordering enum from the std::cmp module.

fn main () {
 
    println!("\nGuess your Number:");

    // Generate a random number between 1 and 100 (inclusive) and store it in secret_number.
    let secret_number = rand::thread_rng().gen_range(1..=100);
    
    // Create an infinite loop to allow multiple guesses.
    loop {
        println!("\nPlease Input your guess.");

        let mut guess = String::new();//create mutable variable to store user input 
        
        // Read a line from the standard input and store it in the guess variable.
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        // Parse the user's input as a u32 (unsigned 32-bit integer).
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num, // If parsing succeeds, assign the parsed number to guess.
            Err(_) => continue,// If parsing fails, ignore this input and continue the loop.
        };

        println!("You guessed:{guess}");
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("\nToo small!\n"),
            Ordering::Greater => println!("\nToo big!\n"),
            Ordering::Equal =>{
                println!("\nYou win!\n");
                break; // Exit the loop as the game is over.
        }
        }
    }
}