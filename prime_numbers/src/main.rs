use std::io;
// Function to generate a list of prime numbers up to n
fn primes_up_to(n: u32) -> Vec<u32> {
    let mut primes = Vec::new();
    for num in 2..=n {
        if is_prime(num) {
            primes.push(num);
        }
    }
    primes
}

// Helper function to check if a number is prime
fn is_prime(num: u32) -> bool {
    if num < 2 {
        return false;
    }
    for i in 2..=((num as f64).sqrt() as u32) {
        if num % i == 0 {
            return false;
        }
    }
    true
}

fn main() {
    println!("Enter a number to find all prime numbers up to that number:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let n: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid positive integer.");
            return;
        }
    };

    let primes = primes_up_to(n);
    println!("Prime numbers up to {}: {:?}", n, primes);
}
