use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    println!("Guess the number!");

    // rand version ^0.8.0 < ^0.9.0 uses `rand::thread_rng()` and `gen_range(start..=end)`(Deprecated).
    let secret_number = rand::rng().random_range(1..=100); // updated methods.

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!! 😄"),
            Ordering::Greater => println!("Too big!! 😄"),
            Ordering::Equal => {
                println!("You guessed it right!! ✅");
                break;
            }
        }
    }
}
