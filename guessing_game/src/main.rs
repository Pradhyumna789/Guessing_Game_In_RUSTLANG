use std::io;
use rand::Rng;
use std::cmp::Ordering;
use colored::Colorize;
fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is {}", secret_number);

    loop {

        let mut guess = String::new();

        println!("Please input your guess");

        io::stdin().read_line(&mut guess).expect("Failed to read the line");

        let guess: u32 = guess.trim().parse().expect("Please enter a number!");

        println!("You guessed {}!", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too small!".red()),
            Ordering::Greater => println!("{}", "Too big!".red()),
            Ordering::Equal => {
                println!("{}", "You got that right!".green());
                break;
            }
        
        }
    }
}