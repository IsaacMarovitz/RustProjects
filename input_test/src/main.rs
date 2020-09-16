extern crate chrono;
use std::io;
use chrono::prelude::*;

fn main() {
    let name = input("What's your name? ");
    let age = input("What's your age? ").trim().parse::<i32>().expect("Invalid age.");
    let current_year = Utc::now().year();
    let hundred_year = 100 - age + current_year;

    println!("Hey, {}! You'll turn 100 in the year: {}.", name, hundred_year);
}

fn input(user_mesage: &str) -> String {
    use std::io::Write;

    print!("{}", user_mesage);
    io::stdout().flush().expect("Aghhhh");

    let mut user_input = String::new();

    io::stdin().read_line(&mut user_input).expect("Aghhhhjhhurgrh");
    user_input
}