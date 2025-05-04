// fn main(){
//     let s1:String = String::from("Hello");
//     let r1 = &s1;
//     println!("r1={}",r1);
//     let r2 = &s1;
//     println!("r2={}",r2);
// }

//here we are only reading the data so it is not having problem , but when we make it mutable it will be throwing an error

fn main(){
    let mut s1:String = String::from("Hello");
    let r1 = &mut s1; 
    r1.push_str("World");
    println!("r1={}",r1);
    let r2 = &mut s1;
    r2.push_str("Code");
    println!("r2={}",r2);
}

// here we are using mutable reference so it will also not throw an error

// fn main(){
//     let mut s1:String = String::from("Hello");
//     let r1 = &mut s1; 
//     r1.push_str("World");
//   //  println!("r1={}",r1);
//     let r2 = &mut s1;
//     r2.push_str("Code");
//  //   println!("r2={}",r2);
//  println!("r1={} r2={}",r1,r2);
// }

// the above code will throw error because of data race