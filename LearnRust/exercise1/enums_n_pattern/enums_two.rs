
#[derive(Debug)] // to print the struct
enum GenderCat{
    male,female,transgender
}

#[derive(Debug)] // to print the struct
struct Person{
    name:String,
    email:String,
    age:u32,
    genderCat:GenderCat,   // enum
}

fn main(){
    let person1 = Person{
        name:String::from("Ali"),
        email:String::from("bhavana@gmail.com"),
        age:25,
        genderCat:GenderCat::male
    };

    println!("{:?}",person1);
    }