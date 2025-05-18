// Traversal: Input: [1, 2, 3]. Output: 1, 2, 3 (printed).
// Insertion: Start with [1, 2], insert 3 at end. Output: [1, 2, 3].
// Deletion: From [1, 2, 3], remove 2. Output: [1, 3].

fn main(){
    let number : [i32;4] = [1,2,3,4];
    println!("Traversal of array is :{:?}",number);
    let mut dynamic_array= vec![1,2];
    dynamic_array.push(3);
    println!("the dynamic array is{:?}",dynamic_array);
    dynamic_array.remove(1);
    println!("the dynamic array after removing 1st index is{:?}",dynamic_array);
}