//hashmap with example
//adding and removing elements in hashmap
fn main() {
    // Create a new HashMap
    let mut orders = std::collections::HashMap::new();

    // Add some orders to the HashMap
    orders.insert(1, "Pizza");
    orders.insert(2, "Burger");
    orders.insert(3, "Pasta");

    // Print the initial HashMap
    println!("Initial orders: {:?}", orders);

    // Remove an order from the HashMap
    orders.remove(&2);

    // Print the HashMap after removing an order
    println!("Orders after removal: {:?}", orders);

    // Check if an order exists in the HashMap
    if orders.contains_key(&1) {
        println!("Order 1 exists in the HashMap.");
    } else {
        println!("Order 1 does not exist in the HashMap.");
    }
}