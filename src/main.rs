use rand::Rng;
use std::{cmp::Ordering, io};
fn main() {
    let correct = rand::rng().random_range(1..=100);
    println!("Hey guess a number from 1 to 100:");

    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(e) => {
                println!("Please enter a valid number, {e}");
                continue;
        }
    };

        match guess.cmp(&correct) {
            Ordering::Less => println!("You guessed too low"),
            Ordering::Greater => println!("You guessed too high"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        };
    }
}
