fn main() {
    let tup:(i32, f64, u8) = (500, 6.4, 1);
    let (x,y,z) = tup;
    println!("the value of y is: {y}");

    let a = [3;5];
    println!("Array: {:?}, Value at index 2: {}", a,a[2]);
}
