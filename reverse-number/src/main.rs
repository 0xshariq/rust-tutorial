use std::io;

fn main() {
    let mut input = String::new();
    println!("Enter a positive integer:");
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Convert string input to an integer
    let mut number: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid positive integer.");
            return;
        }
    };

    let mut reverse_number: u32 = 0;

    // Loop to reverse the digits
    while number > 0 {
        let digit = number % 10; // get last digit
        reverse_number = reverse_number * 10 + digit; // build reverse number
        number /= 10; // remove last digit
    }

    println!("Reversed number: {}", reverse_number);
}
