use std::io;

fn main() {
    let mut input: String = String::new();
    println!("Enter a number : ");
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let input: u32 = input.trim().parse::<u32>().expect("Please enter a valid number");

    println!("\nMultiplication Table of {}", input);
    println!("--------------------------");

    // Print table up to 10
    for i in 1..=10 {
        println!("{} x {} = {}", input, i, input * i);
    }
}
