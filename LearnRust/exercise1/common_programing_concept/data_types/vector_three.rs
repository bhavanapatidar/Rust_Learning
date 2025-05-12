// fn main(){
//     let vrr: Vec<&str> = vec!["apple,banana","orange","grape"]; 
//     write_vrr(vrr); //ownership transfer
//   //  println!("vrr={:?}",vrr);  // this will give error because ownership is transferred to write_vrr function
    
// }

// fn write_vrr(vrr2:Vec<&str>){ //vrr2 is current owner of vrr
//     //vrr2.push("kiwi"); //error: cannot borrow `vrr2` as mutable, as it is not declared as mutable
//     println!("vrr2={:?}",vrr2);

// }

fn main(){
    let vrr: Vec<&str> = vec!["apple","banana","orange","grape"]; 
    write_vrr(&vrr); //but if we will pass the reference of vrr then we can use it again if we dont want to transfer ownership and borrowing we can use vrr.clone() method
    println!("vrr={:?}",vrr);  
   
    //println!("vrr={:?}",vrr);  // this will give error because ownership is transferred to write_vrr function    
}

fn write_vrr(vrr2:&Vec<&str>){ //we borroed here vrr2 as reference
    //vrr2.push("kiwi"); //error: cannot borrow `vrr2` as mutable, as it is not declared as mutable
    println!("vrr2={:?}",vrr2);

}



