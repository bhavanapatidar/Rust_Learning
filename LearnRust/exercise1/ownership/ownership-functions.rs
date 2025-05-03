fn main(){
    let x = 5;
    process_integer(x);
    println!("the value of x in main() is {}",x);
}
fn process_integer(x:u8){
    println!("the value of x in process_integer is {}",x);
}