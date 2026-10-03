use std::io;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    // Make a "random" number from 1 to 100 using the current time
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();
    let secret = (nanos % 100) + 1;

    let mut tries = 0;

    println!("Guess the number between 1 and 100!");

    loop {
        println!("Your guess:");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        // Turn the text into a number, or ask again if it's not valid
        let guess: u32 = match input.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
            }
        };

        tries += 1;

        if guess < secret {
            println!("Too low!");
        } else if guess > secret {
            println!("Too high!");
        } else {
            println!("You got it in {} tries!", tries);
            break;
        }
    }
}