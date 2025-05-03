fn main(){
    let s1:String = String::from("hello");
    let len = calculate_length(s1.clone());
    println!("length of {} is {}", s1,len);
}

fn calculate_length(s:String)->usize{
    return s.len();
}