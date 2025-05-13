fn main(){
    fn is_even(num:i8) -> bool {
        if num % 2 == 0 {
            return true;
        } else {
            return false;
        }
    }

   let number = 3;
    match number{
        x if is_even(x) => println!("even number"),
        x if !is_even(x) => println!("odd number"),
        _ => println!("other number"), // _ is a catch all pattern
    }
}