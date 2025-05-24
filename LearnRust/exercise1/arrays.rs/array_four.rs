//  Problem: Given an array of integers, find the sum of all even numbers in the array with user inputand return index of all even numbers in the array with user input.
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
        .filter_map(|s| s.parse().ok())
        .collect();

    // Calculate the sum of even numbers and collect their indices
    let mut sum_of_evens = 0;
    let mut even_indices = Vec::new();

    for (index, &value) in numbers.iter().enumerate() {
        if value % 2 == 0 {
            sum_of_evens += value;
            even_indices.push(index);
        }
    }

    // Output: Print the result
    println!("The sum of even numbers is: {}", sum_of_evens);
    println!("Indices of even numbers are: {:?}", even_indices);
}