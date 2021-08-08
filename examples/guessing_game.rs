//! Based on the [guessing game] form the rust book.
//!
//! [guessing game]: https://doc.rust-lang.org/book/second-edition/ch02-00-guessing-game-tutorial.html

use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        let guess: u8 = async_std::task::block_on(
            asking::question()
                .repeat_message("Please input your guess: ")
                .min_max(1, 100)
                .help("The secert number is an integer between 1 and 100.\n")
                .feedback(|v| format!("You guessed: {}\n", v))
                .ask(),
        )
        .expect("Failed to read line");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
