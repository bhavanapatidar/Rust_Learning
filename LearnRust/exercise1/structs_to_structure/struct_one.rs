// struct like as class in other languages

struct user{
    name: String,
    company: String,
    age: u32,
}

fn main(){
    let user1 = user{  // creating instance of struct and assigning values , we can also assign in any order
        name: String::from("Ali"),
        company: String::from("Turing"),
        age: 25,
    };
    let user2 = user{  // creating instance of struct and assigning values , we can also assign in any order
        name: String::from("Ali"),
        company: String::from("Turing"),
        age: 25,
    };
    println!("the name of user is {} and the company is {} and the age is {}",user1.name,user1.company,user1.age);
    println!("the name of user is {} and the company is {} and the age is {}",user2.name,user2.company,user2.age);
}