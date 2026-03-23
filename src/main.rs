mod lenstra;
use std::io::{self, Write};

use crate::lenstra::ecm;

fn main() {
    print!("Number to factor: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let n: i128 = input.trim().parse().expect("Please type a valid number");

    let result = ecm(n);
    println!("Factors: {:?}", result);
}
