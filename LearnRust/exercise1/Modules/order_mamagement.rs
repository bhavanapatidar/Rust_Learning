use std::collections::HashMap; // importing HashMap from the standard library , hashmap is a collection of key-value pairs, it uses a hash function to compute the index of the key-value pair in the collection, it is used to store and retrieve data efficiently, it is a part of the standard library

mod order_management {
    use std::collections::HashMap;

    pub struct Order {
        pub order_id: u32,
        pub dish_name: String,
        pub customer_name: String,
        pub table_number: Option<u32>,
        pub address: Option<String>,
        pub amount: f64,
    }

    pub struct OrderManager {
        pub orders: HashMap<u32, Order>,
    }

    impl OrderManager {
        pub fn new() -> Self {
            Self {
                orders: HashMap::new(),
            }
        }

        pub fn create_order(&mut self, order_id: u32, dish_name: &str, customer_name: &str, amount: f64) {
            let order = Order {
                order_id,
                dish_name: dish_name.to_string(),
                customer_name: customer_name.to_string(),
                table_number: None,
                address: None,
                amount,
            };
            self.orders.insert(order_id, order);
            println!("Order #{} for '{}' created for customer '{}'.", order_id, dish_name, customer_name);
        }

        pub fn assign_table(&mut self, order_id: u32, table_number: u32) {
            if let Some(order) = self.orders.get_mut(&order_id) {
                order.table_number = Some(table_number);
                println!("Customer '{}' assigned to Table #{}.", order.customer_name, table_number);
            } else {
                println!("Order #{} not found.", order_id);
            }
        }

        pub fn cook_order(&self, order_id: u32) {
            if let Some(order) = self.orders.get(&order_id) {
                println!("Order #{}: '{}' is being cooked.", order_id, order.dish_name);
            } else {
                println!("Order #{} not found.", order_id);
            }
        }

        pub fn process_payment(&self, order_id: u32) {
            if let Some(order) = self.orders.get(&order_id) {
                println!("Payment of ${:.2} processed for Order #{}.", order.amount, order_id);
            } else {
                println!("Order #{} not found.", order_id);
            }
        }

        pub fn schedule_delivery(&mut self, order_id: u32, address: &str) {
            if let Some(order) = self.orders.get_mut(&order_id) {
                order.address = Some(address.to_string());
                println!("Delivery for Order #{} scheduled to '{}'.", order_id, address);
            } else {
                println!("Order #{} not found.", order_id);
            }
        }

        pub fn cancel_order(&mut self, order_id: u32) {
            if let Some(order) = self.orders.remove(&order_id) {
                println!("Order #{} for '{}' has been canceled.", order_id, order.dish_name);
            } else {
                println!("Order #{} not found.", order_id);
            }
        }
    }
}

fn main() {
    let mut order_manager = order_management::OrderManager::new();

    // Creating orders
    order_manager.create_order(101, "Pasta Alfredo", "Alice", 15.99);
    order_manager.create_order(102, "Margherita Pizza", "Bob", 12.50);

    // Assigning tables
    order_manager.assign_table(101, 5);
    order_manager.assign_table(102, 6);

    // Cooking orders
    order_manager.cook_order(101);
    order_manager.cook_order(102);

    // Processing payments
    order_manager.process_payment(101);
    order_manager.process_payment(102);

    // Scheduling deliveries
    order_manager.schedule_delivery(101, "123 Main Street, Springfield");
    order_manager.schedule_delivery(102, "456 Elm Street, Springfield");

    // Canceling an order
    println!("\n--- Canceling Order #101 ---");
    order_manager.cancel_order(101);
}


