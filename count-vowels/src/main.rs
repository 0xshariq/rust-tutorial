use std::io;

fn count_vowels(s: &str) -> usize {
    let vowels = "aeiouAEIOU";
    s.chars().filter(|c| vowels.contains(*c)).count()
}

fn main() {
    println!("Enter a string:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let vowel_count = count_vowels(&input);
    println!("Number of vowels: {}", vowel_count);
}
