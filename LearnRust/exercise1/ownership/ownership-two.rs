//ownership is Rust's most unique feature, and it enables rust to make memory safety gurantees without needing a garbage collector
// OWNERSHIP RULES :-
// Rule1 = each value in rust is a variable that's called its owner
// Rule2 = There can be only one owner at a time 
// Rule3 = When the owner goes out of scope  , the value will be dropped.


fn main(){
    let str1 = String::from("hello"); //str1 is owner of Hello , str1 is pointer of hello and str2 is also pointer of hello , rust dont agree this 
    let str2 = str1; //Transfer of ownership. str2 is the new owner of hello that is called move operation
    
       //println!("str1={}",str1); // it will throw an error because heap mai memormy mai koi copy    trait nhi hota hai rust mai, it involves the pointers 
    
    println!("str2={}",str2);
}


// In rust , when ownership of a value is transferred to another variable the original variable becomes invalidated.
