// there are two types of memory allocation happens in progrms , 1. Stack Memory and 2. Heap Memory 
// stack stores static data and heap  stores dynamic data
// to make memory allocation more strong we need ownership 
// String and str = String is dynamic in rust and str is static in rust

fn main(){
let a = 4 ;
let b = a;

println!("a={}",a);
println!("b={}",b);
}

// the above code is correct , it will not throw an error , here a is integer and have static value so it will store in Stack so we 
//dont need to give variable type here 