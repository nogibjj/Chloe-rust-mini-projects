use std::cmp::Ordering; // another enum and has the variants Less, Greater, and Equal
use rand::Rng; // defines methods that random number generators implement
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    let mut count = 0;
    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
        // increment the count
        count += 1;

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        // use a match expression to decide what to do next 
        // based on which variant of Ordering was returned from the call 
        // to cmp with the values in guess and secret_number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                println!("You guessed {count} times");
                break;
            }
        }
    }
}
