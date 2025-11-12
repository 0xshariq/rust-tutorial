use std::io;

fn main() {
    let mut input = String::new();
    println!("Enter a positive integer:");
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let input = input.trim().parse::<u32>().expect("Please enter a valid positive integer");
    let mut sum = 0;
    for number in 1..=input {
        sum += number;
    }
    println!("Sum of natural numbers up to {} is: {}", input, sum);
}