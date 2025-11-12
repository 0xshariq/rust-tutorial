use std::io;

fn factorial(n: u128) -> u128 {
    if n == 0 || n == 1 {
        1
    } else {
        n * factorial(n - 1)
    }
}

fn main() {
    let mut input = String::new();
    println!("Enter a number for factorial calculation : ");
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let input: u128 = input.trim().parse::<u128>().expect("Please enter a valid number");
    let result = factorial(input);
    println!("Factorial of {} is {}", input, result);
}