# Rust Guessing Game

A simple command-line number guessing game built in Rust, inspired by Chapter 2 (“Programming a Guessing Game”) of *The Rust Programming Language* book.

## How the game works

- The program randomly selects a secret number within a fixed range (for example, 1–100).  
- You repeatedly enter guesses in the terminal.  
- After each guess, the program tells you whether your guess is too low, too high, or correct, and ends when you guess the number.

## Getting started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) toolchain installed (`rustc`, `cargo`).

You can verify your installation with:

```

rustc --version
cargo --version

```

### Running the game

Clone this repository and run it with Cargo:

```

git clone https://github.com/Satya-Sherkar/rust_guessing_game.git
cd rust_guessing_game
cargo run

```

Follow the on-screen instructions to enter your guesses.

## Project structure

This project uses the standard Cargo layout:

- `Cargo.toml` – Project metadata and dependencies (such as `rand` for random number generation).  
- `src/main.rs` – The main source file containing the game logic (reading user input, generating the secret number, comparing guesses, and printing hints).

## Concepts demonstrated

This small project is mainly for learning Rust fundamentals and shows how to use:

- `use` statements to bring types like `std::io` and external crates (e.g., `rand`) into scope.  
- Mutable variables (`let mut`) and shadowing to parse input from strings into numbers.  
- Error handling with `Result` and `expect` when reading from standard input.  
- `match` expressions and ordering comparisons (`Ordering`) to check whether the guess is less than, greater than, or equal to the secret number.  
- A `loop` with `break` to keep asking for guesses until the player wins.
