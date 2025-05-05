fn main(){
    let x = 5;
    println!("address={:p}",&x);
    let y = &x; //y is reference to the value of x , value of x is 5
    println!("address={:p}",y);
}