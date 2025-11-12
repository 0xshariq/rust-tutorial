use std::io;

fn main() {
    let mut operand1 = String::new();
    let mut operand2 = String::new();
    let mut operator = String::new();

    println!("Enter first number: ");
    io::stdin().read_line(&mut operand1).expect("Failed to read line");

    println!("Enter second number: ");
    io::stdin().read_line(&mut operand2).expect("Failed to read line");

    println!("Enter operator (+, -, *, /): ");
    io::stdin().read_line(&mut operator).expect("Failed to read line");

    let operand1 = operand1.trim().parse::<f64>().expect("Please enter a valid number");
    let operand2 = operand2.trim().parse::<f64>().expect("Please enter a valid number");

    match operator.trim() {
        "+" => {
            let result = operand1 + operand2;
            println!("Result: {}", result);
        }
        "-" => {
            let result = operand1 - operand2;
            println!("Result: {}", result);
        }
        "*" => {
            let result = operand1 * operand2;
            println!("Result: {}", result);
        }
        "/" => {
            let divisor = operand2;
            if divisor == 0.0 {
                println!("Error: Division by zero is not allowed.");
            } else {
                let result = operand1 / divisor;
                println!("Result: {}", result);
            }
        }
        _ => {
            println!("Invalid operator. Please use +, -, *, or /.");
        }
    }
}
