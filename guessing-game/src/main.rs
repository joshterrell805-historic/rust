extern crate rand;

use std::io;
use std::io::Write;
use std::cmp::Ordering;
use rand::Rng;
use std::result;

fn main()
{

  let number = rand::thread_rng().gen_range(1, 101);

  println!("Guessing Game");

  loop
  {
    let guess = match read_number()
    {
      Ok(num) => num,
      Err(message) =>
      {
        println!("{}", message);
        continue;
      },
    };

    match guess.cmp(&number)
    {
      Ordering::Less => println!("Too small."),
      Ordering::Greater => println!("Too big."),
      Ordering::Equal =>
      {
        println!("You win!");
        break;
      },
    }
  }
}

fn read_number() -> result::Result<u32, &'static str>
{
  let mut input = String::new();

  print!("Enter a number: ");
  io::stdout().flush().ok().expect("Failed to flush stdio.");

  io::stdin().read_line(&mut input).ok().expect("Failed to read guess");

  return input.trim().parse::<u32>()
  .or(Err("Not a number!"));
}
