use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Welcome to the Guessing Game");
    println!("Im thinking of a number between 1 and 100.");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please type your guess and press Enter:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num, 
            Err(_) => {
                println!("Please enter a number!");
                continue; 
            }
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small! Try again."),
            Ordering::Greater => println!("Too big! Try again."),
            Ordering::Equal => {
                println!("Congratulations, you guessed it!");
                break; 
            }
        }
    }
}
