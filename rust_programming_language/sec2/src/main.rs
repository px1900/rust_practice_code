use std::io;
use rand;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is {secret_number}");

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
        io::stdin().read_line(& mut guess).expect("Failed to read line");

        // let guess: u32 = guess.trim().parse().expect("Can't be parsed to an integer");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Your input can't be parsed as an integer");
                continue;
            },
        };


        println!("Your guessed value = {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Your input is smaller"),
            Ordering::Greater => println!("Your input is greater"),
            Ordering::Equal => {
                println!("You are correct");
                break;
            },
        }

    }

}
