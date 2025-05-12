fn main(){
    let number = 3;
    match number{
        1 | 2 => println!("one"),
        2 | 3 => println!("two"),
        3 => println!("three"), // warning no value can reach this
        _ => println!("other number"), // _ is a catch all pattern
    }
}