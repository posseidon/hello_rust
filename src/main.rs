use std::io;

fn main() {
  guess();
}

fn guess() {
  println!("Please input your guess.");
  let mut guess = String::new();

  io::stdin()
  .read_line(&mut guess)
  .expect("Failed to read line.");

  println!("You have guessed {guess}");
}