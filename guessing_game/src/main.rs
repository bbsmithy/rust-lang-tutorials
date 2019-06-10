use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("The guessing game if you guess in correctly.....you die");
    println!("Guess a number:");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please guess a number yih madt thing");
                continue;
            }
        };
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("You guessed low"),
            Ordering::Greater => println!("You guessed high"),
            Ordering::Equal => {
                println!("You guessed correctly");
                break;
            }
        }
    }
    

}
