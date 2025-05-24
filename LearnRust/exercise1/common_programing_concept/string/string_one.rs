// solve a Different Problem: Given a string, count the number of vowels in the string with user input.
use std::io;
fn main() {
    // Input: Read a line of input from the user
    println!("Enter a string:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Process: Count the number of vowels in the string
    let vowels = "aeiouAEIOU";
    let count = input.matches(|c: char| "aeiouAEIOU".contains(c)).count(); // |c: char| c is a character, check if it is in the vowels string, and count the matches
    // Alternatively, you can use a more explicit loop:
    // let count = input.chars().filter(|c| vowels.contains(*c)).count(); // |c| c is a character, check if it is in the vowels string, and count the matches
    

    // Output: Print the result
    println!("The number of vowels in the string is: {}", count);
}