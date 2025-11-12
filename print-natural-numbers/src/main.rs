use std::io;

fn main() {
    let mut input = String::new();
    println!("Enter a positive integer:");
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let input = input.trim().parse::<u32>().expect("Please enter a valid positive integer");
    for number in 1..=input {
        println!("{}", number);
    }
}