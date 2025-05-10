fn main(){
    let s1: String = String::from("hello");
    let len: usize = calculate_length(&s1);
    println!("the lenfth of {} is {}", s1,len);
}
fn calculate_length(s:&String)-> usize{
    return s.len(); //Auto dereference  at the place of s.len() if we write (*s).len(); then it will work , we dont need to apply * becuse rust already do that its just a syntax
}