use std::io;

fn cube(n: u128) -> u128 {
    n * n * n
}

fn square(n: u128) -> u128 {
    n * n
}

fn main() {
    let mut input = String::new();
    println!("Enter a number to find its square and cube:");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input: u128 = input.trim().parse().expect("Please enter a valid number");

    let squared = square(input);
    let cubed = cube(input);

    println!("The square of {} is {}", input, squared);
    println!("The cube of {} is {}", input, cubed);
}
