use std::io;
use rand::Rng;
use colored::*;

fn main() {
    println!("\n");
    println!("{}", "----------------Welcome to Guessing Game!----------------".blue());
    println!("");
    println!("{}", "----------------Enter Your Guess!------------------------".magenta());
    
        let mut rng = rand::thread_rng();
        let n: u32 = rng.gen_range(0..100);

    loop {


    let mut guess = String::new();
    io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line");

    //  converting guess into u32 via removing whitespaces
    let guess: u32 = match guess.trim().parse(){
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a number!");
            return; 
        }
    };

    // checking the guess over here.
    if guess == n {
        println!("{}", "You win!".green());
        break;
    } else if guess > n {
        println!("{}", "too large".red());
    } else {
        println!("{}", "too small".yellow());
    }
}

} 
