use std::collections::HashMap; // importing HashMap from the standard library , hashmap is a collection of key-value pairs, it uses a hash function to compute the index of the key-value pair in the collection, it is used to store and retrieve data efficiently, it is a part of the standard library
// we use hashmap to store the orders, where the key is the order id and the value is the order object 

use std::io;
// importing io from the standard library, io is used to read input from the user, it is a part of the standard library
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

    pub struct OrderManager {  // OrderManager is a struct that contains a hashmap of orders because we need to store the orders in a hashmap, where the key is the order id and the value is the order object
        pub orders: HashMap<u32, Order>,
    }

    impl OrderManager {
        pub fn new() -> Self { // new is a function that creates a new instance of the OrderManager struct, it returns an instance of the OrderManager struct
            Self {
                orders: HashMap::new(),
            }
        }

        pub fn create_order(&mut self, order_id: u32, dish_name: &str, customer_name: &str, amount: f64) {
            let order = Order { // Order is a struct that contains the order details, it has fields for order id, dish name, customer name, table number, address and amount
                order_id, // order_id is a field of the Order struct, it is the id of the order
                dish_name: dish_name.to_string(),
                customer_name: customer_name.to_string(),
                table_number: None,
                address: None, // none means that the address is not provided yet
                amount,
            };
            self.orders.insert(order_id, order); // insert is a function that inserts a new order into the hashmap, it takes the order id and the order object as parameters
            println!("Order #{} for '{}' created for customer '{}'.", order_id, dish_name, customer_name); // here we are printing the order details, we are using the order id, dish name and customer name to print the order details
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
    let mut input = String::new();

    loop {
        println!("\n--- Order Management System ---");
        println!("1. Create Order");
        println!("2. Assign Table");
        println!("3. Cook Order");
        println!("4. Process Payment");
        println!("5. Schedule Delivery");
        println!("6. Cancel Order");
        println!("7. Exit");
        println!("Enter your choice: ");

        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let choice: u32 = input.trim().parse().unwrap_or(0);

        match choice {
            1 => {
                println!("Enter Order ID: ");
                input.clear();
                io::stdin().read_line(&mut input).unwrap();
                let order_id: u32 = input.trim().parse().unwrap();

                println!("Enter Dish Name: ");
                input.clear();
                io::stdin().read_line(&mut input).unwrap();
                let dish_name = input.trim().to_string(); // Store the trimmed value in a new variable

                println!("Enter Customer Name: ");
                input.clear();
                io::stdin().read_line(&mut input).unwrap();
                let customer_name = input.trim().to_string(); // Store the trimmed value in a new variable

                println!("Enter Amount: ");
                input.clear();
                io::stdin().read_line(&mut input).unwrap();
                let amount: f64 = input.trim().parse().unwrap();

                order_manager.create_order(order_id, &dish_name, &customer_name, amount);
            }
            2 => {
                println!("Enter Order ID: ");
                input.clear();
                io::stdin().read_line(&mut input).unwrap();
                let order_id: u32 = input.trim().parse().unwrap();

                println!("Enter Table Number: ");
                input.clear();
                io::stdin().read_line(&mut input).unwrap();
                let table_number: u32 = input.trim().parse().unwrap();

                order_manager.assign_table(order_id, table_number);
            }
            3 => {
                println!("Enter Order ID: ");
                input.clear();
                io::stdin().read_line(&mut input).unwrap();
                let order_id: u32 = input.trim().parse().unwrap();

                order_manager.cook_order(order_id);
            }
            4 => {
                println!("Enter Order ID: ");
                input.clear();
                io::stdin().read_line(&mut input).unwrap();
                let order_id: u32 = input.trim().parse().unwrap();

                order_manager.process_payment(order_id);
            }
            5 => {
                println!("Enter Order ID: ");
                input.clear();
                io::stdin().read_line(&mut input).unwrap();
                let order_id: u32 = input.trim().parse().unwrap();

                println!("Enter Delivery Address: ");
                input.clear();
                io::stdin().read_line(&mut input).unwrap();
                let address = input.trim().to_string(); // Store the trimmed value in a new variable

                order_manager.schedule_delivery(order_id, &address);
            }
            6 => {
                println!("Enter Order ID: ");
                input.clear();
                io::stdin().read_line(&mut input).unwrap();
                let order_id: u32 = input.trim().parse().unwrap();

                order_manager.cancel_order(order_id);
            }
            7 => {
                println!("Exiting the system. Goodbye!");
                break;
            }
            _ => println!("Invalid choice. Please try again."),
        }
    }
}

