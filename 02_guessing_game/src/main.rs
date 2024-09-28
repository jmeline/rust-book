use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=1000);

    loop {
        println!("Guess the number!");
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number!");
                continue
            },
        };

        println!(
            "You guessed: {}, the secret_number is {secret_number}",
            guess
        );

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!! {guess} is smaller than {secret_number}"),
            Ordering::Greater => println!("Too big!! {guess} is bigger than {secret_number}"),
            Ordering::Equal => {
                println!("You win!! {guess} is equal to {secret_number}");
                break;
            },
        }
    }
}
