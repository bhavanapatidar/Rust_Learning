// solve a problem using arrays

// Problem: Given an array of integers, find the sum of all even numbers in the array with user input.
use std::io;
fn main() {
    // Input: Read a line of input from the user
    println!("Enter a list of integers separated by spaces:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Process: Split the input into numbers, parse them, and filter for even numbers
    let numbers: Vec<i32> = input
        .trim()
        .split_whitespace()
        .filter_map(|s| s.parse().ok()) // |s| s is a string slice, parse it to i32, and filter out any parsing errors
        .collect();

    // Calculate the sum of even numbers
    let sum_of_evens: i32 = numbers.iter().filter(|&&x| x % 2 == 0).sum(); //|&&x| x is a reference to an i32, check if x is even and sum them up and &&x is used to dereference the reference to get the value

    // Output: Print the result
    println!("The sum of even numbers is: {}", sum_of_evens);
}