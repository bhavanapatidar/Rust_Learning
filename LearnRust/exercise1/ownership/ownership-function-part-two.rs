fn main(){
    let x:String = String::from("Hello"); // x is the owner of hello 
    process_string (x); // transfer of ownership
 //   println!("the value of x in main() is{}",x) if i will uncomment this line then it will throw an error 
}

fn process_string(item :String){ //new owner of hello is item 
    println!("the value of x in process_string is {}",item);
}