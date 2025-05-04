// we use mutable reference in that case where we dont want to transfer ownership and we want to borrow and we want to make changes in value or string
// so we need to mut in order to have a mutable reference.
fn main() {
    let mut s1:String = String::from("Hello");
    append_string(&mut s1);  //passing mutable reference
    println!("the new string is {}",s1);

}
fn append_string(s2:&mut String){
    s2.push_str("World");
}