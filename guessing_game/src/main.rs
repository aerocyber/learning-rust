use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    
    let secret_number = rand::thread_rng().gen_range(1..=100); // TODO: Remove this line or comment it
    
    println!("The secret number is: {secret_number}");
    
    println!("Please insert your guess.");
    
    let mut guess = String::new();
    
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
        
    let guess: i32 = guess.trim().parse().expect("Please enter a number!");
        
    println!("Your guess: {guess}");
    
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small."),
        Ordering::Greater => println!("Too Big."),
        Ordering::Equal => println!("You Win!"),
    }
}
