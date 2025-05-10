fn main(){
    let arr: [&str;3]=["Banana","Apple","orange"];
    write_arr(arr);
    println!("arr = {:?}",arr);
}
fn write_arr(mut arr1: [&str;3]){ // made arr1 new copy of arr in memory , it will be an expensive method so we can do using passing an array be reference 
    arr1[0] = "Papaya";
    println!("arr1 = {:?}",arr1);
}