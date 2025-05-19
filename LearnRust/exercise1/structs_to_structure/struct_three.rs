// multiple structs in one file
struct user {
    name: String,
    age: u32,
}
struct company {
    name: String,
    location: String,
}

fn main() {
    let user1 = user {
        name: String::from("Ali"),
        age: 25,
    };
    let company1 = company {
        name: String::from("Turing"),
        location: String::from("Pakistan"),
    };
    println!("the name of user is {} and the age is {}", user1.name, user1.age);
    println!(
        "the name of company is {} and the location is {}",
        company1.name, company1.location
    );
}