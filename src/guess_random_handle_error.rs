use std::cmp::Ordering;
use std::io;

use rand::{thread_rng, Rng};

pub fn guess_number() {
    let number = thread_rng().gen_range(1..=100);
    println!("Please guess a number between 1 and 100: ");
    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Cannot read user inut");
        let guess = guess.trim().parse::<i32>();
        // .expect("The input is not a number");
        let guess = match guess {
            Ok(number) => number,
            Err(_) => {
                println!("The input is not a number, please try again");
                continue;
            }
        };
        match guess.cmp(&number) {
            Ordering::Less => println!("Guess bigger"),
            Ordering::Greater => println!("Guess smaller"),
            Ordering::Equal => {
                println!("You guessed it!!");
                break;
            }
        }
    }
}
