fn main() {
    let mut x = 5;
    println!("The value of x = {x}");
    let x = 6;
    println!("the value of x = {x}");

    let x = x +1;
    {
        let x = x*2;
        println!("inner value if x = {x}");

    }
    println!("the value of x is = {x}");
}


