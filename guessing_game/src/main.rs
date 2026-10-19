use std::cmp::Ordering;
use std::io;

fn input(message: &str) -> String {
  println!("{message}");

  let mut str = String::new();
  io::stdin()
    .read_line(&mut str)
    .expect("Failed to read from stdio");

  String::from(str.trim())
}

fn main() {
  println!("---- Guessing Game ----");

  // Generate a random number between 1 and 100 (inclusive)
  let secret = fastrand::i32(1..=100);
  let mut tries = 0;

  loop {
    let guess = input("\nPlease input your guess");

    if guess == "quit" {
      println!("Bye!");
      break;
    }

    let guess = match guess.parse::<i32>() {
      Ok(num) => num,
      Err(_) => {
        println!("Invalid guess, enter a number between 1 and 100 or type 'quit' to exit");
        continue;
      }
    };

    tries += 1;
    match secret.cmp(&guess) {
      Ordering::Less => println!("Smaller!"),
      Ordering::Greater => println!("Bigger!"),
      Ordering::Equal => {
        println!("You guessed it right after {tries} tries!");
        break;
      }
    }
  }
}
