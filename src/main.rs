use std::io::{self, Write};
mod lenstra;
use crate::lenstra::ecm;
use num_bigint::{BigInt, ToBigInt};

fn main() {
    print!("Number to factor: ");
    // takes input from console
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let n: BigInt = input
        .trim()
        .parse::<BigInt>()
        .expect("Please type a valid number")
        .to_bigint()
        .unwrap();
    // runs it through the factorization algorithm
    let result = ecm(n);
    println!("Factors: {:?}", result);
}
