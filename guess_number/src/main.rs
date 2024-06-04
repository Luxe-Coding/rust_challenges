use rand::Rng;
use std::cmp::Ordering;
use std::io;

const ATTEMPTS: u32 = 7;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("🕹️ Welcome to 'Guess the Number'! 🕹️");
    println!("A number between 1 and 100 has been chosen. Can you guess what it is?");
    println!("🤞 You have {ATTEMPTS} attempts. Good luck! 🍀");

    let mut guessed = false;

    for attempt in 1..=ATTEMPTS {
        println!("Attempt {attempt}:");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                guessed = true;
                println!("🎉 Congratulations! You guessed the number! 🎉");
                break;
            }
        }
    }

    if !guessed {
        println!("😢 Out of attempts! The number was {secret_number}.");
    }
}
