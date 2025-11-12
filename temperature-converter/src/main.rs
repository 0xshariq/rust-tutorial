use std::io;

fn celcius_to_fahrenheit(celsius: f64) -> f64 {
    (celsius * 9.0 / 5.0) + 32.0
}

fn fahrenheit_to_celcius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

fn main() {
    let mut input = String::new();
    println!("Enter temperature (e.g., 36.6C or 97.9F):");
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let input = input.trim();
    if input.ends_with('C') || input.ends_with('c') {
        let value: f64 = match input[..input.len()-1].trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid temperature.");
                return; 
            }
        };
        let fahrenheit = celcius_to_fahrenheit(value);
        println!("{}C is {}F", value, fahrenheit);
    } else if input.ends_with('F') || input.ends_with('f') {
        let value: f64 = match input[..input.len()-1].trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid temperature.");
                return;
            }
        };
        let celsius = fahrenheit_to_celcius(value);
        println!("{}F is {}C", value, celsius);
    } else {
        println!("Please enter a temperature ending with 'C' or 'F'.");
    }
}