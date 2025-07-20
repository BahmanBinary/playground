use std::io;

use guessing_game::inputs::Guess;
use rand;

fn main() {
    println!("Welcome to guessing game!");

    let random_number = rand::random_range(1..=100);

    loop {
        println!("Please guess a number:");

        let mut guess_input = String::new();

        io::stdin()
            .read_line(&mut guess_input)
            .expect("Failed to read line!");

        let guess = Guess::new(guess_input.trim().parse().unwrap());

        match guess {
            Ok(guess) => {
                let number = guess.value;

                if random_number == number {
                    println!("Congratulation!!! you find the target number: {number}")
                } else {
                    if random_number.abs_diff(number) <= 10 {
                        println!("OMG! you are close to target number!")
                    } else {
                        println!("Sorry! you are too far from target number!")
                    }

                    continue;
                }

                break;
            }

            Err(message) => {
                println!("{message}");

                continue;
            }
        }
    }
}
