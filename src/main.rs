use std::io::{self, Write};
mod lenstra;
use crate::lenstra::ecm;

fn main() {
    print!("Number to factor: ");
    // takes input from console
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let n: i128 = input.trim().parse().expect("Please type a valid number");

    // runs it through the factorization altorithm
    let result = ecm(n);
    println!("Factors: {:?}", result);
}
