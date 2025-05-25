//Problem: Find the Longest Palindromic Substring

use std::io;

fn is_palindrome(s: &str) -> bool {
    let chars: Vec<char> = s.chars().collect(); // Convert the string to a vector of characters for easy indexing
    let len = chars.len();
    for i in 0..len / 2 {
        if chars[i] != chars[len - i - 1] { // Compare characters from the start and end moving towards the center
            return false;
        }
    }
    true
}

fn longest_palindromic_substring(s: &str) -> String { // Function to find the longest palindromic substring
    if s.is_empty() {
        return String::new(); // Return an empty string if the input is empty
    }
    let mut max_length = 0;
    let mut longest_palindrome = String::new();

    for i in 0..s.len() {// Iterate over each character in the string
        for j in i + 1..=s.len() {    // Iterate over all possible substrings starting from index i
            let substring = &s[i..j]; // Get the substring from index i to j
            // Check if the substring is a palindrome and if its length is greater than the current max_length
            if is_palindrome(substring) && substring.len() > max_length {
                max_length = substring.len();
                longest_palindrome = substring.to_string();
            }
        }
    }

    longest_palindrome
}

fn main() {
    println!("Enter a string:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input = input.trim(); // Remove any trailing newline or spaces

    let result = longest_palindromic_substring(input);
    println!("The longest palindromic substring is: {}", result);
}