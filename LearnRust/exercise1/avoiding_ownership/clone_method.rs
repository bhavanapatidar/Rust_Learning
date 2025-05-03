// before understanding this section once go through ownership part in that retaking ownership i have discussed 
//we can avoid ownership using clone method and retaking ownership using tuple
// clone method is use for the deep copy of the heap data. This is an expensive method .


fn main(){
let s1 = String::from("hello");
let s2 = s1.clone();
println!("s1 = {}, s2 = {}", s1,s2);

}