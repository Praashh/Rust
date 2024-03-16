use std::io;
use colored::*;

fn main() {
    println!("\n");
    println!("{}", "----------------Welcome to my RUSTY Calculator!----------------".blue());
    println!("");
    println!("{}", "----------------Enter Your operation Name!------------------------".magenta());
    println!("");
    println!("{}", "----------------ADD, SUB, MUL, DIV--------------------------------".yellow());
    println!("");
    let mut operation : String = String::new();
    io::stdin()
    .read_line(&mut operation)
    .expect("Failed to read line");

    println!("{}", "----------------Enter First Number--------------------------------".blue());
    println!("");
    let mut first = String::new();
    io::stdin()
        .read_line(&mut first)
        .expect("Failed to read line");

    let first: i32 = first.trim().parse().expect("Please type a number!");

    let mut second = String::new();
    io::stdin()
        .read_line(&mut second)
        .expect("Failed to read line");

    let second: i32 = second.trim().parse().expect("Please type a number!");

    let result = match operation.trim().to_uppercase().as_str() {
        "ADD" => first + second,
        "SUB" => first - second,
        "MUL" => first * second,
        "DIV" => first / second,
        _ => panic!("Invalid operation"),
    };

    match operation.trim().to_uppercase().as_str() {
        "ADD" => println!("{}", format!("Result: {}", result).green()),
        "SUB" => println!("{}", format!("Result: {}", result).blue()),
        "MUL" => println!("{}", format!("Result: {}", result).yellow()),
        "DIV" => println!("{}", format!("Result: {}", result).cyan()),
        _ => println!("{}", "Invalid operation".red()),
    }
}
