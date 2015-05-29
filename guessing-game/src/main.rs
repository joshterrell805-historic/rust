extern crate rand;

use std::io;
use std::io::Write;
use std::cmp::Ordering;
use rand::Rng;

fn main()
{

  let number = rand::thread_rng().gen_range(1, 101);

  println!("Guessing Game");
  println!("The secret number is: {}", number);
  let mut input = String::new();

  loop
  {
    print!("Enter a guess: ");
    io::stdout().flush().ok().expect("Could not flush stdout.");

    input.clear();
    io::stdin().read_line(&mut input)
    .ok()
    .expect("Failed to read line!");

    let guess : u32 = match input.trim().parse()
    {
      Ok(num) => num,
      Err(_) => {
        println!("Not a number.");
        continue;
      }
    };

    println!("Your input: {}", guess);

    match guess.cmp(&number)
    {
      Ordering::Less => println!("Too small."),
      Ordering::Greater => println!("Too big."),
      Ordering::Equal => {
        println!("You win!");
        break;
      },
    }
  }
}
