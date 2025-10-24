use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    let secret = rand::thread_rng().gen_range(1..=10);

    loop {
        println!("Enter your guess: ");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if guess < secret {
            println!("Too small!");
        } else if guess > secret {
            println!("Too big!");
        } else {
            println!("You win!");
            break;
        }
    }
}
