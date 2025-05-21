// learn modules with a simple example of college library management system
mod library {
    pub mod books {
        pub fn add_book() {
            println!("Book added to the library.");
        }
    }

    pub mod members {
        pub fn add_member() {
            println!("Member added to the library.");
        }
    }

    pub mod transactions {
        pub fn issue_book() {
            println!("Book issued to member.");
        }
    }
}
fn main() {
    // Adding a book to the library
    library::books::add_book();

    // Adding a member to the library
    library::members::add_member();

    // Issuing a book to a member
    library::transactions::issue_book();
}