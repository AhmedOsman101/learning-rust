use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn input(msg: &str) -> String {
  println!("{msg}");

  let mut res = String::new();
  io::stdin()
    .read_line(&mut res)
    .expect("Failed to read from stdio");

  String::from(res.trim())
}

fn main() {
  println!("---- Guessing Game ----");

  // Generate a random number between 1 and 100 (inclusive)
  let secret: u32 = rand::rng().random_range(1..=100);

  loop {
    let guess = input("\nPlease enter your guess.");

    if guess == "quit" {
      println!("Bye!");
      break;
    }
    let guess = match guess.parse() {
      Ok(num) => num,
      Err(_) => {
        println!("Invalid guess, enter a number between 1 and 100 or type 'quit' to exit");
        continue;
      }
    };

    match secret.cmp(&guess) {
      Ordering::Less => println!("Smaller!"),
      Ordering::Greater => println!("Bigger!"),
      Ordering::Equal => {
        println!("You guessed it right!");
        break;
      }
    }
  }
}
