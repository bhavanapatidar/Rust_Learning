// fn main(){
//     let s1:String = String::from("hello"); //s1 owner
//     let len:usize = calculate_length(s1); // ownership transfer
//     println!("the length of {} is {}",s1, len);
// }
// fn calculate_length(s:String)->usize{ //s will be new owner
//     return s.len(); //return 5
// }

// the above code will throw error , because s1 ownership is lost and we are trying to access the value in line 4
// solution :- either use tuple or retake the ownership 
// lets do using both 

fn main(){
    let s1:String = String::from("hello");
    let (s2,len) = calculate_length(s1);// new owner s2
    println!("The length {} is {}",s2,len);
}

fn calculate_length(s:String)->(String,usize){
    let length:usize = s.len();
    return (s,length); // ownership transfer and returning tuple here 
}

// this is woring but always taking tuple in big program is problematic