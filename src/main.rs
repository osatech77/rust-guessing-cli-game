use rand::Rng;
use std::{cmp::Ordering, io};
fn main() {
    let mut how_many = String::new();
    println!("How many numbers do you want to generate?");
    io::stdin()
        .read_line(&mut how_many)
        .expect("Failed to read line");
    let num_guesses: u32 = how_many.trim().parse().expect("Please type a number!");

    let mut correct = Vec::new();

    for _ in 0..num_guesses {
        correct.push(rand::rng().random_range(1..=100));
    }

    println!("{:?}", correct); // For debugging purposes
    let mut guesses_made = 0;

    while guesses_made < num_guesses {
        println!("Hey guess a number from 1 to 100:");

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

        match guess.cmp(&correct[guesses_made as usize]) {
            Ordering::Less => println!("You guessed too low"),
            Ordering::Greater => println!("You guessed too high"),
            Ordering::Equal => {
                println!("You win!");
                guesses_made += 1;
                if guesses_made < num_guesses {
                    println!(
                        "Now guess the next number! You have guessed {}/{} numbers correctly.",
                        guesses_made, num_guesses
                    );
                }
            }
        };
    }
    println!("You guessed all numbers correctly!");
    for item in correct {
        println!("The number was: {}", item);
    }
}
