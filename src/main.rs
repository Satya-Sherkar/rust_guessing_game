use std::io;

use rand::Rng;

fn main() {
    println!("Guess the number!");

    // rand version ^0.8.0 < ^0.9.0 uses `rand::thread_rng()` and `gen_range(start..=end)`(Deprecated).
    let secret_number = rand::rng().random_range(1..=100); // updated methods.

    println!("The Secret number is: {secret_number}");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}
