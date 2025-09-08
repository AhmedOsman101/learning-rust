use std::io;

fn main() {
  println!("Guess the number!");

  let secret: u8 = 50;

  loop {
    println!("\nPlease enter your guess.");

    let mut guess = String::new();

    io::stdin()
      .read_line(&mut guess)
      .expect("Failed to read from stdio");

    let guess: u8 = guess
      .trim()
      .parse()
      .expect("The input should be a valid integer");

    if guess < secret {
      println!("Bigger!");
    } else if guess > secret {
      println!("Smaller!");
    } else {
      println!("You guessed it right!");
      break;
    }
  }
}
