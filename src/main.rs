use std::io;
use rand::Rng;

pub fn guess() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut number = String::new();

        io::stdin()
            .read_line(&mut number)
            .expect("Failed to read line");

        let number: u32 = match number.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        };

        if number < secret_number {
            println!("Too small!");
        } else if number > secret_number {
            println!("Too big!");
        } else {
            println!("You guessed it! The secret number was {}", secret_number);
            break;
        }
    }
}

fn main() {
    guess();
}
