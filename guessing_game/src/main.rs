use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    // inifinite loop
    loop {
        println!("Please input your guess.");

        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("You guessed: {guess}...");
        // Shadowing guess
        let guess: u32 = match guess.trim().parse::<u32>() {
            Ok(n) => n,
            Err(e) => {
                println!("...but I need a number... {e}");
                // continue the loop on parse errors
                continue;
            }
        };
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                // break the loop when correctly guessed
                break;
            }
        }
    }
}
