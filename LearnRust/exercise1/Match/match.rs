fn main(){
    let number = 3;
    match number{
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("other number"), // _ is a catch all pattern
    }
}