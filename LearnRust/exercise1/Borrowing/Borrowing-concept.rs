// the function borrows 's1' without taking ownership.it can read the content of 's1' but it can not change or modify it 
// when you want to pass data to a function or share it between parts of your program, you can choose to lend or (borrow ) ownership of the data rather than transferring ownership outright.
//Borrowing allows you to temporarily use a reference to the data without taking ownership of it .


fn  main(){
    let s1:String = String::from("Hello");
    let len:usize = calculate_length(&s1); //borrow operation
    println!("the length of {} is {}", s1,len);
}
fn calculate_length(s2:&String)->usize{
    return s2.len();
}

//the &s1 syntax lets us create a reference that refers to the value of s1 but does not own it.
//Because 's' does not own 's1' when 's' goes out of scope nothing happens

// fn main(){
//     let mut s1:String = String::from("Hello");
//     append_string(&s1);
//     println!("the new string is {}", s1);
// }
// fn append_string(s3:&String) {
//     s3.push_str("world");
// }

// above commented code will throw error because we can not make change when we are boorwing \
// for solution of this we will use mutable reference . 