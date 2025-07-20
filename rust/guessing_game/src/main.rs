use std::io;

use guessing_game::inputs::Guess;
use rand;
use std::cmp::Ordering;

fn main() {
    println!("Welcome to guessing game!");

    let target_number = rand::random_range(1..=100);

    loop {
        println!("Please guess a number:");

        let mut guess_input = String::new();

        io::stdin()
            .read_line(&mut guess_input)
            .expect("Failed to read line!");

        let guess = Guess::new(guess_input.trim().parse().unwrap());

        match guess {
            Ok(guess) => {
                let guess_number = guess.value;

                match guess_number.cmp(&target_number) {
                    Ordering::Equal => {
                        println!("Congratulation!!! You find the target number: {guess_number}");
                    }
                    Ordering::Greater => {
                        if target_number.abs_diff(guess_number) <= 10 {
                            println!(
                                "OMG! Your guess is bigger but you are close to target number!"
                            );
                        } else {
                            println!(
                                "Sorry! Your guess is bigger but you are too far from target number!"
                            );
                        }
                    }
                    Ordering::Less => {
                        if target_number.abs_diff(guess_number) <= 10 {
                            println!(
                                "OMG! Your guess is smaller but you are close to target number!"
                            );
                        } else {
                            println!(
                                "Sorry! Your guess is smaller but you are too far from target number!"
                            );
                        }
                    }
                }

                if let Ordering::Equal = target_number.cmp(&guess_number) {
                } else {
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
