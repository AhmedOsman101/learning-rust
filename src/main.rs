use rand;
use rand::Rng;
use std::io;

fn main() {
  println!("---- Guessing Game ----\n");

  let secret_number: u32 = rand::rng().random_range(1..101);

  let mut guess: String = String::new();

  println!("Enter a guess");

  io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line");

  let mut guess_value: u32 = guess
    .trim()
    .parse()
    .expect("Invalid input, expected a number");

  while guess_value != secret_number {
    if guess_value > secret_number {
      println!("Smaller\n");
    } else if guess_value < secret_number {
      println!("Bigger\n");
    }

    println!("Enter a guess");

    guess = String::new();

    io::stdin()
      .read_line(&mut guess)
      .expect("Failed to read line");

    guess_value = guess
      .trim()
      .parse()
      .expect("Invalid input, expected a number");
  }

  println!("\nYou guessed it right!");
}
