// Array with hashmap advanced example
// problem: Create a static array of 5 numbers, then create a HashMap that maps each number to its square and returns the HashMap and access the values using the key
use std::collections::HashMap;
use std::io;
fn main() {
    // Create a static array of 5 numbers
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    
    // Create a HashMap to store the number and its square
    let mut squares: HashMap<i32, i32> = HashMap::new();
    
    // Populate the HashMap with numbers and their squares
    for &num in &numbers {
        squares.insert(num, num * num);
    }
    
    // Output the HashMap
    println!("Number to Square Map: {:?}", squares);
    
    // User input to access the square of a number
    println!("Enter a number to get its square:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    // Parse the input and access the square from the HashMap
    if let Ok(key) = input.trim().parse::<i32>() { // Attempt to parse the input as an i32 , // if successful, key will hold the parsed value , Ok is used to check if the parsing was successful
        match squares.get(&key) {
            Some(&value) => println!("The square of {} is {}", key, value),
            None => println!("No square found for {}", key),
        }
    } else {
        println!("Invalid input. Please enter a valid integer.");
    }
}