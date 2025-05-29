// Online Rust compiler to run Rust program online
// Print "Try programiz.pro" message
use std::collections::HashMap;
fn main() {

//create HashMap
let mut marks : HashMap<&str,i32> = HashMap::new();
//Insert values into HashMap
marks.insert("Rust",10);
marks.insert("Python",20);
marks.insert("C",30);
marks.insert("C++",40);
marks.insert("English",50);

println!("{:?}",marks);
  // find the length  
 println!("length of hashmap is {:?}",marks.len());

// Let's Match the value         
match marks.get("Maths"){
Some(mark) => println!("marks of this subject is {:?}", mark),
None => println!("not found this subject")
    
}

//Remove the Value 
marks.remove("Rust");
println!("{:?}",marks);

//loop through HashMap
for (sub,mar) in &marks{
    println!("for {} you got {} marks", sub, mar);
    
    //check the value
    println!("Did you study C++ = {}  ", marks.contains_key("C++"))
}
}