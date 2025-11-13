use std::io;

fn is_palindrome(s: &str) -> bool {
    let cleaned: String = s.chars().filter(|c| c.is_alphanumeric()).collect::<String>().to_lowercase();
    cleaned == cleaned.chars().rev().collect::<String>()
}

fn main() {
    println!("Enter a string to check if it's a palindrome:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input = input.trim();

    if is_palindrome(input) {
        println!("\"{}\" is a palindrome.", input);
    } else {
        println!("\"{}\" is not a palindrome.", input);
    }
}