fn main() {
    println!("Hello, world!");

    let y = {
        let s = 3;
        s+1
    };
    println!("the value of y is: {y}");
    another_function(5);
    print_labeled_measurement(5,'h');
    let q = five();
    println!("the value of q is : {q}");
    let mut x = plusone(8);
    println!("value after plus one {x}");
}
fn another_function( mut x: i32){
    println!("the value of x is:{x}");
}
fn print_labeled_measurement(value: i32, unit_label: char){
    println!("the measurement is : {value}{unit_label}");
}
fn five() -> i32{
    5               //returning the value so we can't add semicolon 
}
fn plusone(x:i32) -> i32{
    x+1
}
