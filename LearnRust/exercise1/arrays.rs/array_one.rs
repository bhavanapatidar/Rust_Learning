// Static Array: Make an array of 3 numbers: [1, 2, 3]. You can’t add a 4th number.
// Dynamic Array: Start with an empty list, add 1, 2, 3, then add 4. Output: [1, 2, 3, 4].

fn main(){
    let number :[i32;3]= [1,2,3];
    println!("the static array is {:?}",number);
    let mut dynamic_array= Vec::new();
    dynamic_array.push(1);
    dynamic_array.push(2);
    println!("the dynamic array is{:?}",dynamic_array);
}