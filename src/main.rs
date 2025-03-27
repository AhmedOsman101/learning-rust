use std::io;

fn main() {
  println!("---- Guessing Game ----\n");

  let mut guess = String::new();

  println!("Enter a guess");

  io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line");

  let guess: u32 = guess
    .trim()
    .parse()
    .expect("Invalid input, expected a number");

  println!("your guess is {}", guess);
}
