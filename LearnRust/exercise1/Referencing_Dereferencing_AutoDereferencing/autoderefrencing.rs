// fn main(){
//     let x = 5;
//     let y = &x; //y is reference to the value of x 
//     println!("address={}",y); //auto derefrencing
// }


fn main(){
    let mut x = 5;
    x = x + 1;
    let y = &mut x ; // refrencing
    *y = *y+1; // derefrencing
    println!("x={}",y);

}
//reference VS Pointer
// In rust , when you create a reference to a variable, such as let y_ref = &x;,\
// the reference y_ref is essentially a wrapper around a memory address. However, unlike a raw pointer,
// which directly holds the memory address, a reference holds metadata about the reference, such as its lifetime and mutability.



// Reference in rust behave similarly to pointers in other languages, they come with additional metadata
// and safety checks provided by the rust compiler. This allows rust to offer the performance benifits of 
// direct memory access while ensuring memory safety and preventing common programming errors.