use std::io;

// Function to generate Fibonacci series up to n terms
fn fibonacci(n: u32) {
    let mut a = 0;
    let mut b = 1;

    for _ in 0..n {
        print!("{} ", a);
        let next = a + b;
        a = b;
        b = next;
    }
    println!(); // newline after printing all terms
}

fn main() {
    let mut input = String::new();
    println!("Enter the number of terms for the Fibonacci series:");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid non-negative integer.");
            return;
        }
    };

    println!("Fibonacci series up to {} terms:", input);
    fibonacci(input); // call the function
}
