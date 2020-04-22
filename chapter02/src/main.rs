use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess a number between 1 and 10.");

    let secret_number = rand::thread_rng().gen_range(1, 11);

    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input a (new) number.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("The number is too small!"),
            Ordering::Greater => println!("The number is too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
