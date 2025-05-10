//we will pass reference of array

fn main(){
    let mut arr: [&str;3] = ["papaya", "mango", "Melon"];
    write_arr(&mut arr);
    println!("arr = {:?}",arr);

}
fn write_arr(arr2:&mut [&str;3] ){ // arr2 will start to point arr , it will reference the value of arr
    arr2[0] = "orange";
    println!("arr2 = {:?}", arr2);
}